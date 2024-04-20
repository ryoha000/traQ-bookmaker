use crate::model::{
    bet::{Bet, NewBetForLatestMatch},
    r#match::Match,
    Id,
};

use super::error::RepositoryError;

pub trait BetRepository {
    fn insert_for_latest_match(
        &self,
        m: NewBetForLatestMatch,
    ) -> impl std::future::Future<Output = Result<Bet, RepositoryError>> + Send;
    fn select_by_match_id(
        &self,
        match_id: Id<Match>,
    ) -> impl std::future::Future<Output = Result<Vec<Bet>, RepositoryError>> + Send;
}
