# hopbox-core — bootstrap

The shared Rust library that backs the hopbox ecosystem. Pure library code that both `hopbox-desktop` (the local TUI/GUI) and `hopbox-server` (the relay/host daemon) link against so the two never drift on protocol or provider semantics.

This crate owns:

- **AI backends** — provider trait + concrete implementations for Anthropic, OpenAI, Cerebras, and Ollama
- **PTY abstraction** — hopbox-specific wrapper around `portable-pty` for cross-platform pseudo-terminal management
- **Session protocol** — wire types for attach/detach/reconnect, input/output framing, and AI query/response messages
- **Config** — serde-friendly structs that deserialize the on-disk TOML for desktop and server

## Cargo layout

This is **not** a workspace. It is a single crate at the repo root — the repo *is* the crate. Workspace orchestration lives upstream in `hopbox-desktop` and `hopbox-server`, both of which depend on this crate by path (local dev) or git (standalone consumers).

## Module structure

```
src/
├── lib.rs          # public API surface — re-exports + module declarations
├── error.rs        # unified Error type (thiserror)
├── config.rs       # HopboxConfig, AiConfig, ServerConfig — serde-derived
├── ai/
│   └── mod.rs      # AiProvider trait, Message, Role, AiContext, AiResponse
├── protocol/
│   └── mod.rs      # ClientMessage, ServerMessage — tagged enums
└── pty/
    └── mod.rs      # PtyHandle, spawn_shell — thin wrapper over portable-pty
```

Each AI provider lives in its own file under `src/ai/` (e.g. `anthropic.rs`, `openai.rs`) and is gated behind a feature flag once we wire those in. The trait + shared types stay in `mod.rs`.

## Key design rules

1. **No async runtime binding.** We use `async-trait` to express async methods on the `AiProvider` trait but we do **not** depend on `tokio` at compile time for the library proper. `tokio` is a `dev-dependency` only (so tests can `#[tokio::test]`). Consumers pick their runtime — `hopbox-desktop` uses `tokio`, but a future embedded host could use `smol` or `async-std` without forking us.

2. **Pure library — no binary.** There is no `main.rs`, no `[[bin]]` in `Cargo.toml`, no CLI entrypoint. If you find yourself wanting one, you want a separate crate.

3. **Clean module boundaries.** Mobile FFI via UniFFI is a planned downstream consumer. The public API should not assume desktop/server context — types are POD where possible, traits are object-safe where it doesn't cost ergonomics, and the surface stays small. Adding FFI later should be additive (new module, new feature flag), never a restructure.

4. **Errors are opaque to consumers.** The `Error` enum exposes variant names but the inner `String` for provider-specific failures is informational only. We do not promise stable error messages across versions.

## The AiProvider trait

```rust
#[async_trait]
pub trait AiProvider: Send + Sync {
    async fn complete(&self, ctx: &AiContext) -> Result<AiResponse>;
    fn name(&self) -> &'static str;
}
```

`Send + Sync` is required so providers can be stored behind `Arc<dyn AiProvider>` and shared across worker tasks. `complete()` is the only behavioral method — provider construction (API keys, model selection, base URL) happens in each impl's `new()` constructor and is not part of the trait.

### AiContext

```rust
pub struct AiContext {
    pub messages: Vec<Message>,         // full conversation so far
    pub terminal_buffer: Option<String>, // current visible PTY content, if any
    pub system_prompt: Option<String>,   // caller-supplied system instructions
}
```

`terminal_buffer` is what makes hopbox AI different from a generic chat client — the model sees what the user sees. Providers should fold it into the system message or a leading user message per their own conventions; the trait doesn't dictate placement.

## Cargo.toml — key deps

| Dep            | Why                                                                |
| -------------- | ------------------------------------------------------------------ |
| `portable-pty` | cross-platform PTY (Windows ConPTY, unix openpty) — same as wezterm |
| `serde` + `serde_json` | wire format for protocol + config                             |
| `async-trait`  | async methods on the `AiProvider` trait                            |
| `thiserror`    | ergonomic `Error` enum derivation                                  |
| `tracing`      | structured logging — consumers wire up subscribers                 |
| `tokio` (dev)  | test runtime only — not a hard dep                                 |

## Adding a new AI provider

1. Create `src/ai/<provider>.rs` (e.g. `src/ai/groq.rs`).
2. Add a `pub mod <provider>;` line to `src/ai/mod.rs`.
3. Define a struct (e.g. `GroqProvider`) with config fields (`api_key`, `model`, `base_url`).
4. Implement `pub fn new(...) -> Result<Self>` for construction + key validation.
5. `#[async_trait] impl AiProvider for GroqProvider` — implement `complete()` and `name()`.
6. Inside `complete()`: build the request body from `ctx`, send via your HTTP client of choice (consumers will likely pass one in via constructor for runtime-agnosticism, or you can use `reqwest` behind a feature flag).
7. Map upstream errors into `Error::AiProvider(msg)` — never `unwrap()`.
8. Add unit tests in the same file gated by `#[cfg(test)]`. Use a recorded fixture for the HTTP body; do not hit live APIs in unit tests.
9. Add an integration test under `tests/` that exercises the full path with a mock HTTP server (e.g. `wiremock`) if behavior is non-trivial.

## Testing

```sh
cargo test -- --test-threads=1
```

PTY tests spawn real subprocesses and rely on the controlling terminal — running them in parallel produces flake (one test's child can race another's signal). Pin `--test-threads=1` for the full suite. Unit tests for AI and protocol modules are thread-safe and will still benefit from parallel execution if you run them in isolation (`cargo test ai::` etc.).

`cargo clippy --all-targets -- -D warnings` should be clean before any PR.

## Versioning

- `0.x.y` while the protocol is in flux — minor bumps may break the wire format.
- Public API changes that affect `hopbox-desktop` or `hopbox-server` must bump at minimum the minor and be paired with PRs against both consumers in the same milestone.

## License

MIT. See `LICENSE` at the repo root (add if missing before first publish).

## Consumed by

```toml
# Local workspace dev (hopbox-desktop / hopbox-server checkouts side-by-side):
hopbox-core = { path = "../hopbox-core" }

# Standalone consumer (or CI without sibling checkout):
hopbox-core = { git = "https://github.com/ConflictHQ/hopbox-core" }
```

Tag releases (`v0.1.0`, etc.) so git consumers can pin via `tag = "v0.1.0"`.
