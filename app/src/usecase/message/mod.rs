pub mod help;

use std::sync::Arc;

use adapter::modules::RepositoriesModuleExt;
use derive_new::new;
use kernel::traq::{error::TraqRepositoryError, message::MessageTraqRepository};

#[derive(new)]
pub struct MessageUseCase<R: RepositoriesModuleExt> {
    repositories: Arc<R>,
}

impl<R: RepositoriesModuleExt> MessageUseCase<R> {
    pub async fn send_message(&self, source: SendMessage) -> Result<(), MessageUseCaseError> {
        self.repositories
            .message_traq_repository()
            .create(source.into())
            .await
            .map_err(|e| match e {
                TraqRepositoryError::UnexpectedError(e) => MessageUseCaseError::UnexpectedError(e),
            })?;
        Ok(())
    }
}

use thiserror::Error;

use crate::model::message::SendMessage;

#[derive(Error, Debug)]
pub enum MessageUseCaseError {
    #[error("Unexpected error: {0}")]
    UnexpectedError(#[from] anyhow::Error),
}
