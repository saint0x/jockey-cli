use thiserror::Error;

#[derive(Error, Debug)]
pub enum JockeyError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Compression error: {0}")]
    Compression(String),

    #[error("Invalid format: {0}")]
    InvalidFormat(String),

    #[error("Processing error: {0}")]
    Processing(String),

    #[error("Configuration error: {0}")]
    Config(String),
}

pub type Result<T> = std::result::Result<T, JockeyError>; 