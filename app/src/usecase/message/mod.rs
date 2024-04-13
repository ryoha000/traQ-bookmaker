pub mod help;

use std::sync::Arc;

use adapter::modules::RepositoriesModuleExt;
use derive_new::new;

#[derive(new)]
pub struct MessageUseCase<R: RepositoriesModuleExt> {
    repositories: Arc<R>,
}

impl<R: RepositoriesModuleExt> MessageUseCase<R> {}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum MessageUseCaseError {
    #[error("Unexpected error: {0}")]
    UnexpectedError(#[from] anyhow::Error),
}
