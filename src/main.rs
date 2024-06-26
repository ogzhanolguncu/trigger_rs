mod errors;
mod publish;
mod schedule_tasks;
mod sql;
mod trigger_cron;
mod trigger_delay;
mod trigger_headers;
mod trigger_request;
mod utils;

use crate::publish::publish;

use crate::sql::create_table;

use axum::{
    routing::{get, post},
    Router,
};

use tracing_subscriber;

#[tokio::main]
async fn main() {
    create_table().await;

    tracing_subscriber::fmt()
        .with_ansi(true)
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let app = Router::new()
        .route("/ping", get(hello))
        .route("/publish", post(publish));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn hello() -> String {
    "Pong".to_string()
}
