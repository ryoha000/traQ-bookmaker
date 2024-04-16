use std::sync::Arc;

use adapter::modules::RepositoriesModuleExt;
use derive_new::new;
use kernel::model::candidate::NewCandidate;
use kernel::model::message::NewMessage;
use kernel::model::r#match::Match;
use kernel::model::Id;
use kernel::repository::error::RepositoryError;
use kernel::repository::{candidate::CandidateRepository, r#match::MatchRepository};
use kernel::traq::message::MessageTraqRepository;

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
                    "賭けの対象となる候補を2つ以上指定してください\n`@BOT_bookmaker match create 候補A 候補B`の形式で指定できます".to_string(),
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
                    "### 「{}」が開始されました\n対象は{}です。\n`@BOT_bookmaker bet {}`の形式で参加できます",
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
}

use thiserror::Error;

use crate::model::r#match::CreateMatch;

use super::escape_arg;

#[derive(Error, Debug)]
pub enum MatchUseCaseError {
    #[error("Candidates must not be empty")]
    CandidateMustNotBeEmpty,
    #[error("Enabled match already exists")]
    EnabledMatchAlreadyExists,
    #[error("Unexpected error: {0}")]
    UnexpectedError(#[from] anyhow::Error),
}