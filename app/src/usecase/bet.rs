use std::sync::Arc;

use adapter::modules::RepositoriesModuleExt;
use derive_new::new;
use kernel::model::bet::Bet;
use kernel::model::message::NewMessage;
use kernel::model::Id;
use kernel::repository::bet::BetRepository;
use kernel::repository::error::RepositoryError;
use kernel::traq::message::MessageTraqRepository;

#[derive(new)]
pub struct BetUseCase<R: RepositoriesModuleExt> {
    repositories: Arc<R>,
}

impl<R: RepositoriesModuleExt> BetUseCase<R> {
    pub async fn create_bet(&self, source: CreateBet) -> Result<Bet, BetUseCaseError> {
        if source.candidate_name == "" || source.amount <= 0 {
            self.repositories
                .message_traq_repository()
                .create(NewMessage::new(
                    Id::new(source.channel_id),
                    "引数が不正です\n賭けの対象となる候補を指定し、賭けるポイントは正の整数を指定してください\n`@BOT_bookmaker bet 候補A`の形式で指定できます".to_string(),
                    true,
                ))
                .await
                .map_err(|e| match e {
                    _ => BetUseCaseError::UnexpectedError(anyhow::anyhow!(e)),
                })?;
            return Err(BetUseCaseError::AmountMustBePositive);
        }

        Ok(self
            .repositories
            .bet_repository()
            .insert_for_latest_match(source.into())
            .await
            .map_err(|e| match e {
                RepositoryError::RecordNotFound(s) => {
                    if s.contains("Match") {
                        BetUseCaseError::EnabledMatchNotFound
                    } else if s.contains("Candidate") {
                        BetUseCaseError::CandidateNotFound
                    } else if s.contains("User") {
                        BetUseCaseError::UserNotFound
                    } else {
                        BetUseCaseError::UnexpectedError(anyhow::anyhow!(
                            "Record not found but not Match or Candidate or User"
                        ))
                    }
                }
                RepositoryError::DuplicatedRecord(_) => BetUseCaseError::EnabledBetAlreadyExists,
                _ => BetUseCaseError::UnexpectedError(anyhow::anyhow!(e)),
            })?)
    }
}

use thiserror::Error;

use crate::model::bet::CreateBet;

#[derive(Error, Debug)]
pub enum BetUseCaseError {
    #[error("Amount must be positive")]
    AmountMustBePositive,
    #[error("Bet already exists")]
    EnabledBetAlreadyExists,
    #[error("Candidate not found")]
    CandidateNotFound,
    #[error("User not found")]
    UserNotFound,
    #[error("Enabled match not found")]
    EnabledMatchNotFound,
    #[error("Unexpected error: {0}")]
    UnexpectedError(#[from] anyhow::Error),
}
