use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    error::AppError,
    modules::post::{dto::CreatePostRequest, entity::Post},
};
use crate::modules::post::dto::UpdatePostRequest;

#[async_trait]
pub trait PostRepository: Send + Sync {
    // 创建文章并返回实体
    async fn create_post(&self, author_id: Uuid, req: &CreatePostRequest)
    -> Result<Post, AppError>;
    // 更新文章数据
    async fn update_post(&self, post:Uuid ,author_id: Uuid, req: &UpdatePostRequest)
    -> Result<Post, AppError>;
}
