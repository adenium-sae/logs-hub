mod app;
mod core;
mod bootstrap;
mod routes;
use std::sync::Arc;
use tokio::sync::broadcast;
use crate::core::state::AppState;
use crate::core::configs::app::AppConfig;
use crate::core::configs::broadcast::BroadcastConfig;

#[tokio::main]
async fn main() {
    let app_config = AppConfig::default();
    let broadcast_settings = BroadcastConfig::default();
    let (tx, _rx) = broadcast::channel(broadcast_settings.capacity);
    let app_state = Arc::new(AppState {
        app: app_config,
        broadcast_settings,
        messenger: tx,
    });
    let app = bootstrap::create_app(app_state.clone());
    let addr = format!("127.0.0.1:{}", app_state.app.port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("✅ {} started on http://{}", app_state.app.name, addr);
    axum::serve(listener, app).await.unwrap();
}
