use axum::{
    extract::{FromRef, FromRequestParts},
    http::{header::AUTHORIZATION, request::Parts},
};
use uuid::Uuid;

use crate::{error::AppError, util::jwt::verify_token};

// 登录用户身份凭证 (已认证用户的信息)
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub id: Uuid,
}

impl<S> FromRequestParts<S> for AuthUser
where
    // 约束：传入的 State 必须能提取出 String (即 jwt_secret)
    String: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // 1. 从 请求头 中获取 Authorization 字段，使用 ? 运算符处理错误
        let auth_header = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .ok_or_else(|| AppError::Unauthorized("缺少 Authorization 请求头".to_string()))?; // 加上了 ?

        // 2. 检查是否为 Bearer 前缀
        if !auth_header.starts_with("Bearer ") {
            return Err(AppError::Unauthorized(
                "Authorization 请求头格式错误，必须为 Bearer <token>".to_string(),
            ));
        }

        // 3. 截取 Token 字符串
        let token = &auth_header[7..];

        // 4. 从 State 中提取 jwt_secret
        let secret = String::from_ref(state);

        // 5. 校验 Token
        let claims = verify_token(token, &secret)?;

        // 6. 解析 claims.sub 中的字符串 UUID 为 Uuid 类型
        let user_id = Uuid::parse_str(&claims.sub)
            .map_err(|_| AppError::Unauthorized("无效的用户凭证标识".to_string()))?;

        Ok(AuthUser { id: user_id })
    }
}
