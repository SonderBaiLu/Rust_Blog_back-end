use dotenvy::dotenv;
use rust_blog::bootstrap::{bootstrap, AppConfig};
use rust_blog::route;
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. 加载环境变量
    dotenv().ok();

    // 2. 初始化结构化日志 tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_blog=debug,tower_http=debug".into()),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_file(true)
                .with_line_number(true)
                .with_target(true),
        )
        .init();

    // 3. 读取关键环境变量
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set in .env file");

    info!("连接 PostgreSQL 数据库...");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;
    info!("PostgreSQL 数据库连接成功");

    // 4. 集中装配依赖链（Repository → Service → AppState）
    let state = bootstrap(pool, AppConfig { jwt_secret });

    // 5. 挂载路由
    let app = route::create_app(state);

    // 6. 启动 Axum Web 服务
    let addr = SocketAddr::from(([127, 0, 0, 1], 3456));
    info!("服务已启动在 http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
