use crate::model::r#match::{Match, NewMatch, UpdateMatchForLatest};

use super::error::RepositoryError;

pub trait MatchRepository {
    fn insert(
        &self,
        m: NewMatch,
    ) -> impl std::future::Future<Output = Result<Match, RepositoryError>> + Send;
    fn update_for_latest(
        &self,
        m: UpdateMatchForLatest,
    ) -> impl std::future::Future<Output = Result<Match, RepositoryError>> + Send;
}
