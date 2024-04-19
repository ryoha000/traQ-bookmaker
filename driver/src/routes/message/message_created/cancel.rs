use derive_new::new;
use kernel::model::Id;
use std::sync::Arc;

use crate::module::{Modules, ModulesExt};

#[derive(new)]
pub struct CancelArg {
    pub channel_id: String,
}

pub async fn handle(modules: Arc<Modules>, arg: CancelArg) -> anyhow::Result<()> {
    modules
        .match_use_case()
        .delete_match(Id::new(arg.channel_id))
        .await?;

    Ok(())
}
