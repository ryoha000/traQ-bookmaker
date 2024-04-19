use crate::model::{
    channel::Channel,
    r#match::{Match, NewMatch, UpdateMatchForLatest},
    Id,
};

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
    fn find_latest(
        &self,
        channel_id: Id<Channel>,
    ) -> impl std::future::Future<Output = Result<Option<Match>, RepositoryError>> + Send;
    fn delete_latest(
        &self,
        channel_id: Id<Channel>,
    ) -> impl std::future::Future<Output = Result<(), RepositoryError>> + Send;
}
