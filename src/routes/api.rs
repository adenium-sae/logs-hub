use axum::{routing::{get}, Router};
use crate::core::state::SharedState;

pub fn routes() -> Router<SharedState> {
    Router::new()
        .nest("/api", Router::new()
            .route("/", get(|| async { "Api Endpoint" }))
        )
}
