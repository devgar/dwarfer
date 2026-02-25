use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("Not found")]
    NotFound,
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Conflict: {0}")]
    Conflict(String),
    #[error("Error: {0}")]
    Other(String),
}
