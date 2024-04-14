use adapter::modules::RepositoriesModuleExt;
use kernel::traq::{error::TraqRepositoryError, message::MessageTraqRepository};

use crate::model::message::help::{SendHelpMessage, SendSummaryHelpMessage};

use super::{MessageUseCase, MessageUseCaseError};

impl<R: RepositoriesModuleExt> MessageUseCase<R> {
    pub async fn send_summary_help_message(
        &self,
        source: SendSummaryHelpMessage,
    ) -> Result<(), MessageUseCaseError> {
        self.repositories
            .message_traq_repository()
            .create(source.into())
            .await
            .map_err(|e| match e {
                TraqRepositoryError::UnexpectedError(e) => MessageUseCaseError::UnexpectedError(e),
            })?;
        Ok(())
    }
    pub async fn send_help_message(
        &self,
        source: SendHelpMessage,
    ) -> Result<(), MessageUseCaseError> {
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
