use kernel::{
    model::user,
    repository::{error::RepositoryError, user::UserRepository},
};
use sea_orm::{EntityTrait, Set};

use crate::model::user::{ActiveModel, Entity};

use super::DatabaseRepositoryImpl;

impl UserRepository for DatabaseRepositoryImpl<user::User> {
    async fn insert(&self, user: user::NewUser) -> Result<(), RepositoryError> {
        Entity::insert(ActiveModel {
            id: Set(user.id.value.to_string()),
            traq_id: Set(user.traq_id),
            traq_display_id: Set(user.traq_display_id),
            channel_id: Set(user.channel_id),
            balance: Set(user.balance),
        })
        .exec(&self.db.0)
        .await
        .map_err(|e| RepositoryError::UnexpectedError(anyhow::anyhow!(e)))?;
        Ok(())
    }
}
