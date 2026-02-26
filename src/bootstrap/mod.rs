pub mod exceptions;
use axum::Router;
use tower_http::cors::{Any, CorsLayer};
use crate::core::state::SharedState;
use crate::routes;

pub fn create_app(state: SharedState) -> Router {
    let middleware_stack = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    let app = routes::all_routes()
        .with_state(state)
        .layer(middleware_stack);
    app
}
