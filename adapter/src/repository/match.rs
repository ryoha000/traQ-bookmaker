use kernel::{
    model::{
        r#match::{Match, NewMatch},
        Id,
    },
    repository::{error::RepositoryError, r#match::MatchRepository},
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter, TransactionError,
    TransactionTrait, TryIntoModel,
};

use crate::model::{
    candidate::Entity,
    r#match::{ActiveModel, Column, Model},
};

use super::DatabaseRepositoryImpl;

impl TryFrom<ActiveModel> for Match {
    type Error = RepositoryError;

    fn try_from(c: ActiveModel) -> Result<Self, Self::Error> {
        let model = c
            .try_into_model()
            .map_err(|e| RepositoryError::UnexpectedError(anyhow::anyhow!(e)))?;
        Ok(Match::new(
            Id::new(model.id),
            model.title,
            Id::new(model.channel_id),
            model.message_id.map(Id::new),
            model.created_at,
            model.closed_at,
            model.finished_at,
        ))
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
}
