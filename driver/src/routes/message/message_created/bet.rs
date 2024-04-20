use derive_new::new;
use kernel::model::Id;
use std::sync::Arc;

use app::model::{bet::CreateBet, message::r#match::UpsertMatchMessage};

use crate::module::{Modules, ModulesExt};

#[derive(new)]
pub struct BetArg {
    pub traq_id: String,
    pub candidate_name: String,
    pub amount: i32,
    pub channel_id: String,
    pub message_id: String,
}

pub async fn handle(modules: Arc<Modules>, arg: BetArg) -> anyhow::Result<()> {
    let bet = modules
        .bet_use_case()
        .create_bet(CreateBet::new(
            arg.channel_id.clone(),
            arg.message_id,
            arg.traq_id,
            arg.candidate_name,
            arg.amount,
        ))
        .await?;

    modules
        .message_use_case()
        .upsert_match_message(UpsertMatchMessage::new(
            Id::new(arg.channel_id),
            bet.match_id,
        ))
        .await?;

    Ok(())
}
