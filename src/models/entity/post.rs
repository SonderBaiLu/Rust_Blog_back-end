use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Post {
    pub id: Uuid, // 主键
    pub author_id: Uuid,
    pub title: String,               // 标题
    pub summary: Option<String>,     // 文章摘要
    pub content: serde_json::Value,  // 映射 PostgreSQL 的 JSONB AST 树
    pub cover_image: Option<String>, // 封面图片地址
    pub is_published: bool,          // 是否已发布，默认 false
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PostCollaborator {
    pub id: Uuid,
    pub post_id: Uuid,
    pub user_id: Uuid,
    pub role: String, // 对应 'viewer' | 'editor' | 'admin'
    pub created_at: DateTime<Utc>,
}
