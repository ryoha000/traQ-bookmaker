use crate::model::message::{Message, NewMessage, UpdateMessage};

use super::error::TraqRepositoryError;

pub trait MessageTraqRepository {
    fn create(
        &self,
        message: NewMessage,
    ) -> impl std::future::Future<Output = Result<Message, TraqRepositoryError>> + Send;
    fn update(
        &self,
        message: UpdateMessage,
    ) -> impl std::future::Future<Output = Result<(), TraqRepositoryError>> + Send;
}
