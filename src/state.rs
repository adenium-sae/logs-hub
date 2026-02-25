use std::sync::Arc;
use tokio::sync::broadcast;
use crate::models::LogEntry;

pub struct AppState {
    pub tx: broadcast::Sender<LogEntry>,
}

pub type SharedState = Arc<AppState>;