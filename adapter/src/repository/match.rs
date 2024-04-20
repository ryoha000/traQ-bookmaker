use kernel::{
    model::{
        channel::Channel,
        r#match::{Match, NewMatch, UpdateMatch, UpdateMatchForLatest},
        Id,
    },
    repository::{error::RepositoryError, r#match::MatchRepository},
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, IntoActiveModel, QueryFilter, QueryOrder,
    Set, TransactionError, TransactionTrait, TryIntoModel,
};

use crate::model::r#match::{ActiveModel, Column, Entity, Model};

use super::DatabaseRepositoryImpl;

impl From<Model> for Match {
    fn from(model: Model) -> Self {
        Match::new(
            Id::new(model.id),
            model.title,
            Id::new(model.channel_id),
            model.message_id.map(Id::new),
            model.created_at,
            model.closed_at,
            model.winner_candidate_id.map(Id::new),
        )
    }
}

impl TryFrom<ActiveModel> for Match {
    type Error = RepositoryError;

    fn try_from(c: ActiveModel) -> Result<Self, Self::Error> {
        let model = c
            .try_into_model()
            .map_err(|e| RepositoryError::UnexpectedError(anyhow::anyhow!(e)))?;
        Ok(model.into())
    }
}

impl MatchRepository for DatabaseRepositoryImpl<Match> {
    async fn insert(&self, m: NewMatch) -> Result<Match, RepositoryError> {
        self.db
            .0
            .transaction::<_, Match, RepositoryError>(|txn| {
                Box::pin(async move {
                    // 同じ channel_id で finished_at が null の match が存在する場合はエラー
                    let exists = Entity::find()
                        .filter(Column::ChannelId.eq(&m.channel_id.value.to_string()))
                        .filter(Column::WinnerCandidateId.is_null())
                        .all(txn)
                        .await
                        .map_err(|e| RepositoryError::UnexpectedError(anyhow::anyhow!(e)))?;
                    if exists.len() > 0 {
                        return Err(RepositoryError::DuplicatedRecord(
                            "Match with the same channel_id already exists".to_string(),
                        ));
                    }

                    let model = Model {
                        id: m.id.value.to_string(),
                        title: m.title,
                        channel_id: m.channel_id.value.to_string(),
                        message_id: None,
                        created_at: m.created_at,
                        closed_at: None,
                        winner_candidate_id: None,
                    };

                    let result = model
                        .into_active_model()
                        .save(txn)
                        .await
                        .map_err(|e| RepositoryError::UnexpectedError(anyhow::anyhow!(e)))?;

                    result.try_into()
                })
            })
            .await
            .map_err(|e| match e {
                TransactionError::Transaction(repo_err) => repo_err,
                _ => RepositoryError::UnexpectedError(anyhow::anyhow!(e)),
            })
    }
    async fn update(&self, m: UpdateMatch) -> Result<Match, RepositoryError> {
        self.db
            .0
            .transaction::<_, Match, RepositoryError>(|txn| {
                Box::pin(async move {
                    let model = Entity::find()
                        .filter(Column::Id.eq(&m.id.value.to_string()))
                        .one(txn)
                        .await
                        .map_err(|e| match e {
                            DbErr::RecordNotFound(str) => RepositoryError::RecordNotFound(str),
                            _ => RepositoryError::UnexpectedError(anyhow::anyhow!(e)),
                        })?;

                    let model = model.ok_or(RepositoryError::RecordNotFound(
                        "Match not found".to_string(),
                    ))?;
                    let mut match_ = model.into_active_model();

                    if let Some(message_id) = m.message_id {
                        match_.message_id = Set(message_id.map(|id| id.value));
                    }
                    Ok(match_
                        .update(txn)
                        .await
                        .map_err(|e| RepositoryError::UnexpectedError(anyhow::anyhow!(e)))?
                        .into())
                })
            })
            .await
            .map_err(|e| match e {
                TransactionError::Transaction(repo_err) => repo_err,
                _ => RepositoryError::UnexpectedError(anyhow::anyhow!(e)),
            })
    }
    async fn update_for_latest(&self, m: UpdateMatchForLatest) -> Result<Match, RepositoryError> {
        self.db
            .0
            .transaction::<_, Match, RepositoryError>(|txn| {
                Box::pin(async move {
                    let channel_id = m.channel_id;
                    let model = Entity::find()
                        .filter(Column::ChannelId.eq(&channel_id.value))
                        .order_by_desc(Column::CreatedAt)
                        .one(txn)
                        .await
                        .map_err(|e| match e {
                            DbErr::RecordNotFound(str) => RepositoryError::RecordNotFound(str),
                            _ => RepositoryError::UnexpectedError(anyhow::anyhow!(e)),
                        })?;

                    let model = model.ok_or(RepositoryError::RecordNotFound(
                        "Match with the same channel_id not found".to_string(),
                    ))?;
                    let match_id = model.id.clone();
                    let mut match_ = model.into_active_model();

                    if let Some(closed_at) = m.closed_at {
                        if match_.closed_at.is_set() {
                            return Err(RepositoryError::DuplicatedRecord(
                                "Match is already closed".to_string(),
                            ));
                        }
                        match_.closed_at = Set(closed_at);
                    }
                    if let Some(winner_candidate_name) = m.winner_candidate_name {
                        if match_.winner_candidate_id.is_set() {
                            return Err(RepositoryError::DuplicatedRecord(
                                "Winner candidate is already set".to_string(),
                            ));
                        }
                        match winner_candidate_name {
                            Some(name) => {
                                let candidate = crate::model::candidate::Entity::find()
                                    .filter(crate::model::candidate::Column::MatchId.eq(match_id))
                                    .filter(crate::model::candidate::Column::Name.eq(name))
                                    .one(txn)
                                    .await
                                    .map_err(|e| {
                                        RepositoryError::UnexpectedError(anyhow::anyhow!(e))
                                    })?
                                    .ok_or(RepositoryError::RecordNotFound(
                                        "Candidate not found".to_string(),
                                    ))?;
                                match_.winner_candidate_id = Set(Some(candidate.id));
                                return Err(RepositoryError::DuplicatedRecord(
                                    "Winner candidate is already set".to_string(),
                                ));
                            }
                            None => {
                                match_.winner_candidate_id = Set(None);
                            }
                        }
                    }
                    Ok(match_
                        .update(txn)
                        .await
                        .map_err(|e| RepositoryError::UnexpectedError(anyhow::anyhow!(e)))?
                        .into())
                })
            })
            .await
            .map_err(|e| match e {
                TransactionError::Transaction(repo_err) => repo_err,
                _ => RepositoryError::UnexpectedError(anyhow::anyhow!(e)),
            })
    }
    async fn find(&self, match_id: Id<Match>) -> Result<Option<Match>, RepositoryError> {
        let result = Entity::find()
            .filter(Column::Id.eq(&match_id.value.to_string()))
            .one(&self.db.0)
            .await
            .map_err(|e| RepositoryError::UnexpectedError(anyhow::anyhow!(e)))?;

        match result {
            Some(model) => Ok(Some(model.into())),
            None => Ok(None),
        }
    }
    async fn find_latest(&self, channel_id: Id<Channel>) -> Result<Option<Match>, RepositoryError> {
        let result = Entity::find()
            .filter(Column::ChannelId.eq(&channel_id.value.to_string()))
            .order_by_desc(Column::CreatedAt)
            .one(&self.db.0)
            .await
            .map_err(|e| RepositoryError::UnexpectedError(anyhow::anyhow!(e)))?;

        match result {
            Some(model) => Ok(Some(model.into())),
            None => Ok(None),
        }
    }
    async fn delete_latest(&self, channel_id: Id<Channel>) -> Result<(), RepositoryError> {
        let model = Entity::find()
            .filter(Column::ChannelId.eq(&channel_id.value.to_string()))
            .filter(Column::WinnerCandidateId.is_null())
            .order_by_desc(Column::CreatedAt)
            .one(&self.db.0)
            .await
            .map_err(|e| RepositoryError::UnexpectedError(anyhow::anyhow!(e)))?;

        match model {
            Some(model) => {
                model
                    .into_active_model()
                    .delete(&self.db.0)
                    .await
                    .map_err(|e| RepositoryError::UnexpectedError(anyhow::anyhow!(e)))?;
                Ok(())
            }
            None => Err(RepositoryError::RecordNotFound(
                "Match with the same channel_id not found".to_string(),
            )),
        }
    }
}
