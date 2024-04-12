use crate::{module::Modules, routes::message::post_message};
use axum::{routing::post, Router};
use std::sync::Arc;

pub async fn startup(modules: Arc<Modules>) {
    let app = Router::new()
        .route("/message", post(post_message))
        .with_state(modules.clone());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:4351")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .await
        .unwrap_or_else(|_| panic!("Server cannot launch!"));
}

pub fn init_app() {
    tracing_subscriber::fmt::init();
}
