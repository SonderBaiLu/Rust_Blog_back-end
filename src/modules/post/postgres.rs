use crate::error::AppError;
use crate::modules::post::dto::{CreatePostRequest, UpdatePostRequest};
use crate::modules::post::entity::Post;
use crate::modules::post::repository::PostRepository;
use async_trait::async_trait;
use sqlx::{PgPool, Postgres};
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
    //  根据文章id 查询文章
    async fn find_by_id(&self, post: Uuid) -> Result<Option<Post>, AppError>{
        let post = sqlx::query_as::<Postgres, Post>(
            r#" select * from posts where id = $1 and deleted_at is null "#,
        )
            .bind(&post)
            .fetch_optional(&self.pool).await?;
        Ok(post)
    }
    async fn create_post(
        &self,
        author_id: Uuid,
        req: &CreatePostRequest,
    ) -> Result<Post, AppError> {
        let post = sqlx::query_as::<Postgres, Post>(
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
    // 更新文章
    async fn update_post(&self,post:Uuid,author_id: Uuid, req: &UpdatePostRequest) -> Result<Post, AppError> {
        let post = sqlx::query_as::<Postgres, Post>(
            // COALESCE(NULL, title) 会自动回退并保持数据库原有的 title 不变；若传了新值，则更新为新值。
            r#" update posts set
                  title = coalesce($1, title),
                  summary = coalesce($2, summary),
                  content = coalesce($3, content),
                  cover_image = coalesce($4, cover_image),
                  is_published = coalesce($5, is_published),
                  updated_at = now()
                  where id = $6 and author_id = $7 and deleted_at is null
                  returning * "#,
        )
            .bind(&req.title)
            .bind(&req.summary)
            .bind(&req.content)
            .bind(&req.cover_image)
            .bind(&req.is_published)
            .bind(&post)
            .bind(&author_id)
            .fetch_one(&self.pool).await?;
        Ok(post)

    }
}
