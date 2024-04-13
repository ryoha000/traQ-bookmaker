use std::sync::Arc;

use adapter::modules::RepositoriesModuleExt;
use derive_new::new;
use kernel::traq::{error::TraqRepositoryError, message::MessageTraqRepository};

use crate::model::message::help::SendSummaryHelpMessage;

#[derive(new)]
pub struct HelpUseCase<R: RepositoriesModuleExt> {
    repositories: Arc<R>,
}

impl<R: RepositoriesModuleExt> HelpUseCase<R> {
    pub async fn send_summary_help_message(
        &self,
        source: SendSummaryHelpMessage,
    ) -> Result<(), HelpUseCaseError> {
        self.repositories
            .message_traq_repository()
            .create(source.into())
            .await
            .map_err(|e| match e {
                TraqRepositoryError::UnexpectedError(e) => HelpUseCaseError::UnexpectedError(e),
            })?;
        Ok(())
    }
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum HelpUseCaseError {
    #[error("Unexpected error: {0}")]
    UnexpectedError(#[from] anyhow::Error),
}
