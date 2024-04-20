use crate::model::user::{FindUser, NewUser, User};

use super::error::RepositoryError;

pub trait UserRepository {
    fn insert(
        &self,
        user: NewUser,
    ) -> impl std::future::Future<Output = Result<User, RepositoryError>> + Send;
    fn find_by_traq_id_and_channel_id(
        &self,
        user: FindUser,
    ) -> impl std::future::Future<Output = Result<Option<User>, RepositoryError>> + Send;
}
