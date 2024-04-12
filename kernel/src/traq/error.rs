use thiserror::Error;

#[derive(Error, Debug)]
pub enum TraqRepositoryError {
    #[error("Unexpected error: {0}")]
    UnexpectedError(#[from] anyhow::Error),
}
