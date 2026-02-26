use axum::{Router, extract::State, routing::get};
use crate::core::state::SharedState;

pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/info", get(|State(state): State<SharedState>| async move {
            format!("App Name: {}", state.app.name)
        }))
}
