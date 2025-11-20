// Orpheus - Backend-as-a-Service Platform
// 类 Supabase 的核心 BaaS 功能
// 
// 核心功能：
// 1. Auto REST API - 自动将数据库表转换为 RESTful API
// 2. Realtime - 实时数据订阅 (WebSocket)
// 3. Storage - S3 兼容的对象存储
// 4. Meta API - 数据库管理 API

#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::todo)]
#![deny(clippy::unimplemented)]
#![deny(clippy::empty_loop)]
#![deny(clippy::indexing_slicing)]
#![deny(unused)]

// 核心模块声明
mod schema;    // ✅ 数据库 schema 反射（已实现）
// mod rest;      // Auto REST API（下一步）
// mod realtime;  // 实时订阅
// mod storage;   // 对象存储
// mod meta;      // 数据库管理 API

// 临时保留的模块
mod models;    // 基础数据模型
mod handlers;  // 临时保留 GitHub handler 作为 API 示例

use crate::handlers::github_handler::get_github_repo_stars;
use crate::handlers::schema_handler;
use crate::schema::SchemaCache;
use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;
use sqlx::{Pool, Postgres};
use std::env;

/// Orpheus BaaS 平台主入口
///
/// 初始化：
/// - PostgreSQL 数据库连接池
/// - Redis 连接（用于缓存和会话）
/// - HTTP 服务器
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 加载环境变量
    dotenv().ok();

    // 数据库连接
    let database_url: String = env::var("DATABASE_URL")?;
    let pool: Pool<Postgres> = Pool::<Postgres>::connect(&database_url).await?;

    // Redis 连接
    let redis_url: String = env::var("REDIS_URL")?;
    let client = redis::Client::open(redis_url)?;

    // 初始化 Schema 缓存
    let schema_cache = SchemaCache::with_defaults(pool.clone());

    println!("🚀 Orpheus BaaS Platform");
    println!("   Core Services:");
    println!("   - Auto REST API: 开发中...");
    println!("   - Realtime:      开发中...");
    println!("   - Storage:       开发中...");
    println!("   - Meta API:      开发中...");
    println!();
    println!("   ✅ Schema Inspector: 已实现");
    println!();
    println!("🌐 Server running at http://127.0.0.1:8080");
    println!();
    println!("📚 Schema API 端点:");
    println!("   GET  /schema/tables              - 列出所有表");
    println!("   GET  /schema/tables/{{name}}       - 获取表结构");
    println!("   GET  /schema/overview            - Schema 概览");
    println!("   GET  /schema/cached/tables/{{name}} - 获取表结构（缓存）");
    println!("   GET  /schema/cache/stats         - 缓存统计");
    println!("   POST /schema/cache/preload       - 预加载缓存");
    println!();
    println!("📚 其他示例端点:");
    println!("   GET  /github/stars/:owner/:repo  - GitHub 仓库 stars 查询");
    println!();
    println!("💡 提示: 用户认证示例代码已移至 examples/authentication/");
    println!("💡 提示: 前端管理面板已移至 archived_projects/orpheus-admin-panel/");

    // 创建并配置 HTTP 服务器
    HttpServer::new(move || {
        // CORS 配置
        let cors = actix_cors::Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
            .allowed_headers(vec![
                actix_web::http::header::CONTENT_TYPE,
                actix_web::http::header::AUTHORIZATION,
            ])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(client.clone()))
            .app_data(web::Data::new(schema_cache.clone()))
            // Schema API 端点
            .service(schema_handler::get_tables)
            .service(schema_handler::get_table_info)
            .service(schema_handler::get_schema_overview)
            .service(schema_handler::get_cached_table_info)
            .service(schema_handler::get_cache_stats)
            .service(schema_handler::clear_cache)
            .service(schema_handler::preload_cache)
            // 示例端点：GitHub API 集成
            .service(get_github_repo_stars)
            // TODO: 添加核心 BaaS 端点
            // .service(web::scope("/rest/v1").configure(rest::configure))
            // .service(web::scope("/realtime/v1").configure(realtime::configure))
            // .service(web::scope("/storage/v1").configure(storage::configure))
            // .service(web::scope("/meta/v1").configure(meta::configure))
    })
    .workers(10)
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?;

    Ok(())
}
