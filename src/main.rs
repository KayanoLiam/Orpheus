// 严格的安全编码策略：禁用可能引发 panic 或不安全的操作
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::todo)]
#![deny(clippy::unimplemented)]
#![deny(clippy::empty_loop)]
#![deny(clippy::indexing_slicing)]
#![deny(unused)]

// 模块声明
mod auth; // 认证相关功能
mod config;
mod handlers; // HTTP 请求处理器
mod middlewares; // 中间件
mod models; // 数据模型 // 配置常量

// 导入处理器函数
use crate::handlers::session_handler::{user_logout, user_profile};
use crate::handlers::user_handler::{user_login, user_reset_password, user_signup};
// 导入会话验证中间件
use crate::middlewares::session::session_validator;
// 导入 Actix-Web 核心组件
use actix_web::{web, App, HttpServer};
// 导入 HTTP 认证中间件
use actix_web_httpauth::middleware::HttpAuthentication;
// 导入环境变量加载器
use dotenvy::dotenv;
// 导入数据库连接池
use sqlx::{Pool, Postgres};
// 导入环境变量处理
use crate::handlers::github_handler::get_github_repo_stars;
use orpheus::handlers::user_handler::user_delete;
use std::env;
use crate::auth::status::auth_status;

/// 应用程序入口点
///
/// 初始化数据库连接、Redis 连接，并启动 HTTP 服务器
///
/// # 返回
/// 成功时返回 Ok(())，失败时返回错误信息
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 加载 .env 文件中的环境变量
    dotenv().ok();

    // 从环境变量获取数据库连接字符串
    let database_url: String = env::var("DATABASE_URL")?;
    // 创建 PostgreSQL 数据库连接池
    let pool: Pool<Postgres> = Pool::<Postgres>::connect(&database_url).await?;

    // 从环境变量获取 Redis 连接字符串
    let redis_url: String = env::var("REDIS_URL")?;
    // 创建 Redis 客户端
    let client = redis::Client::open(redis_url)?;

    // 输出服务器启动信息
    println!("The server is starting on http://127.0.0.1:8080");

    // 创建并配置 HTTP 服务器
    HttpServer::new(move || {
        // 初始化cors中间件
        let cors = actix_cors::Cors::default()
            .allowed_origin("http://localhost:3000") // 只允许前端域名
            .allowed_methods(vec!["GET"]) // 只允许 GET 请求
            .allowed_headers(vec![actix_web::http::header::CONTENT_TYPE])
            .max_age(3600);

        // 创建 Bearer Token 认证中间件
        let auth = HttpAuthentication::bearer(session_validator);

        // 配置应用程序
        App::new()
            // 应用 CORS 中间件
            .wrap(cors)
            // 注册数据库连接池为应用数据
            .app_data(actix_web::web::Data::new(pool.clone()))
            // 注册 Redis 客户端为应用数据
            .app_data(actix_web::web::Data::new(client.clone()))
            // 公开端点：用户注册
            .service(user_signup)
            // 公开端点：用户登录
            .service(user_login)
            // 公开端点：用户登出
            .service(user_logout)
            .service(user_reset_password)
            .service(user_delete)
            .service(get_github_repo_stars)
            .service(auth_status)
            // 需要认证的 API 端点组
            .service(
                web::scope("/api")
                    // 应用认证中间件
                    .wrap(auth)
                    // 受保护端点：获取用户资料
                    .service(user_profile),
            )
    })
    // 设置工作线程数量
    .workers(10)
    // 绑定到本地地址和端口
    .bind(("0.0.0.0", 8080))?
    // 启动服务器并等待完成
    .run()
    .await?;

    Ok(())
}
