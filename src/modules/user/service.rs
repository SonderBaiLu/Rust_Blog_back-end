use tracing::{info, warn};
use uuid::Uuid;

use crate::error::AppError;
use crate::modules::user::{
    dto::{LoginReq, LoginResp, RegisterReq, UpdateUserRequest, UserResp},
    repository::UserRepository,
};
use crate::util::jwt::generate_token;
use crate::util::password;
use std::sync::Arc;
pub struct UserService {
    // 依赖抽象接口，而不是具体数据库
    user_repo: Arc<dyn UserRepository>,
    // 签发Token
    jwt_secret: String,
}
impl UserService {
    pub fn new(user_repo: Arc<dyn UserRepository>, jwt_secret: String) -> Self {
        Self {
            user_repo,
            jwt_secret,
        }
    }
    /// 1. 用户注册
    #[tracing::instrument(skip(self, req))]
    pub async fn register(&self, req: RegisterReq) -> Result<UserResp, AppError> {
        // A. 校验邮箱
        if let Some(ref email) = req.email {
            if self.user_repo.find_by_email(email).await?.is_some() {
                warn!(email = %email, "注册失败：该邮箱已被注册");
                return Err(AppError::BadRequest("该邮箱已被注册".to_string()));
            }
        }

        // B. 加密密码 (调用 util)
        let password_hash = password::hash_password(req.password.clone()).await?;

        // C. 写入数据库
        let user = self.user_repo.create_user(&req, &password_hash).await?;

        // D. 记录日志与返回安全 DTO
        info!(user_id = %user.id, name = %user.name, "用户注册成功");
        Ok(UserResp::from(user))
    }

    /// 2. 用户登录
    #[tracing::instrument(skip(self, req))] // 注意：同样 skip(req) 防止泄露密码
    pub async fn login(&self, req: LoginReq) -> Result<LoginResp, AppError> {
        // A. 查询用户
        let user = self
            .user_repo
            .find_by_email(&req.email)
            .await?
            .ok_or_else(|| {
                warn!(email = %req.email, "登录失败：账号不存在");
                AppError::Unauthorized("账号或密码错误".to_string())
            })?;

        // B. 校验状态
        if !user.is_active {
            warn!(user_id = %user.id, email = %req.email, "登录失败：账号已被停用");
            return Err(AppError::Forbidden("账号已被停用".to_string()));
        }

        // C. 校验密码 (拦截错误以打 warn 日志)
        if let Err(err) = password::verify_password(req.password, user.password_hash.clone()).await
        {
            warn!(user_id = %user.id, email = %req.email, "登录失败：密码错误");
            return Err(err);
        }

        let token = generate_token(user.id, &self.jwt_secret, 24)?;

        // D. 登录成功日志
        info!(user_id = %user.id, "用户登录成功");
        Ok(LoginResp {
            token,
            user: UserResp::from(user),
        })
    }

    /// 3. 根据 ID 获取用户信息
    #[tracing::instrument(skip(self))]
    pub async fn get_by_id(&self, user_id: Uuid) -> Result<UserResp, AppError> {
        let user = self.user_repo.find_by_id(user_id).await?.ok_or_else(|| {
            warn!(user_id = %user_id, "获取用户信息失败：用户不存在");
            AppError::NotFound("用户不存在".to_string())
        })?;
        Ok(UserResp::from(user))
    }
    // 用户 信息局部更新
    #[tracing::instrument(skip(self))]
    pub async fn update_user_info(
        &self,
        user_id: Uuid,
        req: UpdateUserRequest,
    ) -> Result<UserResp, AppError> {
        // 调用持久层更新，直接解包得到更新后的 User 实体
        let user = self.user_repo.update_user_info(user_id, &req).await?;
        // 打印业务日志
        info!(user_id = %user_id, "用户个人资料修改成功");
        // 转换为安全脱敏契约并返回 (标准 Ok)
        Ok(UserResp::from(user))
    }
}
