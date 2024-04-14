use kernel::{
    model::{
        candidate::{Candidate, NewCandidate},
        Id,
    },
    repository::{candidate::CandidateRepository, error::RepositoryError},
};
use sea_orm::{EntityTrait, IntoActiveModel, SqlErr, TryIntoModel};

use crate::model::candidate::{ActiveModel, Entity, Model};

use super::DatabaseRepositoryImpl;

impl TryFrom<ActiveModel> for Candidate {
    type Error = RepositoryError;

    fn try_from(c: ActiveModel) -> Result<Self, Self::Error> {
        let model = c
            .try_into_model()
            .map_err(|e| RepositoryError::UnexpectedError(anyhow::anyhow!(e)))?;
        Ok(Candidate::new(
            Id::new(model.id),
            model.name,
            Id::new(model.match_id),
            model.is_winner.and_then(|v| Some(v != 0)),
        ))
    }
}

impl CandidateRepository for DatabaseRepositoryImpl<Candidate> {
    async fn bulk_insert(&self, candidates: Vec<NewCandidate>) -> Result<(), RepositoryError> {
        let models = candidates
            .into_iter()
            .map(|c| {
                Model {
                    id: c.id.value.to_string(),
                    name: c.name,
                    match_id: c.match_id.value.to_string(),
                    is_winner: None,
                }
                .into_active_model()
            })
            .collect::<Vec<_>>();

        Entity::insert_many(models)
            .on_empty_do_nothing()
            .exec(&self.db.0)
            .await
            .map_err(|e| match e.sql_err() {
                Some(SqlErr::UniqueConstraintViolation(str)) => {
                    RepositoryError::DuplicatedRecord(str)
                }
                _ => RepositoryError::UnexpectedError(anyhow::anyhow!(e)),
            })?;

        Ok(())
    }
}
