use crate::model::candidate::NewCandidate;

use super::error::RepositoryError;

pub trait CandidateRepository {
    fn bulk_insert(
        &self,
        candidate: Vec<NewCandidate>,
    ) -> impl std::future::Future<Output = Result<(), RepositoryError>> + Send;
}
