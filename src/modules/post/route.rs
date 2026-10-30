use axum::{
    Router,
    routing::{get, post},
};

use crate::state::AppState;
use super::handler;

pub fn post_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(handler::create_post))
        .route(
            "/{id}",
            get(handler::get_post_by_id).patch(handler::update_post),
        )
}
