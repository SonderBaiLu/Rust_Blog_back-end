use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    common::Paginated,
    error::AppError,
    extractors::AuthUser,
    modules::todo::dto::{CreateTodoRequest, TodoQuery, TodoResponse, UpdateTodoRequest},
    state::AppState,
};

/// 创建待办：POST /api/v1/todos
pub async fn create_todo(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(req): Json<CreateTodoRequest>,
) -> Result<impl IntoResponse, AppError> {
    req.validate()?;
    let resp = state.todo_service.create(auth_user.id, req).await?;
    Ok((StatusCode::CREATED, Json(resp)))
}

/// 分页列表：GET /api/v1/todos
pub async fn list_todos(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Query(query): Query<TodoQuery>,
) -> Result<Json<Paginated<TodoResponse>>, AppError> {
    let resp = state.todo_service.list(auth_user.id, query).await?;
    Ok(Json(resp))
}

/// 详情：GET /api/v1/todos/:id
pub async fn get_todo(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<TodoResponse>, AppError> {
    let resp = state.todo_service.get_by_id(auth_user.id, id).await?;
    Ok(Json(resp))
}

/// 更新：PATCH /api/v1/todos/:id
pub async fn update_todo(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateTodoRequest>,
) -> Result<Json<TodoResponse>, AppError> {
    req.validate()?;
    let resp = state.todo_service.update(auth_user.id, id, req).await?;
    Ok(Json(resp))
}

/// 删除（软删除）：DELETE /api/v1/todos/:id
pub async fn delete_todo(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    state.todo_service.delete(auth_user.id, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
