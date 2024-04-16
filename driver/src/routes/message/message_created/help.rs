use std::sync::Arc;

use app::model::message::help::{CommandSummary, SendSummaryHelpMessage};

use crate::module::{Modules, ModulesExt};

pub async fn handle(modules: Arc<Modules>, channel_id: String) -> anyhow::Result<()> {
    modules
        .message_use_case()
        .send_summary_help_message(SendSummaryHelpMessage::new(
            channel_id,
            vec![
                CommandSummary::new(
                    "reg".to_string(),
                    "ユーザーの初期登録を行います".to_string(),
                ),
                CommandSummary::new("start".to_string(), "賭けを開始します".to_string()),
                CommandSummary::new("close".to_string(), "bet を締め切ります".to_string()),
            ],
        ))
        .await?;

    Ok(())
}
