use kernel::{
    model::{
        user::{self, User},
        Id,
    },
    repository::{error::RepositoryError, user::UserRepository},
};
use sea_orm::{ActiveModelTrait, IntoActiveModel, TryIntoModel};

use crate::model::user::{ActiveModel, Model};

use super::DatabaseRepositoryImpl;

impl TryFrom<ActiveModel> for User {
    type Error = RepositoryError;

    fn try_from(c: ActiveModel) -> Result<Self, Self::Error> {
        let model = c
            .try_into_model()
            .map_err(|e| RepositoryError::UnexpectedError(anyhow::anyhow!(e)))?;
        Ok(User::new(
            Id::new(model.id),
            model.traq_id,
            model.traq_display_id,
            model.channel_id,
            model.balance,
        ))
    }
}

impl UserRepository for DatabaseRepositoryImpl<user::User> {
    async fn insert(&self, user: user::NewUser) -> Result<User, RepositoryError> {
        let model = Model {
            id: user.id.value.to_string(),
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
}
