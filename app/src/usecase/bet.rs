use std::sync::Arc;

use adapter::modules::RepositoriesModuleExt;
use derive_new::new;
use kernel::model::bet::Bet;
use kernel::model::message::NewMessage;
use kernel::model::stamp::{NewStamp, StampType};
use kernel::model::Id;
use kernel::repository::bet::BetRepository;
use kernel::repository::error::RepositoryError;
use kernel::traq::message::MessageTraqRepository;
use kernel::traq::stamp::StampTraqRepository;

#[derive(new)]
pub struct BetUseCase<R: RepositoriesModuleExt> {
    repositories: Arc<R>,
}

impl<R: RepositoriesModuleExt> BetUseCase<R> {
    pub async fn create_bet(&self, source: CreateBet) -> Result<Bet, BetUseCaseError> {
        let message_id = Id::new(source.message_id.clone());
        let channel_id = Id::new(source.channel_id.clone());
        if source.candidate_name == "" || source.amount <= 0 {
            self.repositories
                .message_traq_repository()
                .create(NewMessage::new(
                    channel_id,
                    "引数が不正です\n賭けの対象となる候補を指定し、賭けるポイントは正の整数を指定してください\n`@BOT_bookmaker bet 候補A ポイント数`の形式で指定できます".to_string(),
                    true,
                ))
                .await
                .map_err(|e| match e {
                    _ => BetUseCaseError::UnexpectedError(anyhow::anyhow!(e)),
                })?;
            return Err(BetUseCaseError::AmountMustBePositive);
        }

        let bet_result = self
            .repositories
            .bet_repository()
            .insert_for_latest_match(source.into())
            .await;

        match bet_result {
            Ok(bet) => {
                self.repositories
                    .stamp_repository()
                    .create(NewStamp::new(message_id, StampType::WhiteCheckMark))
                    .await
                    .map_err(|e| match e {
                        _ => BetUseCaseError::UnexpectedError(anyhow::anyhow!(e)),
                    })?;

                // TODO: レートの変化を bet.match.message_id から update

                return Ok(bet);
            }
            Err(e) => match e {
                RepositoryError::RecordNotFound(s) => {
                    let error_with_message = {
                        if s.contains("Match") {
                            (
                                "有効な賭けが見つかりませんでした",
                                BetUseCaseError::EnabledMatchNotFound,
                            )
                        } else if s.contains("Candidate") {
                            (
                                "指定した候補が見つかりませんでした",
                                BetUseCaseError::CandidateNotFound,
                            )
                        } else if s.contains("User") {
                            (
                            "ユーザー登録していません\n`@BOT_bookmaker reg`で先に登録してください",
                            BetUseCaseError::UserNotFound,
                        )
                        } else {
                            (
                                "予期せぬエラーが発生しました",
                                BetUseCaseError::UnexpectedError(anyhow::anyhow!(
                                    "Record not found but not Match or Candidate or User"
                                )),
                            )
                        }
                    };
                    self.repositories
                        .message_traq_repository()
                        .create(NewMessage::new(
                            channel_id,
                            error_with_message.0.to_string(),
                            true,
                        ))
                        .await
                        .map_err(|e| match e {
                            _ => BetUseCaseError::UnexpectedError(anyhow::anyhow!(e)),
                        })?;
                    return Err(error_with_message.1);
                }
                RepositoryError::DuplicatedRecord(_) => {
                    return Err(BetUseCaseError::EnabledBetAlreadyExists)
                }
                _ => return Err(BetUseCaseError::UnexpectedError(anyhow::anyhow!(e))),
            },
        }
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
