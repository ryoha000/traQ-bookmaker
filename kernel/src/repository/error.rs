use thiserror::Error;

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("Enabled match already exists")]
    EnabledMatchAlreadyExists,
    #[error("Unexpected error: {0}")]
    UnexpectedError(#[from] anyhow::Error),
}
