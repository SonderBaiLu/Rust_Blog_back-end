use chrono::{DateTime, NaiveDate, Utc};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub avatar_url: Option<String>,        // 头像
    pub bio: Option<String>,               // 用户个人简介
    pub slogan: Option<String>,            // 用户个性签名或短标语
    pub gender: i16,                       // 0:未知, 1:男, 2:女
    pub birthday: Option<NaiveDate>,       // 用户出生日期
    pub deleted_at: Option<DateTime<Utc>>, // 软删除标记
    #[sqlx(rename = "password")]
    pub password_hash: String, // 存储加密后的密码哈希
    pub email: String,                     // 邮箱地址（唯一，不可为 NULL）
    pub email_verified: bool,              // 邮箱是否已验证
    pub phone: Option<String>,             // 手机号码（国际格式，可选）
    pub phone_verified: bool,              // 手机号是否已验证
    pub two_factor_enabled: bool,          // 是否启用二次验证
    pub two_factor_secret: Option<String>, // TOTP 密钥（启用二次验证时设置）
    pub last_login_ip: Option<String>,     // 最近登录 IP 地址（存储字符串形式）
    pub last_login_at: Option<DateTime<Utc>>, // 最近登录时间
    pub is_active: bool,                   // 账户是否激活/未禁用
    pub created_at: DateTime<Utc>,         // 创建时间
    pub updated_at: DateTime<Utc>,         // 更新时间
}
