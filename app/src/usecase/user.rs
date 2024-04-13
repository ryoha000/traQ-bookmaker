use std::sync::Arc;

use adapter::modules::RepositoriesModuleExt;
use derive_new::new;
use kernel::{
    model::user::User,
    repository::{error::RepositoryError, user::UserRepository},
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
                RepositoryError::UnexpectedError(e) => UserUseCaseError::UnexpectedError(e),
            })
    }
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserUseCaseError {
    #[error("Unexpected error: {0}")]
    UnexpectedError(#[from] anyhow::Error),
}
