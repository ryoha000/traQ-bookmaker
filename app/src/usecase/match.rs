use std::sync::Arc;

use adapter::modules::RepositoriesModuleExt;
use derive_new::new;
use kernel::model::candidate::NewCandidate;
use kernel::model::channel::Channel;
use kernel::model::message::NewMessage;
use kernel::model::r#match::Match;
use kernel::model::stamp::{NewStamp, StampType};
use kernel::model::user::UpdateBalance;
use kernel::model::{statistic, Id};
use kernel::repository::error::RepositoryError;
use kernel::repository::{
    bet::BetRepository, candidate::CandidateRepository, r#match::MatchRepository,
    user::UserRepository,
};
use kernel::traq::{message::MessageTraqRepository, stamp::StampTraqRepository};

use crate::model::r#match::{CloseMatch, CreateMatch, FinishMatch};

#[derive(new)]
pub struct MatchUseCase<R: RepositoriesModuleExt> {
    repositories: Arc<R>,
}

struct BalanceDiff {
    pub traq_display_id: String,
    pub diff: i32,
    pub result: i32,
}

impl<R: RepositoriesModuleExt> MatchUseCase<R> {
    pub async fn create_match(
        &self,
        match_source: CreateMatch,
        candidates_source: Vec<String>,
    ) -> Result<Match, MatchUseCaseError> {
        if candidates_source.len() < 2 {
            self.repositories
                .message_traq_repository()
                .create(NewMessage::new(
                    Id::new(match_source.channel_id),
                    "賭けの対象となる候補を2つ以上指定してください\n`@BOT_bookmaker start 賭け名 候補A 候補B`の形式で指定できます".to_string(),
                    true,
                ))
                .await
                .map_err(|e| match e {
                    _ => MatchUseCaseError::UnexpectedError(anyhow::anyhow!(e)),
                })?;
            return Err(MatchUseCaseError::CandidateMustNotBeEmpty);
        }

        let match_ = self
            .repositories
            .match_repository()
            .insert(match_source.into())
            .await
            .map_err(|e| match e {
                RepositoryError::DuplicatedRecord(_) => {
                    MatchUseCaseError::EnabledMatchAlreadyExists
                }
                _ => MatchUseCaseError::UnexpectedError(anyhow::anyhow!(e)),
            })?;

        let match_id_str = match_.id.value.clone();
        let new_candidates = candidates_source
            .clone()
            .into_iter()
            .map(|name| NewCandidate::new(Id::gen(), name, Id::new(match_id_str.clone())))
            .collect::<Vec<_>>();

        self.repositories
            .candidate_repository()
            .bulk_insert(new_candidates)
            .await
            .map_err(|e| match e {
                _ => MatchUseCaseError::UnexpectedError(anyhow::anyhow!(e)),
            })?;

        Ok(match_)
    }
    pub async fn close_match(&self, source: CloseMatch) -> Result<Match, MatchUseCaseError> {
        let message_id = Id::new(source.message_id.clone());
        let match_ = self
            .repositories
            .match_repository()
            .update_for_latest(source.into())
            .await
            .map_err(|e| match e {
                RepositoryError::RecordNotFound(_) => MatchUseCaseError::EnabledMatchNotFound,
                _ => MatchUseCaseError::UnexpectedError(anyhow::anyhow!(e)),
            })?;

        self.repositories
            .stamp_repository()
            .create(NewStamp::new(message_id, StampType::WhiteCheckMark))
            .await
            .map_err(|e| match e {
                _ => MatchUseCaseError::UnexpectedError(anyhow::anyhow!(e)),
            })?;

        Ok(match_)
    }
    pub async fn finish_match(&self, source: FinishMatch) -> Result<Match, MatchUseCaseError> {
        let channel_id = Id::new(source.channel_id.clone());
        let winner_candidate_name = source.winner_candidate_name.clone();
        let match_result = self
            .repositories
            .match_repository()
            .update_for_latest(source.into())
            .await;

        let match_ = match match_result {
            Ok(match_) => match_,
            Err(e) => {
                let error_with_message = {
                    match e {
                        RepositoryError::RecordNotFound(_) => (
                            "有効な賭けが見つかりませんでした",
                            MatchUseCaseError::EnabledMatchNotFound,
                        ),
                        RepositoryError::DuplicatedRecord(_) => (
                            "賭けの勝者は既に設定されています",
                            MatchUseCaseError::WinnerCandidateAlreadySet,
                        ),
                        _ => (
                            "予期せぬエラーが発生しました",
                            MatchUseCaseError::UnexpectedError(anyhow::anyhow!(e)),
                        ),
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
                        _ => MatchUseCaseError::UnexpectedError(anyhow::anyhow!(e)),
                    })?;
                return Err(error_with_message.1);
            }
        };

        // ポイントの増減計算
        let candidates = self
            .repositories
            .candidate_repository()
            .select_by_match_id(Id::new(match_.id.value.clone()))
            .await
            .map_err(|e| match e {
                _ => MatchUseCaseError::UnexpectedError(anyhow::anyhow!(e)),
            })?;
        let bets = self
            .repositories
            .bet_repository()
            .select_by_match_id(Id::new(match_.id.value.clone()))
            .await
            .map_err(|e| match e {
                _ => MatchUseCaseError::UnexpectedError(anyhow::anyhow!(e)),
            })?;
        let statistics = statistic::new_statistics(bets, candidates);

        // ポイントの増減処理
        let users = self
            .repositories
            .user_repository()
            .select_by_channel_id(Id::new(channel_id.value.clone()))
            .await
            .map_err(|e| match e {
                _ => MatchUseCaseError::UnexpectedError(anyhow::anyhow!(e)),
            })?;
        let mut diffs = Vec::new();
        let mut update_balance_vec = Vec::new();
        for statistic in statistics {
            statistic.bets.into_iter().for_each(|bet| {
                users
                    .iter()
                    .find(|u| u.id.value == bet.user_id.value)
                    .map(|user| {
                        let is_correct = statistic.candidate.name == winner_candidate_name;
                        // 内部的には bet したタイミングでポイントを差し引いているので、プラスだけを考えるが送信するメッセージにはマイナスも見せるようにする
                        let mut plus = 0.0;

                        if is_correct {
                            plus = bet.amount as f64 * statistic.rate;
                            update_balance_vec.push(UpdateBalance::new(
                                Id::new(user.id.value.clone()),
                                user.balance + plus as i32,
                            ));
                        }

                        diffs.push(BalanceDiff {
                            traq_display_id: user.traq_display_id.clone(),
                            diff: plus as i32 - bet.amount,
                            result: user.balance + plus as i32,
                        });
                    });
            });
        }
        self.repositories
            .user_repository()
            .update_balance(update_balance_vec)
            .await
            .map_err(|e| match e {
                _ => MatchUseCaseError::UnexpectedError(anyhow::anyhow!(e)),
            })?;

        let channel_id = Id::new(match_.channel_id.value.clone());
        self.repositories
            .message_traq_repository()
            .create(NewMessage::new(
                channel_id,
                format!(
                    "### 「{}」の勝者は{}です\n{}",
                    match_.title,
                    winner_candidate_name,
                    diffs.iter().fold("".to_string(), |acc, diff| {
                        format!(
                            "{}\n::@{}: {}pt({}pt)",
                            acc,
                            diff.traq_display_id,
                            diff.diff.abs(),
                            diff.result
                        )
                    })
                ),
                true,
            ))
            .await
            .map_err(|e| match e {
                _ => MatchUseCaseError::UnexpectedError(anyhow::anyhow!(e)),
            })?;

        Ok(match_)
    }
    pub async fn delete_match(&self, channel_id: Id<Channel>) -> Result<(), MatchUseCaseError> {
        let delete_result = self
            .repositories
            .match_repository()
            .delete_latest(Id::new(channel_id.value.clone()))
            .await;
        if let Err(e) = delete_result {
            match e {
                RepositoryError::RecordNotFound(_) => {
                    self.repositories
                        .message_traq_repository()
                        .create(NewMessage::new(
                            channel_id,
                            "有効な賭けが見つかりませんでした".to_string(),
                            true,
                        ))
                        .await
                        .map_err(|e| match e {
                            _ => MatchUseCaseError::UnexpectedError(anyhow::anyhow!(e)),
                        })?;
                    return Err(MatchUseCaseError::EnabledMatchNotFound);
                }
                _ => {
                    return Err(MatchUseCaseError::UnexpectedError(anyhow::anyhow!(e)));
                }
            }
        }

        self.repositories
            .message_traq_repository()
            .create(NewMessage::new(
                channel_id,
                "最新の賭けをキャンセルしました".to_string(),
                true,
            ))
            .await
            .map_err(|e| match e {
                _ => MatchUseCaseError::UnexpectedError(anyhow::anyhow!(e)),
            })?;

        Ok(())
    }
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum MatchUseCaseError {
    #[error("Winner candidate already set")]
    WinnerCandidateAlreadySet,
    #[error("Candidates must not be empty")]
    CandidateMustNotBeEmpty,
    #[error("Enabled match already exists")]
    EnabledMatchAlreadyExists,
    #[error("Enabled match not found")]
    EnabledMatchNotFound,
    #[error("Unexpected error: {0}")]
    UnexpectedError(#[from] anyhow::Error),
}
