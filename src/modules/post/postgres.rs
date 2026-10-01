use crate::error::AppError;
use crate::modules::post::dto::CreatePostRequest;
use crate::modules::post::entity::Post;
use crate::modules::post::repository::PostRepository;
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

// 定义持有数据库连接池的结构体并提供 new 构造器
#[derive(Clone)]
pub struct PgPostRepository {
    pool: PgPool,
}

impl PgPostRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PostRepository for PgPostRepository {
    async fn create_post(
        &self,
        author_id: Uuid,
        req: &CreatePostRequest,
    ) -> Result<Post, AppError> {
        let post = sqlx::query_as::<_, Post>(
            r#"
            INSERT INTO posts (
                author_id,
                title,
                summary,
                content,
                cover_image,
                is_published
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#,
        )
            .bind(author_id)
            .bind(&req.title)
            .bind(&req.summary)
            .bind(&req.content)
            .bind(&req.cover_image)
            .bind(req.is_published)
            .fetch_one(&self.pool)
            .await?;

        Ok(post)
    }
}
