use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::AppError,
    modules::todo::{
        dto::{CreateTodoRequest, TodoQuery, UpdateTodoRequest},
        entity::Todo,
        repository::TodoRepository,
    },
};

#[derive(Clone)]
pub struct PgTodoRepository {
    pool: PgPool,
}

impl PgTodoRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TodoRepository for PgTodoRepository {
    async fn create(&self, user_id: Uuid, req: &CreateTodoRequest) -> Result<Todo, AppError> {
        // 可空且有默认值的字段，缺省在 Rust 侧补齐，保持 SQL 简洁
        let repeat_type = req.repeat_type.as_deref().unwrap_or("none");
        let priority = req.priority.unwrap_or(0);

        let todo = sqlx::query_as::<_, Todo>(
            r#"
            INSERT INTO todos (user_id, title, description, repeat_type, due_date, priority)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#,
        )
        .bind(user_id)
        .bind(&req.title)
        .bind(&req.description)
        .bind(repeat_type)
        .bind(req.due_date)
        .bind(priority)
        .fetch_one(&self.pool)
        .await?;

        Ok(todo)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Todo>, AppError> {
        let todo =
            sqlx::query_as::<_, Todo>("SELECT * FROM todos WHERE id = $1 AND deleted_at IS NULL")
                .bind(id)
                .fetch_optional(&self.pool)
                .await?;
        Ok(todo)
    }

    async fn list(&self, user_id: Uuid, query: &TodoQuery) -> Result<Vec<Todo>, AppError> {
        let todos = sqlx::query_as::<_, Todo>(
            r#"
            SELECT * FROM todos
            WHERE user_id = $1 AND deleted_at IS NULL
              AND ($2::bool IS NULL OR is_completed = $2)
            ORDER BY created_at DESC
            LIMIT $3 OFFSET $4
            "#,
        )
        .bind(user_id)
        .bind(query.is_completed)
        .bind(query.pagination.limit())
        .bind(query.pagination.offset())
        .fetch_all(&self.pool)
        .await?;

        Ok(todos)
    }

    async fn count(&self, user_id: Uuid, query: &TodoQuery) -> Result<u64, AppError> {
        let total: i64 = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*) FROM todos
            WHERE user_id = $1 AND deleted_at IS NULL
              AND ($2::bool IS NULL OR is_completed = $2)
            "#,
        )
        .bind(user_id)
        .bind(query.is_completed)
        .fetch_one(&self.pool)
        .await?;

        Ok(total as u64)
    }

    async fn update(&self, id: Uuid, req: &UpdateTodoRequest) -> Result<Todo, AppError> {
        let todo = sqlx::query_as::<_, Todo>(
            r#"
            UPDATE todos SET
                title        = COALESCE($1, title),
                description  = COALESCE($2, description),
                repeat_type  = COALESCE($3, repeat_type),
                due_date     = COALESCE($4, due_date),
                priority     = COALESCE($5, priority),
                is_completed = COALESCE($6, is_completed),
                updated_at   = now()
            WHERE id = $7 AND deleted_at IS NULL
            RETURNING *
            "#,
        )
        .bind(&req.title)
        .bind(&req.description)
        .bind(&req.repeat_type)
        .bind(req.due_date)
        .bind(req.priority)
        .bind(req.is_completed)
        .bind(id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| AppError::NotFound("待办不存在".to_string()))?;

        Ok(todo)
    }

    async fn soft_delete(&self, id: Uuid) -> Result<(), AppError> {
        let res = sqlx::query(
            "UPDATE todos SET deleted_at = now() WHERE id = $1 AND deleted_at IS NULL",
        )
        .bind(id)
        .execute(&self.pool)
        .await?;

        if res.rows_affected() == 0 {
            return Err(AppError::NotFound("待办不存在".to_string()));
        }

        Ok(())
    }
}
