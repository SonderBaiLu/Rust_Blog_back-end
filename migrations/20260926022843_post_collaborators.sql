-- 文章协作者表：记录文章与用户之间的协作关系及角色权限
create table if not exists post_collaborators (
    id uuid primary key default gen_random_uuid(), -- 主键
    post_id uuid not null references posts(id) on delete cascade, -- 所属文章 id，删除文章时级联删除
    user_id uuid not null references users(id) on delete cascade, -- 协作者用户 id，删除用户时级联删除
    role varchar(20) not null default 'viewer', -- 协作者角色，默认 viewer，可选 viewer/editor/admin/owner 等
    created_at timestamptz not null default now(), -- 创建时间
    unique (post_id, user_id) -- 同一篇文章中同一用户只能有一条协作记录
);
-- 按文章 id 查询协作者
create index idx_post_collaborators_post_id on post_collaborators(post_id);
-- 按用户 id 查询参与协作的文章
create index idx_post_collaborators_user_id on post_collaborators(user_id);
-- 表注释
comment on table post_collaborators is '文章协作者表：记录文章与用户之间的协作关系及角色权限';
-- 列注释
comment on column post_collaborators.id is '主键';
comment on column post_collaborators.post_id is '文章 id，外键引用 posts(id)，删除文章时级联删除';
comment on column post_collaborators.user_id is '协作者用户 id，外键引用 users(id)，删除用户时级联删除';
comment on column post_collaborators.role is '协作者角色，默认 viewer，可选 viewer/editor/admin/owner 等';
comment on column post_collaborators.created_at is '创建时间';
-- 索引注释
comment on index idx_post_collaborators_post_id is '按文章 id 查询协作者的索引';
comment on index idx_post_collaborators_user_id is '按用户 id 查询协作文章的索引';