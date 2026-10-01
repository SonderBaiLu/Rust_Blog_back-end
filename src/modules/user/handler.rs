use crate::{
    error::AppError,
    extractors::auth::AuthUser,
    modules::user::dto::{LoginReq, LoginResp, RegisterReq, UpdateUserRequest, UserResp},
    state::AppState,
};
use axum::{Json, extract::State};
use validator::Validate;

// POST /api/users/register
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterReq>,
) -> Result<Json<UserResp>, AppError> {
    // 调用Service 层 注册逻辑
    let user_repo = state.user_service.register(payload).await?;
    Ok(Json(user_repo))
}

// POST /api/users/login
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginReq>,
) -> Result<Json<LoginResp>, AppError> {
    // 触发DTO 格式校验: 如果不合法 ?操作符将其丢个AppError...ValidationError报错
    payload.validate()?;
    // 调用业务层
    let login_resp = state.user_service.login(payload).await?;
    Ok(Json(login_resp))
}
// POST /api/users/get_me
pub async fn get_me(
    State(state): State<AppState>, //  注意：这里直接用 Arc<UserService>，去除 dyn
    authuser: AuthUser,
) -> Result<Json<UserResp>, AppError> {
    // 1. 调用 service 层根据 token 中解析出的 id 查询用户
    let user_resp = state.user_service.get_by_id(authuser.id).await?;

    // 2. 返回 JSON 响应
    Ok(Json(user_resp))
}
// POST /api/users/update_user_info
pub async fn update_user_info(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<UserResp>, AppError> {
    payload.validate()?;
    let user_resp = state
        .user_service
        .update_user_info(auth_user.id, payload)
        .await?;
    Ok(Json(user_resp))
}
