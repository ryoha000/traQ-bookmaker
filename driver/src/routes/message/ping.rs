use std::sync::Arc;

use crate::{model::message::ping::PingEvent, module::Modules};

pub async fn handle(_: Arc<Modules>, _: PingEvent) -> anyhow::Result<()> {
    Ok(())
}
