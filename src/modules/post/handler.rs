use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use validator::Validate;

use crate::{
    error::AppError,
    extractors::AuthUser,
    modules::post::dto::CreatePostRequest,
    state::AppState,
};

/// 创建文章接口：POST /api/posts
pub async fn create_post(
    State(state): State<AppState>,
    // 注入鉴权提取器，获取当前已登录用户的上下文身份
    auth_user: AuthUser,
    // Json 提取器反序列化请求体，必须位于提取器参数列表的最后一位
    Json(req): Json<CreatePostRequest>,
) -> Result<impl IntoResponse, AppError> {
    // 1. Fail-Fast 校验字段（长度、URL 格式、AST 根节点等）
    req.validate()?;

    // 2. 强制使用认证身份中的用户 ID，防范越权伪造
    let current_user_id = auth_user.id;

    // 3. 调用 Service 层执行业务与落库
    let post_resp = state.post_service.create_post(current_user_id, req).await?;

    // 4. 返回 201 CREATED 及序列化后的响应 DTO
    Ok((StatusCode::CREATED, Json(post_resp)))
}
