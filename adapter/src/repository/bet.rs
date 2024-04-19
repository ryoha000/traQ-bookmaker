use kernel::{
    model::{
        bet::{Bet, NewBetForLatestMatch},
        Id,
    },
    repository::{bet::BetRepository, error::RepositoryError},
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter, QueryOrder, SqlErr,
    TransactionError, TransactionTrait, TryIntoModel,
};

use crate::model::bet::{ActiveModel, Model};

use super::DatabaseRepositoryImpl;

impl From<Model> for Bet {
    fn from(model: Model) -> Self {
        Bet::new(
            Id::new(model.id),
            Id::new(model.user_id),
            Id::new(model.match_id),
            Id::new(model.candidate_id),
            model.amount,
        )
    }
}

impl TryFrom<ActiveModel> for Bet {
    type Error = RepositoryError;

    fn try_from(c: ActiveModel) -> Result<Self, Self::Error> {
        let model = c
            .try_into_model()
            .map_err(|e| RepositoryError::UnexpectedError(anyhow::anyhow!(e)))?;
        Ok(model.into())
    }
}

impl BetRepository for DatabaseRepositoryImpl<Bet> {
    async fn insert_for_latest_match(
        &self,
        m: NewBetForLatestMatch,
    ) -> Result<Bet, RepositoryError> {
        self.db
            .0
            .transaction::<_, Bet, RepositoryError>(|txn| {
                Box::pin(async move {
                    let match_ = crate::model::r#match::Entity::find()
                        .filter(crate::model::r#match::Column::ClosedAt.is_null())
                        .filter(crate::model::r#match::Column::WinnerCandidateId.is_null())
                        .order_by_desc(crate::model::r#match::Column::CreatedAt)
                        .one(txn)
                        .await
                        .map_err(|e| RepositoryError::UnexpectedError(anyhow::anyhow!(e)))?
                        .ok_or(RepositoryError::RecordNotFound(
                            "Match not found".to_string(),
                        ))?;

                    let candidate = crate::model::candidate::Entity::find()
                        .filter(crate::model::candidate::Column::MatchId.eq(&match_.id))
                        .filter(crate::model::candidate::Column::Name.eq(&m.candidate_name))
                        .one(txn)
                        .await
                        .map_err(|e| RepositoryError::UnexpectedError(anyhow::anyhow!(e)))?
                        .ok_or(RepositoryError::RecordNotFound(
                            "Candidate not found".to_string(),
                        ))?;

                    let user = crate::model::user::Entity::find()
                        .filter(crate::model::user::Column::TraqId.eq(&m.traq_id))
                        .filter(crate::model::user::Column::ChannelId.eq(&m.channel_id))
                        .one(txn)
                        .await
                        .map_err(|e| RepositoryError::UnexpectedError(anyhow::anyhow!(e)))?
                        .ok_or(RepositoryError::RecordNotFound(
                            "User not found".to_string(),
                        ))?;

                    let model = Model {
                        id: m.id.value.to_string(),
                        user_id: user.id,
                        match_id: match_.id,
                        candidate_id: candidate.id,
                        amount: m.amount,
                    };

                    let result =
                        model.into_active_model().save(txn).await.map_err(|e| {
                            match e.sql_err() {
                                Some(SqlErr::UniqueConstraintViolation(s)) => {
                                    RepositoryError::DuplicatedRecord(s.to_string())
                                }
                                _ => RepositoryError::UnexpectedError(anyhow::anyhow!(e)),
                            }
                        })?;

                    result.try_into()
                })
            })
            .await
            .map_err(|e| match e {
                TransactionError::Transaction(repo_err) => repo_err,
                _ => RepositoryError::UnexpectedError(anyhow::anyhow!(e)),
            })
    }
}
