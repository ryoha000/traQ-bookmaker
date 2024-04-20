use derive_new::new;
use std::sync::Arc;

use crate::module::{Modules, ModulesExt};

#[derive(new)]
pub struct InfoArg {
    pub channel_id: String,
}

pub async fn handle(modules: Arc<Modules>, arg: InfoArg) -> anyhow::Result<()> {
    modules.user_use_case().list_users(arg.channel_id).await?;

    Ok(())
}
