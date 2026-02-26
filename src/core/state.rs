use std::sync::Arc;
use tokio::sync::broadcast;
use crate::core::configs::{app::AppConfig, broadcast::BroadcastConfig};

pub struct AppState {
    pub app: AppConfig,
    pub broadcast_settings: BroadcastConfig,
    pub messenger: broadcast::Sender<String>,
}

pub type SharedState = Arc<AppState>;
