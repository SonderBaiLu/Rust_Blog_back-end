use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use thiserror::Error;
use tracing::error;
use validator::ValidationErrors;
// 定义通用错误相应体
#[derive(Serialize)]
pub struct ErrorResponse {
    pub code: u16,
    pub error: &'static str,
    pub message: String,
}
#[derive(Debug, Error)]
pub enum AppError {
    //  数据库错误
    //  注：#[from] sqlx::Error 允许从 sqlx::Error 自动转换为 AppError::Database，简化 ? 操作符的使用。
    #[error("数据库错误：{0}")]
    Database(#[from] sqlx::Error),
    // 资源未找到
    #[error("资源不存在：{0}")]
    NotFound(String),
    // 认证失败
    #[error("认证失败: {0}")]
    Unauthorized(String),
    // 权限不足（已登录但无权操作）
    #[error("权限不足: {0}")]
    Forbidden(String),
    // 业务校验失败（如参数非法）
    #[error("请求参数错误: {0}")]
    BadRequest(String),
    // 内部服务器错误（不可预知的异常）
    #[error("服务器内部错误: {0}")]
    Internal(String),
    // 检验错误
    #[error("Validation error: {0}")]
    ValidationError(#[from] ValidationErrors),
    // 其他自定义错误（例如第三方 API 调用失败）
    #[error("{0}")]
    Other(String),
}
// 枚举实现
impl IntoResponse for AppError {
    // 任何实现了这个trait的类型都可以被直接返回为HTTP响应
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Database(e) => {
                error!("数据库查询执行失败：{:?}", e);
                // 对前端隐藏迷茫感信息，只返回通用提示
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "数据库服务异常，请稍后重试".to_string(),
                )
            }
            AppError::NotFound(e) => (StatusCode::NOT_FOUND, e.to_string()),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::Forbidden(msg) => (StatusCode::FORBIDDEN, msg),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::ValidationError(e) => (StatusCode::BAD_REQUEST, e.to_string()),
            AppError::Internal(msg) => {
                error!("内部系统错误: {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "服务器内部错误".to_string(),
                )
            }
            AppError::Other(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };
        // 构建结构化JSON 相应
        let body = Json(ErrorResponse {
            code: status.as_u16(),
            error: status.canonical_reason().unwrap_or("未知的错误"),
            message,
        });
        (status, body).into_response()
    }
}
