use crate::model::bet::{Bet, NewBetForLatestMatch};

use super::error::RepositoryError;

pub trait BetRepository {
    fn insert_for_latest_match(
        &self,
        m: NewBetForLatestMatch,
    ) -> impl std::future::Future<Output = Result<Bet, RepositoryError>> + Send;
}
