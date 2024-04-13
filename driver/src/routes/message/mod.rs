use std::sync::Arc;

use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
};
use futures::Future;
use tracing::{info, warn};

use crate::module::Modules;

mod message_created;
mod ping;

pub async fn post_message(
    State(modules): State<Arc<Modules>>,
    headers: HeaderMap,
    body: String,
) -> StatusCode {
    let event_type = headers
        .get("X-TRAQ-BOT-EVENT")
        .map(|v| v.to_str().unwrap_or_default())
        .unwrap_or_default();
    info!("Received message: {}", body);
    match handle_event(modules, event_type, body).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(e) => {
            warn!("Failed to handle event: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

async fn handle_event(modules: Arc<Modules>, event_type: &str, body: String) -> anyhow::Result<()> {
    match event_type {
        "PING" => parse_and_exec(modules, &body, ping::handle).await?,
        "MESSAGE_CREATED" => parse_and_exec(modules, &body, message_created::handle).await?,
        _ => {
            return Err(anyhow::anyhow!(
                "Unknown event type: {}, body: {}",
                event_type,
                body
            ))
        }
    }
    Ok(())
}

async fn parse_and_exec<'a, T, S, F>(
    modules: Arc<Modules>,
    body: &'a str,
    func: S,
) -> anyhow::Result<()>
where
    T: serde::Deserialize<'a>,
    S: Fn(Arc<Modules>, T) -> F,
    F: Future<Output = anyhow::Result<()>>,
{
    let event = serde_json::from_str::<T>(body)?;
    func(modules, event).await
}
