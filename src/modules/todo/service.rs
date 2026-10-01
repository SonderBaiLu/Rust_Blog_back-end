use std::sync::Arc;
use tracing::info;
use uuid::Uuid;

use crate::{
    common::Paginated,
    error::AppError,
    modules::todo::{
        dto::{CreateTodoRequest, TodoQuery, TodoResponse, UpdateTodoRequest},
        entity::Todo,
        repository::TodoRepository,
    },
};

#[derive(Clone)]
pub struct TodoService {
    todo_repo: Arc<dyn TodoRepository>,
}

impl TodoService {
    pub fn new(todo_repo: Arc<dyn TodoRepository>) -> Self {
        Self { todo_repo }
    }

    #[tracing::instrument(skip(self))]
    pub async fn create(
        &self,
        user_id: Uuid,
        req: CreateTodoRequest,
    ) -> Result<TodoResponse, AppError> {
        let todo = self.todo_repo.create(user_id, &req).await?;
        info!(todo_id = %todo.id, user_id = %user_id, "待办创建成功");
        Ok(TodoResponse::from(todo))
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_by_id(&self, user_id: Uuid, id: Uuid) -> Result<TodoResponse, AppError> {
        let todo = self
            .todo_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound("待办不存在".to_string()))?;
        ensure_owner(&todo, user_id)?;
        Ok(TodoResponse::from(todo))
    }

    #[tracing::instrument(skip(self))]
    pub async fn list(
        &self,
        user_id: Uuid,
        query: TodoQuery,
    ) -> Result<Paginated<TodoResponse>, AppError> {
        let total = self.todo_repo.count(user_id, &query).await?;
        let todos = self.todo_repo.list(user_id, &query).await?;
        let items = todos.into_iter().map(TodoResponse::from).collect();
        Ok(Paginated::new(
            items,
            query.pagination.page(),
            query.pagination.per_page(),
            total,
        ))
    }

    #[tracing::instrument(skip(self))]
    pub async fn update(
        &self,
        user_id: Uuid,
        id: Uuid,
        req: UpdateTodoRequest,
    ) -> Result<TodoResponse, AppError> {
        let existing = self
            .todo_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound("待办不存在".to_string()))?;
        ensure_owner(&existing, user_id)?;

        let todo = self.todo_repo.update(id, &req).await?;
        info!(todo_id = %id, user_id = %user_id, "待办更新成功");
        Ok(TodoResponse::from(todo))
    }

    #[tracing::instrument(skip(self))]
    pub async fn delete(&self, user_id: Uuid, id: Uuid) -> Result<(), AppError> {
        let existing = self
            .todo_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound("待办不存在".to_string()))?;
        ensure_owner(&existing, user_id)?;

        self.todo_repo.soft_delete(id).await?;
        info!(todo_id = %id, user_id = %user_id, "待办已删除");
        Ok(())
    }
}

/// 校验待办归属：仅允许操作本人创建的待办
fn ensure_owner(todo: &Todo, user_id: Uuid) -> Result<(), AppError> {
    if todo.user_id == user_id {
        Ok(())
    } else {
        Err(AppError::Forbidden("无权操作该待办".to_string()))
    }
}
