use crate::model::user;

use super::error::RepositoryError;

pub trait UserRepository {
    fn insert(
        &self,
        user: user::NewUser,
    ) -> impl std::future::Future<Output = Result<(), RepositoryError>> + Send;
}
