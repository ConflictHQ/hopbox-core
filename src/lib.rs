pub mod ai;
pub mod config;
pub mod error;
pub mod protocol;
pub mod pty;

pub use error::Error;
pub type Result<T> = std::result::Result<T, Error>;
