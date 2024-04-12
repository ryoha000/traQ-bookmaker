use crate::model::user;

use super::error::RepositoryError;

pub trait UserRepository {
    async fn insert(&self, user: user::NewUser) -> Result<(), RepositoryError>;
}
