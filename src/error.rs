use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("AI provider error: {0}")]
    AiProvider(String),
    #[error("PTY error: {0}")]
    Pty(String),
    #[error("Protocol error: {0}")]
    Protocol(String),
    #[error("Config error: {0}")]
    Config(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}
