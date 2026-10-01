use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    error::AppError,
    modules::todo::{
        dto::{CreateTodoRequest, TodoQuery, UpdateTodoRequest},
        entity::Todo,
    },
};

#[async_trait]
pub trait TodoRepository: Send + Sync {
    async fn create(&self, user_id: Uuid, req: &CreateTodoRequest) -> Result<Todo, AppError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Todo>, AppError>;
    async fn list(&self, user_id: Uuid, query: &TodoQuery) -> Result<Vec<Todo>, AppError>;
    async fn count(&self, user_id: Uuid, query: &TodoQuery) -> Result<u64, AppError>;
    async fn update(&self, id: Uuid, req: &UpdateTodoRequest) -> Result<Todo, AppError>;
    async fn soft_delete(&self, id: Uuid) -> Result<(), AppError>;
}
