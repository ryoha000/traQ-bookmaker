use crate::model::{
    candidate::{Candidate, NewCandidate},
    r#match::Match,
    Id,
};

use super::error::RepositoryError;

pub trait CandidateRepository {
    fn bulk_insert(
        &self,
        candidate: Vec<NewCandidate>,
    ) -> impl std::future::Future<Output = Result<(), RepositoryError>> + Send;
    fn find_by_name_and_match_id(
        &self,
        name: String,
        match_id: Id<Match>,
    ) -> impl std::future::Future<Output = Result<Option<Candidate>, RepositoryError>> + Send;
}
