use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    error::AppError,
    modules::post::{dto::{CreatePostRequest}, entity::Post},
};
use crate::modules::post::dto::UpdatePostRequest;

#[async_trait]
pub trait PostRepository: Send + Sync {
    /// 创建文章并返回实体
    /// # Arguments
    /// * `author_id` - 作者的用户id
    /// * `req` - 创建文章请求参数
    /// # Returns
    /// 成功的时候返回包含数据库ID和创建时间的Post 实体
    ///
    /// # Error
    /// 如果作者不存在 数据库连接失败或者参数教研失败，返回 `AppError`
    async fn create_post(&self, author_id: Uuid, req: &CreatePostRequest)
    -> Result<Post, AppError>;
    // 更新文章数据
    async fn update_post(&self, post:Uuid ,author_id: Uuid, req: &UpdatePostRequest)
    -> Result<Post, AppError>;
    /// 根据文章ID 查询详细
    /// # Arguments
    /// * `post` - 文章ID
    /// # Returns
    /// 成功时返回文章本体
    /// # Error
    /// 如果文章不存在 权限不够 则会返回`AppError`
    async fn find_by_id(&self,post: Uuid) -> Result<Option<Post>, AppError>;
    // 分页查询文章 可以选择每页显示  5条10条100条/每页
}
