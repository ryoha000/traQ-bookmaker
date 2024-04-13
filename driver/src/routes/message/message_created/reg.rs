use derive_new::new;
use std::sync::Arc;

use app::model::{message::SendMessage, user::CreateUser};

use crate::module::{Modules, ModulesExt};

#[derive(new)]
pub struct RegArg {
    pub traq_id: String,
    pub traq_display_id: String,
    pub channel_id: String,
}

pub async fn handle(modules: Arc<Modules>, arg: RegArg) -> anyhow::Result<()> {
    let channel_id = arg.channel_id.clone();
    let traq_id = arg.traq_id.clone();
    let user = modules
        .user_use_case()
        .register_user(CreateUser::new(
            arg.traq_id,
            arg.traq_display_id,
            arg.channel_id,
        ))
        .await?;

    modules
        .message_use_case()
        .send_message(SendMessage::new(
            channel_id,
            format!(
                "@{} の登録を完了しました。初期ポイントは{}ポイントです。",
                traq_id, user.balance
            ),
            true,
        ))
        .await?;

    Ok(())
}
