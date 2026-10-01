-- 待办任务表：存储待办任务、重复规则、优先级与随机提醒配置
create table if not exists todos (
    id uuid primary key default gen_random_uuid(),
    user_id uuid not null references users(id) on delete cascade,
    title varchar(200) not null,
    description text,
    -- 重复类型：none / daily / weekday / weekend
    repeat_type varchar(20) not null default 'none',
    due_date date,
    is_completed boolean not null default false,
    completed_at timestamptz,
    completed_count int not null default 0,
    priority smallint not null default 0,      -- 0-3
    -- 随机提醒配置
    reminder_enabled boolean not null default false,
    reminder_period varchar(10),               -- day / week / month
    reminder_count int,
    reminder_window_start time,
    reminder_window_end time,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now(),
    deleted_at timestamptz
);

-- 按用户与截止日期查询
create index idx_todos_user_due on todos(user_id, due_date) where deleted_at is null;
-- 按用户与创建时间倒序查询（列表分页）
create index idx_todos_user_created on todos(user_id, created_at desc) where deleted_at is null;

comment on table todos is '待办任务表：存储待办、重复规则、优先级与随机提醒配置';
comment on column todos.repeat_type is '重复类型：none / daily / weekday / weekend';
comment on column todos.priority is '优先级 0-3，默认 0';
comment on column todos.deleted_at is '软删除时间，为空表示未删除';
