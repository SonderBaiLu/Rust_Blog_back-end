-- 用户（users）是主体
create table if not exists users(
  id UUID primary key default gen_random_uuid(),
  name varchar(100) not null,
  avatar_url varchar(500),          
  bio text,                         
  slogan varchar(200),
  gender smallint not null default 0,      -- 0:未知, 1:男, 2:女
  birthday date,                     
  deleted_at timestamptz,            
  password varchar(255) not null,
  email varchar(255) unique,
  email_verified boolean not null default false,
  phone varchar(50),
  phone_verified boolean not null default false,
  two_factor_enabled boolean not null default false,
  two_factor_secret varchar(255),
  last_login_ip varchar(45),
  last_login_at timestamptz,
  is_active boolean not null default true,
  created_at timestamptz not null default current_timestamp,
  updated_at timestamptz not null default current_timestamp
);
comment on column users.id is '用户主键ID，随机UUID';
comment on column users.name is '用户姓名/登录名';
comment on column users.password is '用户密码（应存储哈希值）';
comment on column users.email is '用户邮箱（唯一）';
comment on column users.email_verified is '邮箱是否已验证，true为已验证，false为未验证';
comment on column users.phone is '用户手机号';
comment on column users.phone_verified is '手机号是否已验证，true为已验证，false为未验证';
comment on column users.two_factor_enabled is '是否开启双因素认证（2FA）';
comment on column users.two_factor_secret is '双因素认证的密钥（TOTP算法用）';
comment on column users.last_login_ip is '最后一次登录的IP地址（支持IPv6，最长45字符）';
comment on column users.last_login_at is '最后一次登录的时间（带时区）';
comment on column users.is_active is '用户是否启用，true为启用，false为封禁/停用';
comment on column users.avatar_url is '用户头像图片的CDN/存储URL';
comment on column users.bio is '用户个人简介（较长的自我描述，支持换行）';
comment on column users.slogan is '用户个性签名或短标语（不超过200字符）';
comment on column users.gender is '性别：0未知，1男，2女';
comment on column users.birthday is '用户出生日期（只存年月日）';
comment on column users.deleted_at is '软删除标记，NULL表示正常用户，有值表示注销时间';
comment on column users.created_at is '账号创建时间（带时区）';
comment on column users.updated_at is '记录最后更新时间（带时区，通常由触发器自动更新）';
