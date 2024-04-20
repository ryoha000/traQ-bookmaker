use derive_new::new;
use std::sync::Arc;

use app::model::r#match::CloseMatch;

use crate::module::{Modules, ModulesExt};

#[derive(new)]
pub struct CloseArg {
    pub channel_id: String,
    pub message_id: String,
}

pub async fn handle(modules: Arc<Modules>, arg: CloseArg) -> anyhow::Result<()> {
    modules
        .match_use_case()
        .close_match(CloseMatch::new(arg.channel_id, arg.message_id))
        .await?;

    Ok(())
}
