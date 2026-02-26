pub mod web;
pub mod api;
pub mod console;

use axum::Router;
use crate::core::state::SharedState;

pub fn all_routes() -> Router<SharedState> {
    Router::new()
        .merge(web::routes())
        .merge(api::routes())
        .merge(console::routes())
}
