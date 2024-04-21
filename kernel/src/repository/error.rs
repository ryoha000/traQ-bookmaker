use thiserror::Error;

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("Record not found: {0}")]
    RecordNotFound(String),
    #[error("Duplicated record: {0}")]
    DuplicatedRecord(String),
    #[error("Insufficient balance")]
    InsufficientBalance,
    #[error("Unexpected error: {0}")]
    UnexpectedError(#[from] anyhow::Error),
}
