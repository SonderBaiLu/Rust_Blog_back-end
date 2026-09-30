-- 文章表：存储文章基本信息、内容、发布状态等
create table if not exists posts (
    id uuid primary key default gen_random_uuid(), -- 主键
    author_id uuid not null references users(id) on delete cascade, -- 作者 id，外键引用 users(id)，删除用户时级联删除
    title varchar(200) not null, -- 文章标题
    summary varchar(500), -- 文章摘要
    content jsonb not null, -- 文章内容，jsonb 格式
    cover_image varchar(500), -- 封面图片地址
      boolean not null default false, --
    created_at timestamptz not null default now(), -- 创建时间
    updated_at timestamptz not null default now(), -- 更新时间
    deleted_at timestamptz -- 软删除时间，为空表示未作者 id，外键引用 users(id)，删除用户时级联删除删除
);

-- 按作者 id 查询文章
create index idx_posts_author_id on posts(author_id);
-- 查询未删除的已发布文章（部分索引）
create index idx_posts_published on posts(is_published) where deleted_at is null;
-- 表注释
comment on table posts is '文章表：存储文章基本信息、内容、发布状态等';
-- 列注释
comment on column posts.id is '主键';
comment on column posts.author_id is '作者 id，外键引用 users(id)，删除用户时级联删除';
comment on column posts.title is '文章标题';
comment on column posts.summary is '文章摘要';
comment on column posts.content is '文章内容，jsonb 格式';
comment on column posts.cover_image is '封面图片地址';
comment on column posts.is_published is '是否已发布，默认 false';
comment on column posts.created_at is '创建时间';
comment on column posts.updated_at is '更新时间';
comment on column posts.deleted_at is '软删除时间，为空表示未删除';
-- 索引注释
comment on index idx_posts_author_id is '按作者 id 查询文章的索引';
comment on index idx_posts_published is '查询未删除的已发布文章的部分索引，条件为 deleted_at is null';