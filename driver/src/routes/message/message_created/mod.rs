use std::sync::Arc;

use crate::{
    model::message::message_created::MessageCreatedEvent,
    module::{Modules, ModulesExt},
};

mod help;

pub async fn handle(modules: Arc<Modules>, event: MessageCreatedEvent) -> anyhow::Result<()> {
    let is_mentioned = event
        .message
        .embedded
        .iter()
        .any(|e| e.r#type == "user" && e.id == modules.bot_user_id());
    if !is_mentioned {
        return Ok(());
    }

    let args = event.message.text.split_whitespace().collect::<Vec<_>>();
    if args.is_empty() {
        return Err(anyhow::anyhow!("No command specified"));
    }

    let command_name = args[0];
    match command_name {
        "help" | "--help" | "-h" => help::handle(modules, event.message.channel_id).await?,
        _ => {
            return Err(anyhow::anyhow!("Unknown command: {}", command_name));
        }
    }

    Ok(())
}
