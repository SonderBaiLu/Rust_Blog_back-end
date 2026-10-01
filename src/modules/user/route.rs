use axum::{
    Router,
    routing::{get, post},
};

use crate::state::AppState;

use super::handler;

/// 暴露用户模块的所有路由映射
pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(handler::register))
        .route("/login", post(handler::login))
        .route(
            "/me",
            get(handler::get_me).patch(handler::update_user_info),
        )
}
