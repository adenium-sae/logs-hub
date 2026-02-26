use axum::{routing::get, Router};
use crate::core::state::SharedState;

pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/api", get(|| async { "Api Endpoint" }))
}
