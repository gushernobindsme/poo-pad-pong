use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("NotFound, id id {0}")]
    NotFound(String),
    #[error("Unexpected Error: [{0}]")]
    Unexpected(String),
}
