use axum::{routing::get, Router};
use crate::core::state::SharedState;

pub fn routes() -> Router<SharedState> {
    // TODO: Implement console routes as system health checks or statistics
    Router::new()
        .route("/console/status", get(|| async { "System OK" }))
}
