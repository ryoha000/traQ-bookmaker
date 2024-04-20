use adapter::modules::RepositoriesModuleExt;
use kernel::model::message::{NewMessage, UpdateMessage};
use kernel::model::r#match::UpdateMatch;
use kernel::model::{statistic, Id};
use kernel::repository::{
    bet::BetRepository, candidate::CandidateRepository, r#match::MatchRepository,
    user::UserRepository,
};
use kernel::traq::{error::TraqRepositoryError, message::MessageTraqRepository};

use crate::model::message::r#match::UpsertMatchMessage;

use super::{MessageUseCase, MessageUseCaseError};

impl<R: RepositoriesModuleExt> MessageUseCase<R> {
    pub async fn upsert_match_message(
        &self,
        source: UpsertMatchMessage,
    ) -> Result<(), MessageUseCaseError> {
        let candidates = self
            .repositories
            .candidate_repository()
            .select_by_match_id(Id::new(source.match_.id.value.clone()))
            .await
            .map_err(|e| MessageUseCaseError::UnexpectedError(anyhow::anyhow!(e)))?;
        let bets = self
            .repositories
            .bet_repository()
            .select_by_match_id(Id::new(source.match_.id.value.clone()))
            .await
            .map_err(|e| MessageUseCaseError::UnexpectedError(anyhow::anyhow!(e)))?;
        let users = self
            .repositories
            .user_repository()
            .select_by_channel_id(Id::new(source.channel_id.value.clone()))
            .await
            .map_err(|e| MessageUseCaseError::UnexpectedError(anyhow::anyhow!(e)))?;

        let statistics = statistic::new_statistics(bets, candidates);

        let content = format!(
            "### 「{}」が作成されました\n{}",
            source.match_.title,
            statistics.iter().fold("".to_string(), |acc, statistic| {
                format!(
                    "- {}\n{}: {}倍({}pt)\n  - {}\n",
                    acc,
                    statistic.candidate.name,
                    statistic.rate,
                    statistic.amount,
                    statistic.bets.iter().fold("".to_string(), |acc, bet| {
                        format!(
                            ":@{}:{}",
                            users
                                .iter()
                                .find(|u| u.id.value == bet.user_id.value)
                                .map(|u| u.traq_display_id.clone())
                                .unwrap_or("unknown".to_string()),
                            acc
                        )
                    })
                )
            })
        );

        match source.match_.message_id {
            Some(message_id) => {
                self.repositories
                    .message_traq_repository()
                    .update(UpdateMessage::new(message_id, content, true))
                    .await
                    .map_err(|e| match e {
                        TraqRepositoryError::UnexpectedError(e) => {
                            MessageUseCaseError::UnexpectedError(e)
                        }
                    })?;
            }
            None => {
                let message = self
                    .repositories
                    .message_traq_repository()
                    .create(NewMessage::new(
                        Id::new(source.channel_id.value.clone()),
                        content,
                        true,
                    ))
                    .await
                    .map_err(|e| match e {
                        TraqRepositoryError::UnexpectedError(e) => {
                            MessageUseCaseError::UnexpectedError(e)
                        }
                    })?;
                self.repositories
                    .match_repository()
                    .update(UpdateMatch::new(source.match_.id, Some(Some(message.id))))
                    .await
                    .map_err(|e| match e {
                        _ => MessageUseCaseError::UnexpectedError(anyhow::anyhow!(e)),
                    })?;
            }
        }

        Ok(())
    }
}
