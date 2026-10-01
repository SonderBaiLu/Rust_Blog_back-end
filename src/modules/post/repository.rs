use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    error::AppError,
    modules::post::{dto::CreatePostRequest, entity::Post},
};

#[async_trait]
pub trait PostRepository: Send + Sync {
    // 创建文章并返回实体
    async fn create_post(&self, author_id: Uuid, req: &CreatePostRequest)
    -> Result<Post, AppError>;
}
