use crate::{handler::post_handler, state::AppState};
use axum::{Router, routing::post};

pub fn post_routes() -> Router<AppState> {
    Router::new().route("/", post(post_handler::create_post))
}
