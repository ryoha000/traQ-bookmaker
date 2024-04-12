use crate::model::message::{Message, NewMessage};

use super::error::TraqRepositoryError;

pub trait MessageTraqRepository {
    fn create(
        &self,
        message: NewMessage,
    ) -> impl std::future::Future<Output = Result<Message, TraqRepositoryError>> + Send;
}
