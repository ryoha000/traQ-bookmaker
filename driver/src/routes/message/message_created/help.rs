use std::sync::Arc;

use app::model::message::help::{CommandSummary, SendSummaryHelpMessage};

use crate::module::{Modules, ModulesExt};

pub async fn handle(modules: Arc<Modules>, channel_id: String) -> anyhow::Result<()> {
    modules
        .help_use_case()
        .send_summary_help_message(SendSummaryHelpMessage::new(
            channel_id,
            vec![CommandSummary::new(
                "reg".to_string(),
                "ユーザー登録".to_string(),
                "ユーザーの初期登録を行います".to_string(),
            )],
        ))
        .await?;

    Ok(())
}
