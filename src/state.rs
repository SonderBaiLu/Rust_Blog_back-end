use axum::extract::FromRef;
use std::sync::Arc;

use crate::modules::post::PostService;
use crate::modules::todo::TodoService;
use crate::modules::user::UserService;

#[derive(Clone)]
pub struct AppState {
    pub user_service: Arc<UserService>,
    pub post_service: Arc<PostService>,
    pub todo_service: Arc<TodoService>,
    pub jwt_secret: String,
}

impl AppState {
    /// 构造全局应用状态容器
    pub fn new(
        user_service: UserService,
        post_service: PostService,
        todo_service: TodoService,
        jwt_secret: String,
    ) -> Self {
        Self {
            user_service: Arc::new(user_service),
            post_service: Arc::new(post_service),
            todo_service: Arc::new(todo_service),
            jwt_secret,
        }
    }
}

// 供 JWT 鉴权提取器从 AppState 中直接提取密钥
impl FromRef<AppState> for String {
    fn from_ref(state: &AppState) -> Self {
        state.jwt_secret.clone()
    }
}
