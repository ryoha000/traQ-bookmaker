use app::model::r#match::FinishMatch;
use derive_new::new;
use std::sync::Arc;

use crate::module::{Modules, ModulesExt};

#[derive(new)]
pub struct FinishArg {
    pub channel_id: String,
    pub winner_candidate_name: String,
}

pub async fn handle(modules: Arc<Modules>, arg: FinishArg) -> anyhow::Result<()> {
    modules
        .match_use_case()
        .finish_match(FinishMatch::new(arg.channel_id, arg.winner_candidate_name))
        .await?;

    Ok(())
}
