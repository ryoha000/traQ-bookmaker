use crate::model::{
    user::{FindUser, NewUser, UpdateBalance, User},
    Id,
};

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
    fn select_by_channel_id(
        &self,
        channel_id: Id<String>,
    ) -> impl std::future::Future<Output = Result<Vec<User>, RepositoryError>> + Send;
    fn update_balance(
        &self,
        users: Vec<UpdateBalance>,
    ) -> impl std::future::Future<Output = Result<(), RepositoryError>> + Send;
}
