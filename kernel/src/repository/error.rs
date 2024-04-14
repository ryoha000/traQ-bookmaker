use thiserror::Error;

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("Duplicated record: {0}")]
    DuplicatedRecord(String),
    #[error("Unexpected error: {0}")]
    UnexpectedError(#[from] anyhow::Error),
}
