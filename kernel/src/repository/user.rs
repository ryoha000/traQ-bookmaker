use crate::model::user::{NewUser, User};

use super::error::RepositoryError;

pub trait UserRepository {
    fn insert(
        &self,
        user: NewUser,
    ) -> impl std::future::Future<Output = Result<User, RepositoryError>> + Send;
    fn find_by_traq_id(
        &self,
        traq_id: String,
    ) -> impl std::future::Future<Output = Result<Option<User>, RepositoryError>> + Send;
}
