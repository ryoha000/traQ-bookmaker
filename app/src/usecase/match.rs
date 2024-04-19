use std::sync::Arc;

use adapter::modules::RepositoriesModuleExt;
use derive_new::new;
use kernel::model::candidate::NewCandidate;
use kernel::model::channel::Channel;
use kernel::model::message::NewMessage;
use kernel::model::r#match::Match;
use kernel::model::Id;
use kernel::repository::error::RepositoryError;
use kernel::repository::{candidate::CandidateRepository, r#match::MatchRepository};
use kernel::traq::message::MessageTraqRepository;

use crate::model::r#match::{CloseMatch, CreateMatch, FinishMatch};

use super::escape_arg;

#[derive(new)]
pub struct MatchUseCase<R: RepositoriesModuleExt> {
    repositories: Arc<R>,
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

        let channel_id = Id::new(match_.channel_id.value.clone());
        self.repositories
            .message_traq_repository()
            .create(NewMessage::new(
                channel_id,
                format!(
                    "### 「{}」が開始されました\n対象は{}です。\n`@BOT_bookmaker bet {} ポイント数`の形式で参加できます",
                    match_.title,
                    candidates_source.join(", "),
                    escape_arg(&candidates_source[0]),
                ),
                true,
            ))
            .await
            .map_err(|e| match e {
                _ => MatchUseCaseError::UnexpectedError(anyhow::anyhow!(e)),
            })?;

        Ok(match_)
    }
    pub async fn close_match(&self, source: CloseMatch) -> Result<Match, MatchUseCaseError> {
        let match_ = self
            .repositories
            .match_repository()
            .update_for_latest(source.into())
            .await
            .map_err(|e| match e {
                RepositoryError::RecordNotFound(_) => MatchUseCaseError::EnabledMatchNotFound,
                _ => MatchUseCaseError::UnexpectedError(anyhow::anyhow!(e)),
            })?;

        let channel_id = Id::new(match_.channel_id.value.clone());
        self.repositories
            .message_traq_repository()
            .create(NewMessage::new(
                channel_id,
                format!(
                    "### 「{}」への bet を締め切りました\nレートは以下の通りです\nTODO: レートの表示",
                    match_.title
                ),
                true,
            ))
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

        let channel_id = Id::new(match_.channel_id.value.clone());
        self.repositories
            .message_traq_repository()
            .create(NewMessage::new(
                channel_id,
                format!(
                    "### 「{}」の勝者は{}です\nTODO: ユーザーのポイント増減と総ポイント数",
                    match_.title, winner_candidate_name
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
