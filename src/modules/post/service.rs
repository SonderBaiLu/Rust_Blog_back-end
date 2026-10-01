use std::sync::Arc;
use tracing::info;
use uuid::Uuid;

use crate::error::AppError;
use crate::modules::post::dto::{CreatePostRequest, PostResponse};
use crate::modules::post::repository::PostRepository;

#[derive(Clone)]
pub struct PostService {
    post_repo: Arc<dyn PostRepository>,
}

impl PostService {
    /// 构造函数，注入持久层抽象特征对象
    pub fn new(post_repo: Arc<dyn PostRepository>) -> Self {
        Self { post_repo }
    }

    /// 创建文章业务流水线
    #[tracing::instrument(skip(self))]
    pub async fn create_post(
        &self,
        author_id: Uuid,
        req: CreatePostRequest,
    ) -> Result<PostResponse, AppError> {
        // 1. 调用持久层原子化落库，获取完整的 Post 实体
        let post = self.post_repo.create_post(author_id, &req).await?;

        // 2. 记录业务审计链路追踪日志
        info!(
            post_id = %post.id,
            author_id = %author_id,
            "文章创建成功"
        );

        // 3. 实体转安全响应 DTO 并返回
        Ok(PostResponse::from(post))
    }
}
