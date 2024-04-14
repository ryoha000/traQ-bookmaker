use derive_new::new;
use std::sync::Arc;

use app::model::r#match::CreateMatch;

use crate::module::{Modules, ModulesExt};

#[derive(new)]
pub struct StartArg {
    pub channel_id: String,
    pub title: String,
    pub candidate_names: Vec<String>,
}

pub async fn handle(modules: Arc<Modules>, arg: StartArg) -> anyhow::Result<()> {
    modules
        .match_use_case()
        .create_match(
            CreateMatch::new(arg.title, arg.channel_id),
            arg.candidate_names,
        )
        .await?;

    Ok(())
}
