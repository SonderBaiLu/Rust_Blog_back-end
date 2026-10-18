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
// 登录用户身份凭证 在游客的身份下访问
#[derive(Debug,Clone)]
pub struct OptionalAuthUser(pub Option<AuthUser>);

impl<S> FromRequestParts<S> for AuthUser
where
    // 约束：传入的 State 必须能提取出 String (即 jwt_secret)
    String: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
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

// TODO: 删了重写
impl<S> FromRequestParts<S> for OptionalAuthUser
where
    String: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // 检查是否存在 Authorization 请求头
        if parts.headers.get(AUTHORIZATION).is_none() {
            // 未携带请求头：判定为合法游客身份，直接返回 None
            return Ok(OptionalAuthUser(None));
        }
        // 存在请求头：直接委托给 AuthUser 现有的提取逻辑
        // 若非 Bearer 格式或 Token 签名/过期校验失败，此处直接向上冒泡 AppError::Unauthorized
        let auth_user = AuthUser::from_request_parts(parts, state).await?;
        // 3. 校验通过，包装为登录用户身份返回
        Ok(OptionalAuthUser(Some(auth_user)))
    }
}


