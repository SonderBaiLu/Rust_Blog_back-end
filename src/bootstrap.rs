use std::sync::Arc;

use sqlx::PgPool;

use crate::{
    modules::{
        post::{postgres::PgPostRepository, PostService},
        todo::{postgres::PgTodoRepository, TodoService},
        user::{postgres::PgUserRepository, UserService},
    },
    state::AppState,
};

/// 应用配置：集中承载运行时依赖的环境变量与密钥。
///
/// 后续接入 MinIO、SMTP、用户默认时区等配置时，在此处扩展字段。
pub struct AppConfig {
    pub jwt_secret: String,
}

/// 集中装配所有依赖链：Repository → Service → AppState。
///
/// `main.rs` 只负责「读环境变量 → 建连接池」，真正的依赖注入收敛在这里，
/// 新增模块时只改这一个函数，避免在 main 里散落 30 行手工装配。
pub fn bootstrap(pool: PgPool, config: AppConfig) -> AppState {
    // 用户体系依赖链
    let user_repo = Arc::new(PgUserRepository::new(pool.clone()));
    let user_service = UserService::new(user_repo, config.jwt_secret.clone());

    // 文章体系依赖链
    let post_repo = Arc::new(PgPostRepository::new(pool.clone()));
    let post_service = PostService::new(post_repo);

    // 待办体系依赖链
    let todo_repo = Arc::new(PgTodoRepository::new(pool.clone()));
    let todo_service = TodoService::new(todo_repo);

    AppState::new(user_service, post_service, todo_service, config.jwt_secret)
}
