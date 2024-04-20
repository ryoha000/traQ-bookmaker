use kernel::{
    model::{
        candidate::{Candidate, NewCandidate},
        r#match::Match,
        Id,
    },
    repository::{candidate::CandidateRepository, error::RepositoryError},
};
use sea_orm::{ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter, SqlErr, TryIntoModel};

use crate::model::candidate::{ActiveModel, Column, Entity, Model};

use super::DatabaseRepositoryImpl;

impl From<Model> for Candidate {
    fn from(model: Model) -> Self {
        Candidate::new(
            Id::new(model.id),
            model.name,
            Id::new(model.match_id),
            model.is_winner.and_then(|v| Some(v != 0)),
        )
    }
}

impl TryFrom<ActiveModel> for Candidate {
    type Error = RepositoryError;

    fn try_from(c: ActiveModel) -> Result<Self, Self::Error> {
        let model = c
            .try_into_model()
            .map_err(|e| RepositoryError::UnexpectedError(anyhow::anyhow!(e)))?;
        Ok(model.into())
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
    async fn find_by_name_and_match_id(
        &self,
        name: String,
        match_id: Id<Match>,
    ) -> Result<Option<Candidate>, RepositoryError> {
        let result = Entity::find()
            .filter(Column::Name.eq(name))
            .filter(Column::MatchId.eq(match_id.value))
            .one(&self.db.0)
            .await
            .map_err(|e| RepositoryError::UnexpectedError(anyhow::anyhow!(e)))?;

        match result {
            Some(model) => Ok(Some(model.into_active_model().try_into()?)),
            None => Ok(None),
        }
    }
    async fn select_by_match_id(
        &self,
        match_id: Id<Match>,
    ) -> Result<Vec<Candidate>, RepositoryError> {
        let result = Entity::find()
            .filter(Column::MatchId.eq(match_id.value))
            .all(&self.db.0)
            .await
            .map_err(|e| RepositoryError::UnexpectedError(anyhow::anyhow!(e)))?;

        Ok(result.into_iter().map(|model| model.into()).collect())
    }
}
