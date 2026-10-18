use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use axum::extract::Path;
use uuid::Uuid;
use validator::Validate;

use crate::{
    error::AppError,
    extractors::{AuthUser, OptionalAuthUser},
    modules::post::dto::{CreatePostRequest, UpdatePostRequest},
    state::AppState,
};

/// 创建文章接口：POST /api/v1/posts
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
// 更新文章接口：PATCH /api/v1/posts/{id}
pub async fn update_post(
    State(state): State<AppState>,
    // 获取文章ID，path 获取URL中的id
    Path(post_id): Path<Uuid>,
    // 鉴权提取器
    auth_user: AuthUser,
    Json(req): Json<UpdatePostRequest>,
) -> Result<impl IntoResponse, AppError> {
    // 翻出校验
    req.validate()?;
    // 2. 调用业务服务层
    let resp = state.post_service.update_post(auth_user.id, post_id, &req).await?;
    // 3. 返回 200 OK 与 JSON 载荷
    Ok(Json(resp))
}

// 根据文章ID查询接口：GET /api/v1/posts/{id}
pub async fn get_post_by_id(
    State(state): State<AppState>,
    Path(post_id): Path<Uuid>,
    OptionalAuthUser(auth_user): OptionalAuthUser,
) -> Result<impl IntoResponse, AppError> {
    // 提取当前用户id
    let current_user_id = auth_user.map(|u| u.id);
    let post = state.post_service.get_post_by_id(post_id, current_user_id).await?;
    Ok(Json(post))
}
