// Schema module - 数据库结构检查和缓存
//
// 这个模块提供了读取 PostgreSQL 数据库结构的功能，
// 是 Auto REST API 生成的核心基础。
//
// # 主要组件
//
// - `types`: 数据结构定义（TableSchema, ColumnInfo 等）
// - `inspector`: 数据库结构检查器（从 information_schema 读取）
// - `cache`: Schema 缓存层（避免频繁查询）
//
// # 使用示例
//
// ```rust
// use orpheus::schema::{SchemaCache, inspector};
// use sqlx::PgPool;
//
// # async fn example(pool: PgPool) -> anyhow::Result<()> {
// // 方式1: 直接使用 inspector（无缓存）
// let tables = inspector::get_all_tables(&pool, None).await?;
// let schema = inspector::get_table_schema(&pool, "users", None).await?;
//
// // 方式2: 使用缓存（推荐）
// let cache = SchemaCache::with_defaults(pool);
// let schema = cache.get_table_schema("users", None).await?;
//
// // 预加载所有表的 schema
// cache.preload(None).await?;
// # Ok(())
// # }
// ```

// 这些函数和类型会在后续的 REST API 模块中使用
#![allow(dead_code)]

pub mod cache;
pub mod inspector;
pub mod types;

// 重新导出常用类型和函数
pub use cache::SchemaCache;
pub use inspector::{get_all_tables, get_schema_overview, get_table_schema};
