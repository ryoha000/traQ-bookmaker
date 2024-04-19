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
                CommandSummary::new("bet".to_string(), "あなたのポイントを賭けます".to_string()),
                CommandSummary::new("cancel".to_string(), "賭けをキャンセルします".to_string()),
                CommandSummary::new(
                    "finish".to_string(),
                    "賭けを終了しポイントを配分します".to_string(),
                ),
            ],
        ))
        .await?;

    Ok(())
}
