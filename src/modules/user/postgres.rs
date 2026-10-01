use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::AppError,
    modules::user::{
        dto::{RegisterReq, UpdateUserRequest},
        entity::User,
        repository::UserRepository,
    },
};

pub struct PgUserRepository {
    pub(crate) pool: PgPool,
}
impl PgUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PgUserRepository {
    // 根据ID 查询用户信息
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, AppError> {
        let user =
            sqlx::query_as::<_, User>("select * from users where id = $1 and deleted_at is null")
                .bind(id)
                .fetch_optional(&self.pool)
                .await?;
        Ok(user)
    }
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as::<_, User>(
            "select * from users where email = $1 and deleted_at is null",
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;
        Ok(user)
    }
    // 创建用户
    async fn create_user(&self, req: &RegisterReq, password_hash: &str) -> Result<User, AppError> {
        let user = sqlx::query_as::<_, User>(
            r#"
        insert into users (name, password, email) values ($1, $2, $3) returning *
        "#,
        )
        .bind(&req.name)
        .bind(password_hash)
        .bind(&req.email)
        .fetch_one(&self.pool)
        .await?;
        Ok(user)
    }
    // 更新用户个人资料并返回更新后的实体
    async fn update_user_info(&self, id: Uuid, req: &UpdateUserRequest) -> Result<User, AppError> {
        // sqlx::query 产出的是未经结构化映射的原始 Row，函数签名要求返回 Result<User, AppError>，必须使用 sqlx::query_as::<_, User>(...) 将行数据自动反序列化为实体
        let userinfo = sqlx::query_as::<_, User>(
            r#"
            update users set name = coalesce($1, name),updated_at = now()
            where id = $2 and deleted_at is null returning *  
            "#,
        )
        .bind(&req.name)
        .bind(id)
        // Query 构建器在 .bind() 之后，需要调用 .fetch_one(&self.pool) 传入连接池执行查询，之后才能挂接 .await?
        .fetch_one(&self.pool)
        .await?;
        Ok(userinfo)
    }
}
