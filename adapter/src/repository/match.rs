use kernel::{
    model::{
        r#match::{Match, MatchStatus, NewMatch},
        Id,
    },
    repository::{error::RepositoryError, r#match::MatchRepository},
};
use sea_orm::{ActiveModelTrait, IntoActiveModel, TryIntoModel};

use crate::model::{
    r#match::{ActiveModel, Model},
    sea_orm_active_enums::Status,
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
            model.deadline_at,
            model.status.into(),
        ))
    }
}

impl From<MatchStatus> for Status {
    fn from(c: MatchStatus) -> Self {
        match c {
            MatchStatus::Scheduled => Status::Scheduled,
            MatchStatus::OnGoing => Status::OnGoing,
            MatchStatus::Finished => Status::Finished,
            MatchStatus::Cancelled => Status::Cancelled,
        }
    }
}
impl From<Status> for MatchStatus {
    fn from(c: Status) -> Self {
        match c {
            Status::Scheduled => MatchStatus::Scheduled,
            Status::OnGoing => MatchStatus::OnGoing,
            Status::Finished => MatchStatus::Finished,
            Status::Cancelled => MatchStatus::Cancelled,
        }
    }
}

impl MatchRepository for DatabaseRepositoryImpl<Match> {
    async fn insert(&self, m: NewMatch) -> Result<Match, RepositoryError> {
        let model = Model {
            id: m.id.value.to_string(),
            title: m.title,
            channel_id: m.channel_id.value.to_string(),
            message_id: None,
            created_at: m.created_at,
            deadline_at: None,
            status: m.status.into(),
        };

        let result = model
            .into_active_model()
            .save(&self.db.0)
            .await
            .map_err(|e| RepositoryError::UnexpectedError(anyhow::anyhow!(e)))?;

        result.try_into()
    }
}
