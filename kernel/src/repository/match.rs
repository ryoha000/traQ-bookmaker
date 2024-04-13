use crate::model::r#match::{Match, NewMatch};

use super::error::RepositoryError;

pub trait MatchRepository {
    fn insert(
        &self,
        m: NewMatch,
    ) -> impl std::future::Future<Output = Result<Match, RepositoryError>> + Send;
}
