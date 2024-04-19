use crate::model::stamp::NewStamp;

use super::error::TraqRepositoryError;

pub trait StampTraqRepository {
    fn create(
        &self,
        stamp: NewStamp,
    ) -> impl std::future::Future<Output = Result<(), TraqRepositoryError>> + Send;
}
