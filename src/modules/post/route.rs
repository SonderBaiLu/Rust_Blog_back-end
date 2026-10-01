use axum::{Router, routing::post};

use crate::state::AppState;

use super::handler;

pub fn post_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(handler::create_post))
}
