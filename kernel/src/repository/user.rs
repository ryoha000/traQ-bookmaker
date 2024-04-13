use crate::model::user::{self, User};

use super::error::RepositoryError;

pub trait UserRepository {
    fn insert(
        &self,
        user: user::NewUser,
    ) -> impl std::future::Future<Output = Result<User, RepositoryError>> + Send;
}
