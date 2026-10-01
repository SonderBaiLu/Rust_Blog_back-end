use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::modules::user::entity::User;

// 用户注册请求
#[derive(Debug, Deserialize, Validate)]
pub struct RegisterReq {
    #[validate(
        length(min = 2, max = 20, message = "用户名长度须在2-20个字符之间"),
        custom(function = "crate::util::validator::validate_username")
    )]
    pub name: String,
    #[validate(length(min = 6, message = "密码长度至少需要 6 位"))]
    pub password: String, //接受前端的明文密码
    #[validate(
        email(message = "邮箱格式不正确"),
        custom(function = "crate::util::validator::validate_email_format")
    )]
    pub email: Option<String>,
}

// 用户登录 请求体
#[derive(Debug, Deserialize, Validate)]
pub struct LoginReq {
    //validator属性宏不支持直接写函数调用语法
    //validate自带的RFC标准的内置邮箱校验规则已经足够完善
    // 所以这里直接使用内置的校验，如果非要自己写函数需要使用"custom" 属性宏
    // custom(function = "crate::util::validator::validate_email_format")
    // 或者联合调用
    // #[validate(length(min = 1), custom(function = "validate_unique_username"))]
    #[validate(email(message = "邮箱格式不正确"))]
    pub email: String,
    #[validate(length(min = 1, message = "密码不能为空"))]
    pub password: String,
    // TODO: 后期添加 邮箱验证码登录 手机号验证码登录
}
// 用户登录 响应体
#[derive(Debug, Serialize)]
pub struct LoginResp {
    pub token: String,
    pub user: UserResp,
}
// 用户信息更新 请求体 TODO: 后续完善用户更新 请求体
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUserRequest {
    #[validate(
        length(min = 2, max = 20, message = "用户名长度须在2-20个字符之间"),
        custom(function = "crate::util::validator::validate_username")
    )]
    pub name: Option<String>,
}
// 用户信息 响应体 TODO: 后续根据user实体类补全用户信息
#[derive(Debug, Serialize)]
pub struct UserResp {
    pub id: Uuid,                  // 用户id 自增
    pub name: String,              // 用户名称
    pub email: String,             // 用户邮箱
    pub email_verified: bool,      // 用户邮箱是否验证 默认为 否
    pub phone: Option<String>,     // 用户手机号
    pub is_active: bool,           // 是否为活跃账号
    pub created_at: DateTime<Utc>, // 注册时间
}
// 信息脱敏
impl From<User> for UserResp {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            name: user.name,
            email: user.email,
            email_verified: user.email_verified,
            phone: user.phone,
            is_active: user.is_active,
            created_at: user.created_at,
        }
    }
}
