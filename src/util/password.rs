use crate::error::AppError;
use argon2::{
    Argon2,
    password_hash::{PasswordHasher, PasswordVerifier},
};

/// 对明文密码进行 Argon2 加密
pub async fn hash_password(password: String) -> Result<String, AppError> {
    tokio::task::spawn_blocking(move || {
        let argon2 = Argon2::default();
        argon2
            .hash_password(password.as_bytes()) // 0.6.0: 自动生成盐
            .map_err(|e| AppError::Internal(format!("密码加密失败：{}", e)))
            .map(|hash| hash.to_string())
    })
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?
}

/// 验证明文密码与哈希值是否匹配
pub async fn verify_password(password: String, password_hash: String) -> Result<(), AppError> {
    tokio::task::spawn_blocking(move || {
        let argon2 = Argon2::default();
        argon2
            .verify_password(password.as_bytes(), password_hash.as_str())
            .map_err(|_| AppError::Unauthorized("账号或密码错误".to_string()))
    })
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?
}
