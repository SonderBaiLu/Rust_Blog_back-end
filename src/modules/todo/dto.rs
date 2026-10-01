use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::{common::PaginationQuery, modules::todo::entity::Todo};

#[derive(Debug, Deserialize, Validate)]
pub struct CreateTodoRequest {
    #[validate(length(min = 1, max = 200))]
    pub title: String,
    #[validate(length(max = 1000))]
    pub description: Option<String>,
    pub repeat_type: Option<String>, // 缺省 none
    pub due_date: Option<NaiveDate>,
    #[validate(range(min = 0, max = 3))]
    pub priority: Option<i16>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateTodoRequest {
    #[validate(length(min = 1, max = 200))]
    pub title: Option<String>,
    #[validate(length(max = 1000))]
    pub description: Option<String>,
    pub repeat_type: Option<String>,
    pub due_date: Option<NaiveDate>,
    #[validate(range(min = 0, max = 3))]
    pub priority: Option<i16>,
    pub is_completed: Option<bool>,
}

// 列表查询入参：复用通用分页 + 完成状态过滤
#[derive(Debug, Deserialize)]
pub struct TodoQuery {
    #[serde(flatten)]
    pub pagination: PaginationQuery,
    pub is_completed: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct TodoResponse {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub repeat_type: String,
    pub due_date: Option<NaiveDate>,
    pub is_completed: bool,
    pub completed_at: Option<DateTime<Utc>>,
    pub completed_count: i32,
    pub priority: i16,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Todo> for TodoResponse {
    fn from(t: Todo) -> Self {
        Self {
            id: t.id,
            title: t.title,
            description: t.description,
            repeat_type: t.repeat_type,
            due_date: t.due_date,
            is_completed: t.is_completed,
            completed_at: t.completed_at,
            completed_count: t.completed_count,
            priority: t.priority,
            created_at: t.created_at,
            updated_at: t.updated_at,
        }
    }
}
