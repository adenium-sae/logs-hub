mod models;
mod state;
mod handlers;

use axum::{routing::{get, post}, Router};
use std::sync::Arc;
use tokio::sync::broadcast;
use crate::state::{AppState, SharedState};

use tower_http::cors::{Any, CorsLayer};
use axum::http::Method;

#[tokio::main]
async fn main() {
    let (tx, _rx) = broadcast::channel(100);
    let app_state: SharedState = Arc::new(AppState { tx });
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any);
    let app = Router::new()
        .route("/", get(|| async { "Logs Hub Started 🚀" }))
        .route("/log", post(handlers::handle_log))
        .route("/ws", get(handlers::ws_handler))
        .with_state(app_state)
        .layer(cors);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Listening on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}