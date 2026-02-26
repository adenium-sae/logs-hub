use tokio::sync::broadcast;

pub struct BroadcastConfig {
    pub capacity: usize,
}

impl Default for BroadcastConfig {
    fn default() -> Self {
        Self { capacity: 100 }
    }
}
