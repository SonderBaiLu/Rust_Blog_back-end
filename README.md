# 待解决问题

### 1. CPU密集型

-[x] TODO: Argon2 密码哈希/校验是故意设计得极度消耗 CPU 的计算任务。在 Rust 异步生态中，如果直接在 async fn 主线程里运行耗时的密码比对，会卡住 Tokio 的 Worker 异步线程，导致其他 Concurrent 请求被排队延迟。
**实现思路**:

- 具体技术难点：Tokio 是基于协作式调度（Cooperative Scheduling）的高性能异步运行时，其工作线程（Worker Threads）数量与机器的 CPU 物理核心数严格绑定。Argon2 属于高强度计算与大内存占用的 CPU 密集型算法，单次哈希运算可能耗时数十毫秒。若直接在异步上下文中同步执行，会长时间霸占当前 Worker 线程，阻塞排队在该线程上的网络 I/O 事件循环，在高并发场景下将导致系统的整体响应吞吐断崖式下跌
- 利用 tokio::task::spawn_blocking 将 util/password.rs 中的 hash_password 和 verify_password 重构为异步函数，把 CPU 密集型任务派发给 Tokio 的专用阻塞线程池（Blocking Threadpool），在 Service 层通过 .await 解包结果并合理处理 JoinError
- x 账号软删除与注销: 暂时不开发以后再说

# 开发注意事项

1. 关于密码字段：**登录接口**只需非空/基础长度校验

- 为什么不需要调用密码复杂度校验？（核心设计原则）
  - 注册阶段 vs 登录阶段的职责不同：
    - 注册（RegisterReq）：需要强制复杂度（如长度 $\ge 6$、特殊字符、大小写等），确保入库密码符合安全策略。
    - 登录（LoginReq）：绝对不要进行严格的密码格式/复杂度校验。若未来系统的密码策略升级（例如从“至少 6 位”提高到“至少 8 位包含大写字母”），如果在登录接口做强校验，那些旧规则下注册的合法用户将在接口层直接被拦截，导致无法登录。
- 职责归属：
  - DTO 只需确保密码不为空（Fail-Fast，避免无效请求进入后续流程）。
  - 密码“正确与否”属于业务层（Service）的职责，由 password::verify_password 通过 Argon2 哈希算法进行比对。

# 采用相关技术

### 储存 JSON 抽象语法树

1. PostgreSQL层： 正文字段采用JSONB类型
1. JSONB 会在写入时解析为二进制存储，天然校验 JSON 格式合法性，避免非法脏数据入库。
1. JSONB 支持在 PostgreSQL 内部建立 GIN 倒排索引，未来若需要在文章正文中检索特定节点属性，无需全表反序列化

- Rust/SQL层: 在实体与 DTO 中使用 serde_json::Value 或 sqlx::types::Json<serde_json::Value> 进行透明映射

2. MinIO 媒体上传模式规划（后续实现）

   架构选型建议：采用预签名 URL（Pre-signed URL）直传模式。

        流程：前端上传图片时，先向后端请求一个临时带鉴权的 PUT URL；前端直接把二进制文件推入 MinIO，上传完毕后仅将图片最终访问 URL 填入文章的 AST 节点或 cover_image。

        理由：大文件不经过 Axum 服务端中转，完全不占用后端的内存与网络带宽。
