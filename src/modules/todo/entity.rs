use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Todo {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub repeat_type: String,           // none / daily / weekday / weekend
    pub due_date: Option<NaiveDate>,
    pub is_completed: bool,
    pub completed_at: Option<DateTime<Utc>>,
    pub completed_count: i32,
    pub priority: i16,                 // 0-3
    pub reminder_enabled: bool,
    pub reminder_period: Option<String>,       // day / week / month
    pub reminder_count: Option<i32>,
    pub reminder_window_start: Option<NaiveTime>,
    pub reminder_window_end: Option<NaiveTime>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}
