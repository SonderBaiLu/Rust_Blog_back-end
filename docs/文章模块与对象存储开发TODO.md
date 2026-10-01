# Rust Blog 个人博客平台 — 全功能开发 TODO

> 本文档是「文章核心管理」「对象存储媒体直传」两大既有模块，以及「音乐播放器 / Todo 清单 / 语录集 / 阅读清单 / Anki / 留言墙 / 足迹 / 项目页 / 习惯打卡 / 目标管理 / 番茄钟 / 统计面板 / 数据导出 / 联系页 / 个人资料 / 安全设置」等后续功能模块的逐项开发清单。
> 所有命名、路径、错误类型、分层约定均对齐当前代码库。

## 目录

- [第 0 章 基础设施与跨模块依赖](#第-0-章-基础设施与跨模块依赖)
- [第 1 章 文章读写与权限边界管控](#第-1-章-文章读写与权限边界管控)
- [第 2 章 对象存储媒体直传](#第-2-章-对象存储媒体直传)
- [第 3 章 个人内容模块](#第-3-章-个人内容模块)
- [第 4 章 生产力工具模块](#第-4-章-生产力工具模块)
- [第 5 章 展示与互动模块](#第-5-章-展示与互动模块)
- [第 6 章 个人中心与安全](#第-6-章-个人中心与安全)
- [第 7 章 测试与验收](#第-7-章-测试与验收)

## 代码库约定速查

| 分层 | 文件路径 | 说明 |
| --- | --- | --- |
| 鉴权提取器 | `src/extractors/auth.rs` | 已存在 `AuthUser`，需新增 `OptionalAuthUser` |
| 持久层契约 | `src/repository/post.rs` | `PostRepository` trait |
| Pg 实现 | `src/repository/postgres/post_repo.rs` | `PgPostRepository` |
| 实体 | `src/models/entity/post.rs` | `Post`、`PostCollaborator` |
| 请求 / 响应 DTO | `src/models/dto/post.rs` | `CreatePostRequest`、`PostResponse` |
| 统一错误 | `src/error.rs` | `AppError` 枚举 |
| 业务层 | `src/service/post_service.rs` | `PostService` |
| 处理器 | `src/handler/post_handler.rs` | 各端点处理器 |
| 路由 | `src/route/post_route.rs` | 统一前缀 `/api/v1/posts` |
| 全局状态 | `src/state.rs` | `AppState` |

### 通用分层约定（新增模块一律遵循）

每个新功能模块按固定五层拆分，命名与路径对齐现有代码：

| 层 | 命名 | 示例路径 |
| --- | --- | --- |
| 实体 | `Xxx` | `src/models/entity/xxx.rs` |
| DTO | `CreateXxxRequest` / `XxxResponse` / `XxxQuery` | `src/models/dto/xxx.rs` |
| 契约 | `XxxRepository` trait | `src/repository/xxx.rs` |
| 实现 | `PgXxxRepository` | `src/repository/postgres/xxx_repo.rs` |
| 业务 | `XxxService` | `src/service/xxx_service.rs` |
| 处理器 | `xxx_handler.rs` | `src/handler/xxx_handler.rs` |
| 路由 | `xxx_route.rs` | `src/route/xxx_route.rs` |

---

## 第 0 章 基础设施与跨模块依赖

> Todo 随机提醒、习惯打卡、目标到期、番茄钟提醒等功能都依赖「定时任务调度 + 通知 + 时区」。本章先搭好这些地基，避免各模块重复造轮子。

### 0.1 数据库迁移基础设施

- [ ] 统一迁移命名规范：`YYYYMMDDHHMMSS_<snake_case>.sql`（与现有 `sqlx` 迁移一致）。
- [ ] 所有新表遵循软删除约定：`deleted_at timestamptz`，查询时统一 `AND deleted_at IS NULL`。
- [ ] 所有外键明确 `ON DELETE` 行为（`CASCADE` / `SET NULL` / `RESTRICT`）。
- [ ] 统一时间字段：`created_at` / `updated_at` 均 `timestamptz not null default now()`。

### 0.2 定时任务调度器

- [ ] 引入调度依赖（二选一并记录）：
  - [ ] `tokio-cron-scheduler`：支持 cron 表达式，可动态增删任务，推荐。
  - [ ] 自研简单调度：基于 `tokio::time` + 数据库轮询，可控性高但代码量大。
- [ ] 新增 `SchedulerService`（`src/service/scheduler_service.rs`）：
  - [ ] 启动时从数据库加载「到期时间在未来的提醒/打卡」注册定时任务。
  - [ ] 提供 `schedule_at(task_id, at: DateTime<Utc>, payload)` 动态注册单次任务。
  - [ ] 提供 `cancel(task_id)` 撤销任务。
  - [ ] 任务触发后调用 `NotificationService` 推送，并从待办队列移除。
- [ ] 兜底机制：定时任务可能因进程重启丢失，增加「每 1 分钟扫描一次过期未推送的提醒」补偿任务，保证不漏。

### 0.3 通知服务

- [ ] 新增 `notifications` 表（站内通知，统一承载 Todo / 习惯 / 目标 / 番茄钟提醒）：

```sql
create table if not exists notifications (
    id uuid primary key default gen_random_uuid(),
    user_id uuid not null references users(id) on delete cascade,
    type varchar(30) not null,        -- todo_reminder / habit_check / goal_deadline / pomodoro / system
    title varchar(200) not null,
    content text,
    target_type varchar(30),          -- 关联模块，如 todo / habit
    target_id uuid,                   -- 关联资源 id
    is_read boolean not null default false,
    created_at timestamptz not null default now()
);
create index idx_notifications_user on notifications(user_id, created_at desc);
```

- [ ] 新增 `NotificationService`：
  - [ ] `push(user_id, type, title, content, target)` 写入站内通知。
  - [ ] `list(user_id, page, per_page)` 分页查询。
  - [ ] `mark_read(id)` / `mark_all_read(user_id)`。
  - [ ] `unread_count(user_id)` 返回未读数（供前端角标）。
- [ ] 扩展渠道（进阶，可选）：邮件 / WebPush，通过 `NotificationChannel` trait 抽象，首版仅站内。

### 0.4 用户时区支持

- [ ] `users` 表新增 `timezone varchar(50) default 'Asia/Shanghai'`。
- [ ] 新增 `TimezoneHelper`：将用户本地时间（如「工作日上午 9 点」）转换为 `DateTime<Utc>` 落库，所有「按天/周/月」的随机时间与重复规则都基于用户时区计算。
- [ ] 引入时区库（`chrono-tz` 或 `jiff`），处理夏令时与工作日判定。

### 0.5 统一分页与通用响应

- [ ] 新增 `Paginated<T>` 通用分页结构体（`src/models/dto/mod.rs`）：

```rust
#[derive(Debug, Serialize)]
pub struct Paginated<T> {
    pub items: Vec<T>,
    pub page: u32,
    pub per_page: u32,
    pub total: u64,
    pub total_pages: u32,
}
```

- [ ] 新增通用 `PaginationQuery`（`page`、`per_page`，含默认值与边界钳制）。
- [ ] 各列表接口统一返回 `Paginated<T>`，避免每个模块重复定义。

### 0.6 离线优先与多端同步（对接 App / 桌面端储备）

> 未来对接移动端 / 桌面端时，客户端在离线状态下先写入本地 SQLite，联网后再上传。本章是后端必须提前长出的配套能力，与「前端做 UI」分工互补。

#### 0.6.1 目标与边界

- [ ] 明确分工：客户端 UI 由独立前端项目负责（移动端 Flutter / RN，桌面端 Tauri / Electron），本仓库只负责 API 与同步协议。
- [ ] 后端补齐离线同步所需的三大能力：幂等写入、增量拉取、软删除墓碑。

#### 0.6.2 后端接口约定

- [ ] **客户端生成主键**：所有同步实体主键由客户端生成 UUID，服务器不生成，保证离线创建也能确定 id。
- [ ] **幂等 upsert**：`POST /api/v1/sync/upsert`，按 `id` 存在则覆盖、不存在则插入。
- [ ] **增量拉取**：`GET /api/v1/sync/changes?since=<updated_at>`，只返回该时间点之后的变化。
- [ ] **软删除墓碑**：删除动作统一打 `deleted_at`，不物理删除，确保删除能被同步到其他端。
- [ ] 每条同步记录必须携带 `updated_at`（微秒精度），作为版本与冲突裁决依据。

#### 0.6.3 数据模型约定

- [ ] 所有需同步的表强制包含三字段：`id uuid`（客户端生成）、`updated_at timestamptz`、`deleted_at timestamptz`。
- [ ] 补充迁移：核对现有 `todos` / `habits` / `cards` 等表是否具备上述三字段，缺失则 `ALTER TABLE` 补列。
- [ ] `updated_at` 采用 `timestamptz` 且写入 `now()` 保证微秒精度，避免并发写入时精度丢失导致冲突裁决错误。

#### 0.6.4 客户端本地存储（outbox 发件箱模式）

- [ ] 客户端本地 SQLite 分两类表，事务内同时写入：

```sql
-- 数据缓存表：长期保留，供离线阅读 / 秒开
todo(id TEXT PRIMARY KEY, title TEXT, ..., dirty INTEGER DEFAULT 0)

-- 待同步队列：上传成功后删除这一行
outbox(id TEXT PRIMARY KEY, entity TEXT, op TEXT, payload TEXT, created_at ...)
```

- [ ] 写入流程：离线修改 → 同一事务内写「数据缓存表 + outbox」→ 后台同步器取 outbox 上传 → 收到服务器 ACK 后只删 outbox 行。
- [ ] **清理原则**：outbox 队列上传成功即清，数据缓存保留不清。缓存仅在存储吃紧、隐私要求、用户手动清除时删除。

#### 0.6.5 冲突策略

- [ ] 单人博客采用「最后写入者胜（last-write-wins）」，以 `updated_at` 较大者为准，不上 CRDT 等重方案。
- [ ] 服务器 upsert 时若本地版本旧于服务器版本，返回冲突提示而非静默覆盖（可选，进阶）。

#### 0.6.6 后端落地清单

- [ ] 新增 `SyncService` 或各模块自带 upsert / changes 接口，二选一并记录。
- [ ] 增量拉取接口按模块提供 `since` 过滤（`WHERE updated_at > $1`）。
- [ ] 为 `updated_at` 建立索引，支撑增量扫描。
- [ ] 记录同步审计日志（设备标识、同步条数、时间）。

---

## 第 1 章 文章读写与权限边界管控

### 1.1 文章详情查询与可见性判定

#### 1.1.1 提取器扩展（基础设施层）

- [ ] 在 `src/extractors/auth.rs` 中新增 `OptionalAuthUser` 提取器：

```rust
#[derive(Debug, Clone)]
pub struct OptionalAuthUser(pub Option<AuthUser>);
```

- [ ] 实现 `FromRequestParts<S>`，三种情形：
  - [ ] 未携带 `Authorization` 请求头：返回 `Ok(OptionalAuthUser(None))`，不报错。
  - [ ] 携带 `Authorization` 但非 `Bearer ` 前缀或 Token 校验失败：返回 `AppError::Unauthorized`。
  - [ ] 携带合法 Token：解析出 `Some(AuthUser)`。
- [ ] 在 `src/extractors/mod.rs` 中导出：`pub use auth::{AuthUser, OptionalAuthUser};`
- [ ] 编写单元测试覆盖三种情形（无 Header / 非法 Token / 合法 Token）。

#### 1.1.2 持久层契约与 SQL（Repository 层）

- [ ] 在 `src/repository/post.rs` 的 `PostRepository` trait 中新增契约：

```rust
async fn find_by_id(&self, id: Uuid) -> Result<Option<Post>, AppError>;
```

- [ ] 在 `src/repository/postgres/post_repo.rs` 中实现，使用 `sqlx::query_as::<_, Post>` + `.fetch_optional`：

```sql
SELECT * FROM posts WHERE id = $1 AND deleted_at IS NULL
```

- [ ] `None` 表示不存在或已软删除。

#### 1.1.3 权限矩阵判定（Service 层）

- [ ] 在 `src/service/post_service.rs` 新增：

```rust
pub async fn get_post_by_id(
    &self,
    viewer: Option<AuthUser>,
    id: Uuid,
) -> Result<PostResponse, AppError>
```

- [ ] 实现可见性决策树（顺序判断，命中即返回）：
  - [ ] 先 `find_by_id(id)`，`None` 返回 `AppError::NotFound("文章不存在")`。
  - [ ] `is_published == true`：直接允许读取。
  - [ ] `is_published == false` 且 `viewer == None`：统一返回 `NotFound`，规避信息泄露。
  - [ ] `is_published == false` 且 `viewer.id == author_id`（作者本人）：允许读取。
  - [ ] 进阶：`viewer.id` 命中 `post_collaborators` 且 `role` 有效：允许读取。
  - [ ] 其余情况：返回 `NotFound`。
- [ ] 用 `info!` 记录「读取通过 / 被拒」链路日志（含 `post_id`、`viewer_id`、`reason`）。
- [ ] 为决策树编写单元测试（公开 / 私密匿名 / 私密作者 / 私密协作者 / 私密非协作者）。

#### 1.1.4 端点暴露（Handler & Route 层）

- [ ] 在 `src/handler/post_handler.rs` 新增 `get_post_by_id`：
  - [ ] 提取 `Path(id): Path<Uuid>`、`State`、`optional_user: OptionalAuthUser`。
  - [ ] 调用 `state.post_service.get_post_by_id(optional_user.0, id)`。
- [ ] 在 `src/route/post_route.rs` 挂载：

```rust
.route("/{id}", get(post_handler::get_post_by_id))
```

- [ ] 确认最终路由为 `GET /api/v1/posts/:id`。

---

### 1.2 文章编辑、正文保存与状态流转

#### 1.2.1 输入契约设计（DTO 层）

- [ ] 在 `src/models/dto/post.rs` 新增 `UpdatePostRequest`，全部字段 `Option`，仅更新非空项：

```rust
#[derive(Debug, Deserialize, Validate)]
pub struct UpdatePostRequest {
    #[validate(length(min = 1, max = 200))]
    pub title: Option<String>,
    #[validate(length(max = 500))]
    pub summary: Option<String>,
    pub content: Option<serde_json::Value>, // 富文本 AST，增量保存
    #[validate(url)]
    pub cover_image: Option<String>,
    pub is_published: Option<bool>, // 草稿 / 公开发布切换
}
```

- [ ] 明确「全字段均为 `None`」的处理策略（二选一并记录）：
  - [ ] 方案 A：视为 `BadRequest("无可更新字段")`，推荐。
  - [ ] 方案 B：直接返回当前文章（幂等）。
- [ ] 明确 `content` 为 `JSONB` 的「整体替换」语义：增量富文本由前端组装 AST 后整体 PUT，服务端不做 diff 合并。

#### 1.2.2 原子化局部更新（Repository 层）

- [ ] 在 `PostRepository` trait 中新增契约：

```rust
async fn update_post(&self, id: Uuid, req: &UpdatePostRequest) -> Result<Post, AppError>;
```

- [ ] 在 `PgPostRepository` 中用 `COALESCE` 动态 SQL 实现：

```sql
UPDATE posts
SET
    title        = COALESCE($1, title),
    summary      = COALESCE($2, summary),
    content      = COALESCE($3, content),
    cover_image  = COALESCE($4, cover_image),
    is_published = COALESCE($5, is_published),
    updated_at   = NOW()
WHERE id = $6 AND deleted_at IS NULL
RETURNING *
```

- [ ] 参数绑定说明：
  - [ ] `title` / `summary` / `cover_image`：绑定 `Option<String>`。
  - [ ] `content`：绑定 `Option<serde_json::Value>`，映射 JSONB。
  - [ ] `is_published`：绑定 `Option<bool>`。
  - [ ] `id`：绑定 `Uuid`。
- [ ] 使用 `sqlx::query_as::<_, Post>` + `.fetch_one`，行不存在时映射为 `NotFound`。
- [ ] 确认 `Option<serde_json::Value>` 的 sqlx `Encode` 兼容性。

#### 1.2.3 修改权限审计（Service 层）

- [ ] 在 `PostService` 新增：

```rust
pub async fn update_post(
    &self,
    operator_id: Uuid,
    id: Uuid,
    req: UpdatePostRequest,
) -> Result<PostResponse, AppError>
```

- [ ] 先 `find_by_id(id)`，`None` 返回 `NotFound`。
- [ ] 校验操作人身份（修改权限，比读取更严格）：
  - [ ] `operator_id == author_id`（作者）：允许。
  - [ ] 进阶：`operator_id` 在 `post_collaborators` 且 `role` 为 `editor` 或 `admin`：允许。
  - [ ] 否则：返回 `AppError::Forbidden("无权编辑该文章")`。
- [ ] 通过后调用 `update_post` 落库。
- [ ] 记录审计日志：`info!(post_id, operator_id, fields_changed, "文章更新成功")`。
- [ ] 明确草稿 / 发布切换走同一接口，或单独提供 `PATCH /:id/status`（二选一并记录）。
- [ ] 编写权限矩阵单元测试（作者 / editor / admin / viewer / 非协作者）。

#### 1.2.4 端点暴露（Handler & Route 层）

- [ ] 在 `post_handler.rs` 新增 `update_post`：
  - [ ] `Path(id)` + `State` + `auth_user: AuthUser` + `Json(req)`。
  - [ ] `req.validate()?`。
  - [ ] 调用 `state.post_service.update_post(auth_user.id, id, req)`。
- [ ] 在 `post_route.rs` 挂载：

```rust
.route("/{id}", patch(post_handler::update_post))
```

- [ ] 确认最终路由为 `PATCH /api/v1/posts/:id`。

---

### 1.3 列表分页与元数据轻量化检索

#### 1.3.1 分页请求与轻量契约（DTO 层）

- [ ] 在 `src/models/dto/post.rs` 新增分页入参 `PostQuery`：

```rust
#[derive(Debug, Deserialize)]
pub struct PostQuery {
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_per_page")]
    pub per_page: u32,
    pub is_published: Option<bool>,
    pub author_id: Option<Uuid>,
}

fn default_page() -> u32 { 1 }
fn default_per_page() -> u32 { 10 }
```

- [ ] 在 Service 层做边界钳制：`per_page` 限制在 `1..=100`，`page` 最小 1，超出返回 `BadRequest` 或自动钳制（二选一并记录）。
- [ ] 新增轻量响应模型 `PostListItemResp`，严格剔除 `content`：

```rust
#[derive(Debug, Serialize)]
pub struct PostListItemResp {
    pub id: Uuid,
    pub author_id: Uuid,
    pub title: String,
    pub summary: Option<String>,
    pub cover_image: Option<String>,
    pub is_published: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

- [ ] 实现 `From<Post> for PostListItemResp`，显式不复制 `content`。

#### 1.3.2 按需高效查询（Repository 层）

- [ ] 在 `PostRepository` trait 中新增契约：

```rust
async fn list_posts(&self, filter: &PostQuery) -> Result<Vec<Post>, AppError>;
async fn count_posts(&self, filter: &PostQuery) -> Result<u64, AppError>;
```

- [ ] 在 `PgPostRepository` 中实现分页 + 计数 SQL（`LIMIT` / `OFFSET`，首版不建议游标分页）：

```sql
SELECT id, author_id, title, summary, cover_image,
       is_published, created_at, updated_at
FROM posts
WHERE deleted_at IS NULL
  AND ($1::bool IS NULL OR is_published = $1)
  AND ($2::uuid IS NULL OR author_id = $2)
ORDER BY created_at DESC
LIMIT $3 OFFSET $4
```

- [ ] 计数查询使用相同 WHERE 条件，`SELECT COUNT(*)`。
- [ ] 建议用 `query_as` 直接映射轻量结构体，避免把 `content` 拉进内存。
- [ ] 区分两种检索场景：
  - [ ] 公开博客流：`is_published = Some(true)`。
  - [ ] 作者私密控制台：`author_id = Some(current_user_id)`，并要求操作者本人或具备查看权限。

#### 1.3.3 端点暴露（Handler & Route 层）

- [ ] 在 `post_handler.rs` 新增 `list_posts`：
  - [ ] `Query(query): Query<PostQuery>` + `State` + `optional_user: OptionalAuthUser`。
  - [ ] 判定检索模式：匿名 / 公开强制 `is_published = true`；已登录可选作者控制台。
  - [ ] 调用 service 层返回 `Paginated<PostListItemResp>`。
- [ ] 在 `post_route.rs` 挂载：

```rust
.route("/", get(post_handler::list_posts))
```

- [ ] 确认与已有 `POST /`（创建）共存，最终路由为 `GET /api/v1/posts`。

---

### 1.4 协作者权限体系（进阶，可选）

- [ ] 新增协作者管理接口（如需要）：
  - [ ] `POST /api/v1/posts/:id/collaborators`
  - [ ] `DELETE /api/v1/posts/:id/collaborators/:user_id`
- [ ] 在 `PostRepository` 中新增：

```rust
async fn find_collaborator_role(&self, post_id: Uuid, user_id: Uuid) -> Result<Option<String>, AppError>;
async fn add_collaborator(&self, post_id: Uuid, user_id: Uuid, role: &str) -> Result<(), AppError>;
async fn remove_collaborator(&self, post_id: Uuid, user_id: Uuid) -> Result<(), AppError>;
```

- [ ] 统一角色枚举常量：`viewer` / `editor` / `admin` / `owner`，避免散落魔法字符串。
- [ ] 将 1.1.3 / 1.2.3 的权限判定抽象为 `PostService` 内私有辅助函数 `can_view` / `can_edit`，供详情、更新、删除复用。

---

## 第 2 章 对象存储媒体直传

### 2.1 MinIO 预签名 URL 签发服务

#### 2.1.1 依赖与客户端构建（基础设施层）

- [ ] 引入 S3 兼容客户端依赖（二选一）：
  - [ ] 方案 A（官方 SDK，功能全但依赖较重）：`aws-sdk-s3` + `aws-config`。
  - [ ] 方案 B（轻量、预签名 URL 简洁，推荐）：`rust-s3`（crate 名 `s3`，开启 `tokio` 特性）。
- [ ] 在 `.env` 中补齐 MinIO 相关变量（勿提交真实密钥）：

```
S3_ENDPOINT=http://localhost:9000
S3_ACCESS_KEY=minioadmin
S3_SECRET_KEY=minioadmin
S3_BUCKET=blog-media
S3_REGION=us-east-1
S3_PUBLIC_BASE_URL=http://localhost:9000/blog-media
S3_UPLOAD_URL_TTL=900
```

- [ ] 新增存储客户端封装（建议 `src/storage/` 目录）：
  - [ ] `src/storage/mod.rs`
  - [ ] `src/storage/client.rs`：`S3Storage` 结构体，持有 `Bucket`、凭证、Endpoint、Region、TTL。
- [ ] 在 `AppState` 中注入存储客户端（`Arc<MediaStorage>` 或经 `MediaService` 间接持有）。
- [ ] 确保开启 path-style 寻址（MinIO 默认 path-style）。

#### 2.1.2 直传凭证签发（DTO & Service 层）

- [ ] 新增 `GetUploadUrlRequest` DTO（建议新建 `src/models/dto/media.rs`）：

```rust
#[derive(Debug, Deserialize, Validate)]
pub struct GetUploadUrlRequest {
    #[validate(length(min = 1, max = 255))]
    pub content_type: String,
    #[validate(length(min = 1, max = 16))]
    pub ext: String,
}
```

- [ ] 使用白名单校验 `content_type` 与 `ext`：仅允许图片 / 视频 / 音频等安全类型，拒绝 `text/html`、可执行 / 可注入类型。
- [ ] 新增响应 `GetUploadUrlResponse`：

```rust
#[derive(Debug, Serialize)]
pub struct GetUploadUrlResponse {
    pub upload_url: String,
    pub object_key: String,
    pub permanent_url: String,
    pub expires_in: u32,
}
```

- [ ] 新增 `MediaService`（`src/service/media_service.rs`），方法 `create_upload_url`：
  - [ ] 用 UUID 自动重命名文件：`object_key = format!("media/{}.{}", Uuid::new_v4(), sanitize_ext(&req.ext))`。
  - [ ] 对 `ext` 做严格清洗：去点、去 `/`、`\\`、`..`，仅保留 `[a-zA-Z0-9]+`，杜绝路径穿越。
  - [ ] 对 `content_type` 做白名单校验，非法返回 `AppError::BadRequest`。
  - [ ] 调用 S3 SDK 签发 PUT 预签名 URL（有效期 15 分钟）。
  - [ ] 拼装 `permanent_url`：`S3_PUBLIC_BASE_URL` + `object_key`。
- [ ] 将 `MediaService` 注入 `AppState`，或在 `PostService` 中组合引用（二选一并记录）。

#### 2.1.3 端点暴露（Handler & Route 层）

- [ ] 新增 `src/handler/media_handler.rs`：
  - [ ] `create_upload_url(State, auth_user: AuthUser, Json(req))`。
  - [ ] `req.validate()?`。
  - [ ] 调用 `MediaService::create_upload_url`，返回 `Json(GetUploadUrlResponse)`。
- [ ] 新增 `src/route/media_route.rs`：

```rust
pub fn media_routes() -> Router<AppState> {
    Router::new().route("/upload-url", post(media_handler::create_upload_url))
}
```

- [ ] 在 `src/route/mod.rs` 中挂载，受 `AuthUser` 保护：

```rust
.nest("/api/v1/assets", media_route::media_routes())
```

- [ ] 确认最终路由为 `POST /api/v1/assets/upload-url`。

---

## 第 3 章 个人内容模块

> 音乐播放器、语录集、阅读清单、足迹、项目页属于「个人内容收藏 / 展示」类，结构相似：实体 + 增删改查 + 分页 + 可选标签分类。本章用统一模板描述，各模块特有字段单独列出。

### 3.1 音乐播放器

#### 3.1.1 数据模型

- [ ] 新增 `songs` 表（歌曲库）：

```sql
create table if not exists songs (
    id uuid primary key default gen_random_uuid(),
    user_id uuid not null references users(id) on delete cascade,
    title varchar(200) not null,
    artist varchar(200),
    album varchar(200),
    cover_url varchar(500),
    audio_url varchar(500) not null,     -- 复用对象存储直传
    duration int,                        -- 时长（秒）
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);
```

- [ ] 新增 `playlists` 与 `playlist_songs`（播放列表，多对多）：

```sql
create table if not exists playlists (
    id uuid primary key default gen_random_uuid(),
    user_id uuid not null references users(id) on delete cascade,
    name varchar(100) not null,
    cover_url varchar(500),
    is_public boolean not null default false,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

create table if not exists playlist_songs (
    playlist_id uuid not null references playlists(id) on delete cascade,
    song_id uuid not null references songs(id) on delete cascade,
    position int not null default 0,     -- 排序
    added_at timestamptz not null default now(),
    primary key (playlist_id, song_id)
);
```

- [ ] 新增 `play_history`（播放记录，用于「最近播放 / 常听」）：

```sql
create table if not exists play_history (
    id uuid primary key default gen_random_uuid(),
    user_id uuid not null references users(id) on delete cascade,
    song_id uuid not null references songs(id) on delete cascade,
    played_at timestamptz not null default now()
);
create index idx_play_history_user on play_history(user_id, played_at desc);
```

#### 3.1.2 功能清单

- [ ] 歌曲上传（复用 `POST /api/v1/assets/upload-url`，音频 `content_type` 加入白名单）。
- [ ] 歌曲 / 播放列表增删改查（`songs`、`playlists`）。
- [ ] 播放列表内歌曲排序与添加移除。
- [ ] 播放记录上报 + 最近播放 / 常听列表。
- [ ] 公开播放列表支持匿名访问（游客可查看 `is_public = true` 的列表）。

#### 3.1.3 路由

- [ ] 挂载 `/api/v1/music`：`songs`、`playlists`、`playlists/:id/songs`、`history`。

---

### 3.2 语录集

#### 3.2.1 数据模型

- [ ] 新增 `quotes` 与 `quote_tags`（标签多对多）：

```sql
create table if not exists quotes (
    id uuid primary key default gen_random_uuid(),
    user_id uuid not null references users(id) on delete cascade,
    content text not null,
    author varchar(200),
    source varchar(200),                 -- 出处
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

create table if not exists quote_tags (
    quote_id uuid not null references quotes(id) on delete cascade,
    tag varchar(50) not null,
    primary key (quote_id, tag)
);
```

#### 3.2.2 功能清单

- [ ] 语录增删改查 + 标签。
- [ ] 随机返回一条（`GET /api/v1/quotes/random`）。
- [ ] 按标签 / 作者 / 来源筛选分页。
- [ ] 收藏语录（进阶，复用通用收藏表或独立 `quote_likes`）。

#### 3.2.3 路由

- [ ] 挂载 `/api/v1/quotes`。

---

### 3.3 阅读清单

#### 3.3.1 数据模型

- [ ] 新增 `books` 与 `reading_notes`：

```sql
create table if not exists books (
    id uuid primary key default gen_random_uuid(),
    user_id uuid not null references users(id) on delete cascade,
    title varchar(200) not null,
    author varchar(200),
    cover_url varchar(500),
    isbn varchar(30),
    status varchar(20) not null default 'wishlist',  -- wishlist / reading / finished / dropped
    rating smallint,                     -- 1-5 星
    started_at timestamptz,
    finished_at timestamptz,
    notes text,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

create table if not exists reading_notes (
    id uuid primary key default gen_random_uuid(),
    book_id uuid not null references books(id) on delete cascade,
    user_id uuid not null references users(id) on delete cascade,
    content text not null,
    page varchar(50),
    created_at timestamptz not null default now()
);
```

#### 3.3.2 功能清单

- [ ] 书籍增删改查，状态流转（想读 → 在读 → 读完 / 弃读）。
- [ ] 阅读笔记增删改查，按书籍归组。
- [ ] 评分、起止日期记录。
- [ ] 年度阅读统计（读完 N 本，接入统计面板）。

#### 3.3.3 路由

- [ ] 挂载 `/api/v1/books`、`/api/v1/books/:id/notes`。

---

### 3.4 足迹

#### 3.4.1 数据模型

- [ ] 新增 `footprints`：

```sql
create table if not exists footprints (
    id uuid primary key default gen_random_uuid(),
    user_id uuid not null references users(id) on delete cascade,
    title varchar(200) not null,
    description text,
    location varchar(200),               -- 地点名
    latitude double precision,
    longitude double precision,
    photos jsonb default '[]',           -- 图片 object_key 数组
    visited_at date not null,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);
create index idx_footprints_user_date on footprints(user_id, visited_at desc);
```

#### 3.4.2 功能清单

- [ ] 足迹增删改查 + 照片上传（复用对象存储）。
- [ ] 地图聚合（前端用经纬度渲染，后端提供按区域 / 年份筛选）。
- [ ] 时间线 / 年份归档。

#### 3.4.3 路由

- [ ] 挂载 `/api/v1/footprints`。

---

### 3.5 项目页

#### 3.5.1 数据模型

- [ ] 新增 `projects`：

```sql
create table if not exists projects (
    id uuid primary key default gen_random_uuid(),
    user_id uuid not null references users(id) on delete cascade,
    name varchar(200) not null,
    description text,
    cover_url varchar(500),
    repo_url varchar(500),
    demo_url varchar(500),
    tech_stack jsonb default '[]',       -- 技术栈标签数组
    status varchar(20) not null default 'active',  -- active / archived / deprecated
    is_featured boolean not null default false,     -- 首页精选
    sort_order int not null default 0,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);
```

#### 3.5.2 功能清单

- [ ] 项目增删改查。
- [ ] 首页精选（`is_featured`）排序展示。
- [ ] 按技术栈 / 状态筛选。
- [ ] 公开项目匿名可访问。

#### 3.5.3 路由

- [ ] 挂载 `/api/v1/projects`。

---

## 第 4 章 生产力工具模块

> Todo 清单、Anki、习惯打卡、目标管理、番茄钟属于「生产力」类，涉及调度、统计、重复规则等较复杂逻辑，本章逐项展开。

### 4.1 Todo 清单（含随机提醒，重点）

#### 4.1.1 数据模型

- [ ] 新增 `todos`（任务主表）：

```sql
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
    completed_count int not null default 0,   -- 累计完成次数
    priority smallint not null default 0,     -- 优先级 0-3
    -- 随机提醒配置
    reminder_enabled boolean not null default false,
    reminder_period varchar(10),              -- day / week / month
    reminder_count int,                       -- 周期内随机提醒 N 次
    reminder_window_start time,               -- 可提醒时间段起点
    reminder_window_end time,                 -- 可提醒时间段终点
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);
create index idx_todos_user on todos(user_id, due_date);
```

- [ ] 新增 `todo_records`（完成记录，用于报表）：

```sql
create table if not exists todo_records (
    id uuid primary key default gen_random_uuid(),
    todo_id uuid not null references todos(id) on delete cascade,
    user_id uuid not null references users(id) on delete cascade,
    note text,                            -- 完成备注
    completed_at timestamptz not null default now()
);
create index idx_todo_records_todo on todo_records(todo_id, completed_at desc);
create index idx_todo_records_user on todo_records(user_id, completed_at desc);
```

- [ ] 新增 `todo_reminders`（已生成的随机提醒时间点）：

```sql
create table if not exists todo_reminders (
    id uuid primary key default gen_random_uuid(),
    todo_id uuid not null references todos(id) on delete cascade,
    user_id uuid not null references users(id) on delete cascade,
    remind_at timestamptz not null,       -- 具体提醒时刻（UTC）
    fired boolean not null default false, -- 是否已触发
    created_at timestamptz not null default now()
);
create index idx_todo_reminders_pending on todo_reminders(fired, remind_at);
```

#### 4.1.2 随机提醒生成算法

- [ ] 在 `TodoService` 实现 `regenerate_reminders(todo)`：
  - [ ] 入参：`reminder_period`（day / week / month）、`reminder_count`（N 次）、`reminder_window_start` / `end`（时间段）。
  - [ ] 算法：
    - [ ] 按周期划分时间跨度（当天 / 本周 / 本月）。
    - [ ] 在该周期内、且落在 `window_start ~ window_end` 的可用时间段里，均匀 / 随机采样 `N` 个时间点。
    - [ ] 每个时间点落库为一条 `todo_reminders`（`remind_at` 转 UTC）。
  - [ ] 周期结束（次日 / 下周 / 下月初）自动重新生成下一周期的随机时间点（由调度器每日兜底任务触发）。
- [ ] 处理重复任务（`repeat_type != none`）：每天 / 工作日 / 休息日，完成状态在下一个周期开始时自动复位，`completed_count` 累计不清零。
- [ ] 工作日 / 休息日判定基于用户 `timezone`（见 0.4）。

#### 4.1.3 调度接入

- [ ] 提醒触发流程：`SchedulerService` 扫描 `fired = false AND remind_at <= now()` 的记录 → 调用 `NotificationService.push` → 标记 `fired = true`。
- [ ] 兜底：每日扫描任务，将周期结束的 `todo_reminders` 滚动重建。

#### 4.1.4 备注与完成

- [ ] 新建任务支持 `description`（初始备注）。
- [ ] 完成任务接口 `POST /api/v1/todos/:id/complete`，请求体可带 `note`（完成备注），写入 `todo_records`。
- [ ] 支持查看某任务的历史完成记录与备注列表。

#### 4.1.5 报表

- [ ] 新增报表查询（返回统计结构，不落库）：
  - [ ] 日报表：当天完成数、按时完成率、按优先级分布。
  - [ ] 周报表：本周 7 天每日完成数折线数据。
  - [ ] 月报表：本月每天完成数 + 总完成数 + 环比。
  - [ ] 年报表：12 个月每月完成数。
- [ ] 报表接口统一挂 `/api/v1/todos/reports?period=day|week|month|year`。

#### 4.1.6 路由

- [ ] 挂载 `/api/v1/todos`：CRUD、`/:id/complete`、`/:id/records`、`/reports`。

---

### 4.2 Anki 记忆卡

#### 4.2.1 数据模型

- [ ] 新增 `decks`、`cards`、`card_reviews`：

```sql
create table if not exists decks (
    id uuid primary key default gen_random_uuid(),
    user_id uuid not null references users(id) on delete cascade,
    name varchar(200) not null,
    description text,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

create table if not exists cards (
    id uuid primary key default gen_random_uuid(),
    deck_id uuid not null references decks(id) on delete cascade,
    user_id uuid not null references users(id) on delete cascade,
    front text not null,                 -- 正面（问题）
    back text not null,                  -- 背面（答案）
    tags jsonb default '[]',
    -- SM-2 字段
    ease_factor double precision not null default 2.5,
    interval_days int not null default 0,   -- 当前间隔（天）
    repetitions int not null default 0,     -- 连续答对次数
    due_at timestamptz not null default now(),
    lapses int not null default 0,          -- 忘记次数
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);
create index idx_cards_due on cards(user_id, due_at);

create table if not exists card_reviews (
    id uuid primary key default gen_random_uuid(),
    card_id uuid not null references cards(id) on delete cascade,
    user_id uuid not null references users(id) on delete cascade,
    rating smallint not null,            -- 0=again 1=hard 2=good 3=easy
    interval_before int,
    interval_after int,
    reviewed_at timestamptz not null default now()
);
```

#### 4.2.2 SM-2 间隔重复算法

- [ ] 实现 `sm2_update(card, rating) -> Card`：
  - [ ] `rating = again(0)`：`repetitions` 归零，`interval_days = 0`，`lapses += 1`，`ease_factor = max(1.3, ease_factor - 0.2)`。
  - [ ] `rating = hard(1)`：`interval_days = interval * 1.2`（最小 1 天），`ease_factor -= 0.15`。
  - [ ] `rating = good(2)`：`interval_days = (repetitions == 0) ? 1 : (repetitions == 1) ? 6 : round(interval * ease_factor)`，`ease_factor` 不变。
  - [ ] `rating = easy(3)`：`interval_days = round(interval * ease_factor * 1.3)`，`ease_factor += 0.15`。
  - [ ] `due_at = now + interval_days`。
  - [ ] 每次复习写入 `card_reviews`。
- [ ] 引入可配置的 SM-2 参数（`EASY_BONUS`、`HARD_INTERVAL_MULTIPLIER` 等），避免硬编码。

#### 4.2.3 复习流程

- [ ] `GET /api/v1/anki/decks/:id/due`：返回到期卡片（`due_at <= now()`）。
- [ ] `POST /api/v1/anki/cards/:id/review`：提交 `rating`，执行 SM-2 更新。
- [ ] 学习量统计：今日新卡 / 今日复习数 / 到期总数（接入统计面板）。

#### 4.2.4 路由

- [ ] 挂载 `/api/v1/anki`：`decks`、`cards`、`cards/:id/review`。

---

### 4.3 习惯打卡

#### 4.3.1 数据模型

- [ ] 新增 `habits` 与 `habit_checks`：

```sql
create table if not exists habits (
    id uuid primary key default gen_random_uuid(),
    user_id uuid not null references users(id) on delete cascade,
    name varchar(200) not null,
    description text,
    icon varchar(50),
    color varchar(20),
    -- 打卡频率：daily / weekly / monthly
    frequency varchar(20) not null default 'daily',
    target_days int not null default 1,      -- 每周 / 每月目标天数
    reminder_time time,                      -- 每日提醒时间
    is_archived boolean not null default false,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

create table if not exists habit_checks (
    id uuid primary key default gen_random_uuid(),
    habit_id uuid not null references habits(id) on delete cascade,
    user_id uuid not null references users(id) on delete cascade,
    check_date date not null,                -- 打卡日期（用户时区）
    checked_at timestamptz not null default now(),
    unique (habit_id, check_date)
);
create index idx_habit_checks_habit on habit_checks(habit_id, check_date desc);
```

#### 4.3.2 连续天数（Streak）计算

- [ ] 实现 `calc_streak(habit, today)`：
  - [ ] 从今天（用户时区）往前遍历 `habit_checks.check_date`，统计连续打卡天数。
  - [ ] 若今天未打卡但昨天已打卡，streak 从昨天算起（允许「今天还没打，连续未断」）。
  - [ ] 返回 `current_streak` 与 `longest_streak`（历史最长）。
- [ ] 提供 `/api/v1/habits/:id/streak` 查询连续天数。

#### 4.3.3 打卡与提醒

- [ ] 打卡接口 `POST /api/v1/habits/:id/check`（幂等：同一天重复打卡返回已存在状态，不重复计数）。
- [ ] 取消打卡 `DELETE /api/v1/habits/:id/check`。
- [ ] 每日提醒接入调度器，`reminder_time` 触发站内通知。

#### 4.3.4 路由

- [ ] 挂载 `/api/v1/habits`。

---

### 4.4 目标管理

#### 4.4.1 数据模型

- [ ] 新增 `goals` 与 `goal_milestones`：

```sql
create table if not exists goals (
    id uuid primary key default gen_random_uuid(),
    user_id uuid not null references users(id) on delete cascade,
    title varchar(200) not null,
    description text,
    category varchar(50),
    target_date date,
    progress int not null default 0,         -- 0-100 进度百分比
    status varchar(20) not null default 'active',  -- active / achieved / abandoned
    -- 可关联习惯 / 项目
    linked_habit_id uuid references habits(id) on delete set null,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

create table if not exists goal_milestones (
    id uuid primary key default gen_random_uuid(),
    goal_id uuid not null references goals(id) on delete cascade,
    title varchar(200) not null,
    is_done boolean not null default false,
    done_at timestamptz,
    created_at timestamptz not null default now()
);
```

#### 4.4.2 功能清单

- [ ] 目标增删改查 + 里程碑拆分。
- [ ] 进度更新：手动设置或由关联习惯自动累计（进阶）。
- [ ] 到期提醒（`target_date` 前 N 天通知，接入调度器）。
- [ ] 目标状态流转：进行中 → 达成 / 放弃。

#### 4.4.3 路由

- [ ] 挂载 `/api/v1/goals`、`/api/v1/goals/:id/milestones`。

---

### 4.5 番茄钟

#### 4.5.1 数据模型

- [ ] 新增 `pomodoro_sessions`：

```sql
create table if not exists pomodoro_sessions (
    id uuid primary key default gen_random_uuid(),
    user_id uuid not null references users(id) on delete cascade,
    task_title varchar(200),             -- 关联的任务描述
    -- 可选关联 todo / 项目
    todo_id uuid references todos(id) on delete set null,
    duration_minutes int not null,       -- 番茄时长
    started_at timestamptz not null,
    ended_at timestamptz,
    status varchar(20) not null default 'running',  -- running / completed / interrupted
    created_at timestamptz not null default now()
);
create index idx_pomodoro_user on pomodoro_sessions(user_id, started_at desc);
```

#### 4.5.2 功能清单

- [ ] 开始番茄钟（`POST /api/v1/pomodoro/start`，记录 `started_at`）。
- [ ] 结束 / 中断番茄钟（`POST /api/v1/pomodoro/:id/end`，区分完成 / 中断）。
- [ ] 自定义番茄时长（25 分钟默认）与休息时长（配置存前端或用户设置）。
- [ ] 专注统计：今日 / 本周番茄数、专注总时长（接入统计面板）。
- [ ] 番茄结束提醒（接入通知）。

#### 4.5.3 路由

- [ ] 挂载 `/api/v1/pomodoro`。

---

## 第 5 章 展示与互动模块

### 5.1 统计面板

#### 5.1.1 功能清单

- [ ] 新增聚合查询服务 `DashboardService`，返回统一统计结构：

```rust
#[derive(Debug, Serialize)]
pub struct DashboardSummary {
    pub posts: CountStats,          // 文章总数、本月新增
    pub todos: TodoStats,           // 待办 / 完成 / 完成率
    pub habits: HabitStats,         // 今日打卡 / 最长连续
    pub anki: AnkiStats,           // 到期 / 今日复习
    pub pomodoro: PomodoroStats,   // 专注总时长 / 番茄数
    pub reading: ReadingStats,      // 在读 / 读完
    pub footprints: CountStats,     // 足迹数
}
```

- [ ] 各统计项提供「日 / 周 / 月 / 年」维度切换。
- [ ] 提供 `GET /api/v1/dashboard/summary` 一次返回所有聚合数据，前端渲染图表。

#### 5.1.2 路由

- [ ] 挂载 `/api/v1/dashboard`。

---

### 5.2 数据导出

#### 5.2.1 功能清单

- [ ] 新增 `ExportService`，支持导出用户全量数据为 JSON 归档：

```rust
#[derive(Debug, Serialize)]
pub struct ExportBundle {
    pub exported_at: DateTime<Utc>,
    pub profile: UserResp,
    pub posts: Vec<PostResponse>,
    pub todos: Vec<Todo>,
    pub habits: Vec<Habit>,
    pub cards: Vec<Card>,
    pub quotes: Vec<Quote>,
    pub books: Vec<Book>,
    pub footprints: Vec<Footprint>,
    pub projects: Vec<Project>,
}
```

- [ ] `GET /api/v1/export/json`：流式返回 JSON（或生成临时文件 + 下载链接）。
- [ ] 进阶：支持按模块选择导出、CSV 格式导出。
- [ ] 生成任务走后台异步，避免大数据量阻塞请求（可选）。

#### 5.2.2 路由

- [ ] 挂载 `/api/v1/export`。

---

### 5.3 留言墙

#### 5.3.1 数据模型

- [ ] 新增 `messages` 与 `message_replies`：

```sql
create table if not exists messages (
    id uuid primary key default gen_random_uuid(),
    user_id uuid references users(id) on delete set null,  -- 登录用户可空（匿名留言）
    nickname varchar(50) not null,
    content text not null,
    is_approved boolean not null default false,  -- 审核开关
    created_at timestamptz not null default now()
);

create table if not exists message_replies (
    id uuid primary key default gen_random_uuid(),
    message_id uuid not null references messages(id) on delete cascade,
    user_id uuid references users(id) on delete set null,
    content text not null,
    created_at timestamptz not null default now()
);
```

#### 5.3.2 功能清单

- [ ] 留言发布（登录或匿名 + 昵称）。
- [ ] 留言审核（`is_approved`，公开列表仅展示已审核）。
- [ ] 回复 / 删除。
- [ ] 分页 + 匿名留言的防刷（频率限制，进阶）。

#### 5.3.3 路由

- [ ] 挂载 `/api/v1/messages`（公开读取已审核，写操作受保护）。

---

### 5.4 联系页

#### 5.4.1 数据模型

- [ ] 复用 `messages` 表，或用独立 `contact_submissions` 表：

```sql
create table if not exists contact_submissions (
    id uuid primary key default gen_random_uuid(),
    name varchar(100) not null,
    email varchar(200) not null,
    subject varchar(200),
    content text not null,
    is_read boolean not null default false,
    created_at timestamptz not null default now()
);
```

#### 5.4.2 功能清单

- [ ] 联系表单提交（防 spam：Honeypot / 频率限制 / 验证码，进阶）。
- [ ] 站长侧查看 / 标记已读。
- [ ] 可选：接入邮件通知（第三方 SMTP，进阶）。

#### 5.4.3 路由

- [ ] 挂载 `/api/v1/contact`。

---

## 第 6 章 个人中心与安全

### 6.1 个人资料

#### 6.1.1 功能清单

- [ ] 完善 `UpdateUserRequest` 字段，与 `User` 实体对齐：

```rust
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub slogan: Option<String>,
    pub gender: Option<i16>,
    pub birthday: Option<NaiveDate>,
    pub phone: Option<String>,
}
```

- [ ] 头像上传复用对象存储（`avatar_url` 存永久链接）。
- [ ] 公开资料页 `GET /api/v1/profile/:id`（脱敏，仅返回公开字段）。
- [ ] 补齐 `UserResp` 字段（`bio`、`slogan`、`avatar_url`、`gender`、`birthday`）。

#### 6.1.2 路由

- [ ] 挂载 `/api/v1/profile`（`GET /:id`、`PATCH /`、`PUT /avatar`）。

---

### 6.2 安全设置

#### 6.2.1 功能清单

- [ ] 修改密码 `POST /api/v1/security/password`：校验旧密码 + Argon2 重哈希新密码。
- [ ] 邮箱验证（进阶）：发送验证邮件 + 校验 token，更新 `email_verified`。
- [ ] 两步验证（进阶）：基于 TOTP（`two_factor_secret`），生成 / 验证 / 解绑。
- [ ] 登录会话管理：签发 JWT 时记录 `session_id`，支持「退出全部设备」（维护会话黑名单或使用短期 token + refresh token）。
- [ ] 登录审计：记录 `last_login_ip`、`last_login_at`（登录时更新）。
- [ ] 密码强度校验：`validate_password_strength`（长度 / 复杂度）。
- [ ] 登录限流：失败次数过多锁定（进阶，接入 `rate limiting`）。

#### 6.2.2 数据模型变更

- [ ] `users` 表新增 `updated_at`（如缺）、`password_changed_at`（用于失效旧 token，进阶）。

#### 6.2.3 路由

- [ ] 挂载 `/api/v1/security`。

---

## 第 7 章 测试与验收

### 7.1 单元测试

- [ ] 提取器：`OptionalAuthUser` 三种情形。
- [ ] DTO 校验：`UpdatePostRequest`、`PostQuery`、`GetUploadUrlRequest`（非法 `content_type`、危险 `ext`）。
- [ ] 可见性决策树与编辑权限矩阵。
- [ ] `From<Post> for PostListItemResp` 确认不含 `content`。
- [ ] SM-2 间隔重复算法（四种 rating 分支）。
- [ ] 习惯连续天数计算（含边界：跨月、今天未打、历史最长）。
- [ ] Todo 随机提醒生成算法（窗口内 N 次、时间段约束、UTC 转换）。
- [ ] 工作日 / 休息日判定（时区相关）。

### 7.2 集成测试 / 手工验收

- [ ] 文章：详情可见性、编辑权限、列表分页（见第 1 章）。
- [ ] 对象存储：上传 URL 签发、实际 PUT 上传、过期失效（见第 2 章）。
- [ ] Todo：随机提醒按周期触发、重复任务每日复位、完成次数累计、备注记录、日 / 周 / 月 / 年报表。
- [ ] Anki：到期卡片列表、复习后间隔递增、遗忘回退。
- [ ] 习惯：打卡幂等、streak 计算、每日提醒。
- [ ] 目标：里程碑完成、到期提醒、进度更新。
- [ ] 番茄钟：开始 / 结束 / 中断、专注统计。
- [ ] 统计面板：各聚合数据准确。
- [ ] 数据导出：JSON 归档完整性。
- [ ] 留言墙 / 联系页：审核开关、防刷。
- [ ] 个人资料：头像上传、字段更新、脱敏。
- [ ] 安全设置：改密、退出全部设备、密码强度校验。

### 7.3 文档与收尾

- [ ] 更新 `README.md` / `docs/学习内容.md`，补充全部新接口说明与环境变量。
- [ ] 补齐 `.env.example`，列出 MinIO、SMTP、时区等相关变量。
- [ ] 代码评审前跑通 `cargo fmt`、`cargo clippy`、`cargo test`、`cargo build`。
