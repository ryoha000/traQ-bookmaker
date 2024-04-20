use std::sync::Arc;

use adapter::modules::RepositoriesModuleExt;
use derive_new::new;
use kernel::{
    model::{message::NewMessage, user::User, Id},
    repository::user::UserRepository,
    traq::message::MessageTraqRepository,
};

use crate::model::user::CreateUser;

#[derive(new)]
pub struct UserUseCase<R: RepositoriesModuleExt> {
    repositories: Arc<R>,
}

impl<R: RepositoriesModuleExt> UserUseCase<R> {
    pub async fn register_user(&self, source: CreateUser) -> Result<User, UserUseCaseError> {
        self.repositories
            .user_repository()
            .insert(source.into())
            .await
            .map_err(|e| match e {
                _ => UserUseCaseError::UnexpectedError(anyhow::anyhow!(e)),
            })
    }
    pub async fn list_users(&self, channel_id: String) -> Result<Vec<User>, UserUseCaseError> {
        let users = self
            .repositories
            .user_repository()
            .select_by_channel_id(Id::new(channel_id.clone()))
            .await
            .map_err(|e| match e {
                _ => UserUseCaseError::UnexpectedError(anyhow::anyhow!(e)),
            })?;

        self.repositories
            .message_traq_repository()
            .create(NewMessage::new(
                Id::new(channel_id),
                users.iter().fold("".to_string(), |acc, user| {
                    format!("{}\n:@{}: {}pt", acc, user.traq_display_id, user.balance)
                }),
                true,
            ))
            .await
            .map_err(|e| match e {
                _ => UserUseCaseError::UnexpectedError(anyhow::anyhow!(e)),
            })?;
        Ok(users)
    }
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserUseCaseError {
    #[error("Unexpected error: {0}")]
    UnexpectedError(#[from] anyhow::Error),
}
