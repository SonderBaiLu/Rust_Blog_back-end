use std::sync::Arc;
use tracing::info;
use uuid::Uuid;

use crate::error::AppError;
use crate::modules::post::dto::{CreatePostRequest, PostResponse, UpdatePostRequest};
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
    // 更新文章业务
    #[tracing::instrument(skip(self, req))]
    pub async fn update_post(
        &self,
        author_id: Uuid,
        post: Uuid,
        req: &UpdatePostRequest,
    ) -> Result<PostResponse, AppError> {
        let post = self.post_repo.update_post(post, author_id, req).await?;
        // 日志
        info!(post_id = %post.id, author_id = %author_id, "文章更新成功");
        Ok(PostResponse::from(post))
    }
    // 根据id ID 查询文章业务
    #[tracing::instrument(skip(self))]
    /// current_user_id 是当前用户的ID，用于判断是否有权限查看该文章 未登录时为None
    pub async fn get_post_by_id(
        &self,
        post_id: Uuid,
        current_user_id: Option<Uuid>,
    ) -> Result<PostResponse, AppError> {
        let post = self
            .post_repo
            .find_by_id(post_id)
            .await?
            .ok_or_else(|| AppError::NotFound("文章不存在".to_string()))?;
        // 判断权限
        if !post.is_published {
            let is_author = current_user_id == Some(post.author_id);
            if !is_author {
                // 统一返回 404，防止外部通过 403 探测私密文章的存在性
                return Err(AppError::NotFound("文章未发布".to_string()));
            }
        }
        // 实体转换为DTO 并返回
        Ok(PostResponse::from(post))
    }
}


