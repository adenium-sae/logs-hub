use std::sync::Arc;
use tokio::sync::broadcast;
use crate::core::state::{AppState, SharedState};
use crate::core::configs::app::AppConfig;
use crate::core::configs::broadcast::BroadcastConfig;

pub struct AppServiceProvider;

impl AppServiceProvider {
    pub fn boot() -> SharedState {
        let app = AppConfig::load(); 
        let broadcast_settings = BroadcastConfig::default();
        let (messenger, _) = broadcast::channel(broadcast_settings.capacity);
        Arc::new(AppState {
            app,
            broadcast_settings,
            messenger,
        })
    }
}