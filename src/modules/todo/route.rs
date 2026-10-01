use axum::{
    Router,
    routing::{delete, get, patch, post},
};

use crate::state::AppState;

use super::handler;

pub fn todo_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(handler::create_todo).get(handler::list_todos))
        .route(
            "/{id}",
            get(handler::get_todo)
                .patch(handler::update_todo)
                .delete(handler::delete_todo),
        )
}
