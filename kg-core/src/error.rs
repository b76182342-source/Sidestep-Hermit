use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Network error: {0}")]
    Network(String),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Authentication required")]
    AuthRequired,

    #[error("Authentication failed: {0}")]
    AuthFailed(String),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Invalid data: {0}")]
    InvalidData(String),

    #[error("Provider error: {0}")]
    Provider(String),

    #[error("Database error: {0}")]
    Database(String),

    #[error("Player error: {0}")]
    Player(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

pub type Result<T> = std::result::Result<T, Error>;
