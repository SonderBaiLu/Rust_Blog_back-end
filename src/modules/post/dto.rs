use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json;
use uuid::Uuid;
use validator::Validate;

use crate::modules::post::entity::Post;

#[derive(Debug, Deserialize, Validate)]
// 文章请求参数
pub struct CreatePostRequest {
    #[validate(length(min = 1, max = 200))]
    pub title: String,
    #[validate(length(min = 0, max = 500))]
    pub summary: Option<String>,
    pub content: serde_json::Value,
    #[validate(url)]
    pub cover_image: Option<String>,
    pub is_published: bool,
}

// 文章响应参数
#[derive(Debug, Deserialize, Serialize)]
pub struct PostResponse {
    pub id: Uuid,
    pub author_id: Uuid,
    pub title: String,
    pub summary: Option<String>,
    pub content: serde_json::Value,
    pub cover_image: Option<String>,
    pub is_published: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// From<T> 是Rust 标准库提供的值转Trait
// 用于定义类型A 到 类型B 的五岁类型转换
// 1. 为什么需要From<Post> ?
// 在为 PostResponse 实现了 From<Post> 特征后， 任何需要将数据库实体转为
// 前端响应的地方，只需要调用
// let resp = PostResponse::from(post);
// 或者借助 Rust 自动反向炮声的 Into 特性：
// let resp: PostResponse = post.into();
impl From<Post> for PostResponse {
    fn from(post: Post) -> Self {
        Self {
            id: post.id,
            author_id: post.author_id,
            title: post.title,
            summary: post.summary,
            content: post.content,
            cover_image: post.cover_image,
            is_published: post.is_published,
            created_at: post.created_at,
            updated_at: post.updated_at,
        }
    }
}
