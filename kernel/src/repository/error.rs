use thiserror::Error;

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("Unexpected error: {0}")]
    UnexpectedError(#[from] anyhow::Error),
}
