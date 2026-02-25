use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LogEntry {
    pub app_id: String,
    pub message: String,
}