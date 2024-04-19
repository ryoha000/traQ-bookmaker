use kernel::{
    model::{
        channel::Channel,
        r#match::{Match, NewMatch, UpdateMatchForLatest},
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
            model.finished_at,
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
                        .filter(Column::FinishedAt.is_null())
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
                        finished_at: None,
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

                    let mut match_ = model
                        .ok_or(RepositoryError::RecordNotFound(
                            "Match with the same channel_id not found".to_string(),
                        ))?
                        .into_active_model();

                    if let Some(closed_at) = m.closed_at {
                        match_.closed_at = Set(closed_at);
                    }
                    if let Some(finished_at) = m.finished_at {
                        match_.finished_at = Set(finished_at);
                    }
                    match_
                        .update(txn)
                        .await
                        .map_err(|e| RepositoryError::UnexpectedError(anyhow::anyhow!(e)))?
                        .into_active_model()
                        .try_into()
                })
            })
            .await
            .map_err(|e| match e {
                TransactionError::Transaction(repo_err) => repo_err,
                _ => RepositoryError::UnexpectedError(anyhow::anyhow!(e)),
            })
    }
    async fn find_latest(&self, channel_id: Id<Channel>) -> Result<Option<Match>, RepositoryError> {
        let result = Entity::find()
            .filter(Column::ChannelId.eq(&channel_id.value.to_string()))
            .order_by_desc(Column::CreatedAt)
            .one(&self.db.0)
            .await
            .map_err(|e| RepositoryError::UnexpectedError(anyhow::anyhow!(e)))?;

        match result {
            Some(model) => Ok(Some(model.into_active_model().try_into()?)),
            None => Ok(None),
        }
    }
    async fn delete_latest(&self, channel_id: Id<Channel>) -> Result<(), RepositoryError> {
        let model = Entity::find()
            .filter(Column::ChannelId.eq(&channel_id.value.to_string()))
            .filter(Column::FinishedAt.is_null())
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
