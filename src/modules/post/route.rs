use axum::{
    Router,
    routing::{patch, post},
};

use crate::state::AppState;

use super::handler;

pub fn post_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(handler::create_post))
        .route("/{id}", patch(handler::update_post))
}
