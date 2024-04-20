use derive_new::new;
use kernel::model::Id;
use std::sync::Arc;

use app::model::{message::r#match::UpsertMatchMessage, r#match::CreateMatch};

use crate::module::{Modules, ModulesExt};

#[derive(new)]
pub struct StartArg {
    pub channel_id: String,
    pub title: String,
    pub candidate_names: Vec<String>,
}

pub async fn handle(modules: Arc<Modules>, arg: StartArg) -> anyhow::Result<()> {
    let match_ = modules
        .match_use_case()
        .create_match(
            CreateMatch::new(arg.title, arg.channel_id.clone()),
            arg.candidate_names,
        )
        .await?;

    modules
        .message_use_case()
        .upsert_match_message(UpsertMatchMessage::new(Id::new(arg.channel_id), match_.id))
        .await?;

    Ok(())
}
