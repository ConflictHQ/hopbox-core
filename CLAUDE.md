# CLAUDE.md — hopbox-core

Read `bootstrap.md` first. It is the source of truth for crate layout, design rules, and the AI provider contract.

## Identity

Git author for this repo: `ragelink` / `lmata@weareconflict.com`. Set per-repo `git config user.name` and `user.email` — never rely on global config.

## What this crate is

A **pure Rust library**. There is no `main.rs`, no `[[bin]]`, no CLI. If a task asks you to add a binary entrypoint here, stop and ask — that work probably belongs in `hopbox-desktop` or `hopbox-server`.

## Async runtime stance

**Do not depend on `tokio` (or any runtime) as a regular dependency.** `async-trait` is fine. `tokio` belongs in `[dev-dependencies]` only, for tests. Consumers pick their own runtime; if the library locks one in, downstream consumers (mobile, embedded, alternative server) lose that choice.

If you find yourself reaching for `tokio::spawn`, `tokio::time::sleep`, or anything else from the `tokio` namespace in non-test code, you are about to break the design rule. Stop and reconsider.

## Public API discipline

Think about what `hopbox-desktop` and `hopbox-server` actually need before exposing a type. Re-exports in `lib.rs` are load-bearing — they are the API. Adding a `pub` keyword commits us to that surface across the SemVer window. Default to private; promote only when a real consumer needs it.

Document every public item with a doc comment that explains intent, not just signature.

## Tests

```sh
cargo test -- --test-threads=1
```

PTY tests cannot run in parallel — they spawn subprocesses tied to the same controlling tty. Always pin `--test-threads=1` for the full suite.

For provider tests, use recorded fixtures or `wiremock` — never hit live APIs from `cargo test`.

## License

MIT. Keep the public API small, well-documented, and stable. Anything `pub` is a promise to downstream consumers.
