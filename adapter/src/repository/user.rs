use kernel::{
    model::{
        user::{self, FindUser, UpdateBalance, User},
        Id,
    },
    repository::{error::RepositoryError, user::UserRepository},
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter, Set, TryIntoModel,
};
use sea_query::OnConflict;

use crate::model::user::{ActiveModel, Column, Entity, Model};

use super::DatabaseRepositoryImpl;

impl From<Model> for User {
    fn from(model: Model) -> Self {
        User::new(
            Id::new(model.id),
            model.traq_id,
            model.traq_display_id,
            model.channel_id,
            model.balance,
        )
    }
}

impl TryFrom<ActiveModel> for User {
    type Error = RepositoryError;

    fn try_from(c: ActiveModel) -> Result<Self, Self::Error> {
        let model = c
            .try_into_model()
            .map_err(|e| RepositoryError::UnexpectedError(anyhow::anyhow!(e)))?;
        Ok(model.into())
    }
}

impl UserRepository for DatabaseRepositoryImpl<user::User> {
    async fn insert(&self, user: user::NewUser) -> Result<User, RepositoryError> {
        let model = Model {
            id: user.id.value,
            traq_id: user.traq_id,
            traq_display_id: user.traq_display_id,
            channel_id: user.channel_id,
            balance: user.balance,
        };

        let result = model
            .into_active_model()
            .save(&self.db.0)
            .await
            .map_err(|e| RepositoryError::UnexpectedError(anyhow::anyhow!(e)))?;

        result.try_into()
    }
    async fn find_by_traq_id_and_channel_id(
        &self,
        user: FindUser,
    ) -> Result<Option<User>, RepositoryError> {
        let result = Entity::find()
            .filter(Column::TraqId.eq(user.traq_id))
            .filter(Column::ChannelId.eq(user.channel_id.value.to_string()))
            .one(&self.db.0)
            .await
            .map_err(|e| RepositoryError::UnexpectedError(anyhow::anyhow!(e)))?;

        match result {
            Some(model) => Ok(Some(model.into_active_model().try_into()?)),
            None => Ok(None),
        }
    }
    async fn select_by_channel_id(
        &self,
        channel_id: Id<String>,
    ) -> Result<Vec<User>, RepositoryError> {
        let result = Entity::find()
            .filter(Column::ChannelId.eq(channel_id.value))
            .all(&self.db.0)
            .await
            .map_err(|e| RepositoryError::UnexpectedError(anyhow::anyhow!(e)))?;

        Ok(result.into_iter().map(|model| model.into()).collect())
    }
    async fn update_balance(&self, users: Vec<UpdateBalance>) -> Result<(), RepositoryError> {
        Entity::insert_many(users.into_iter().map(|u| ActiveModel {
            id: Set(u.user_id.value),
            balance: Set(u.balance),
            ..Default::default()
        }))
        .on_conflict(
            OnConflict::column(Column::Id)
                .update_column(Column::Balance)
                .to_owned(),
        )
        .exec(&self.db.0)
        .await
        .map_err(|e| RepositoryError::UnexpectedError(anyhow::anyhow!(e)))?;

        Ok(())
    }
}
