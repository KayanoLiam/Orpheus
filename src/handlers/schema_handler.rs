// Schema Handler - Schema 信息查询 API
// 提供查询数据库结构的 HTTP 端点

use crate::models::response::ApiResponse;
use crate::schema::{self, SchemaCache};
use actix_web::{get, web, HttpResponse, Result};
use serde_json::json;
use sqlx::PgPool;

/// 获取所有表名
///
/// GET /schema/tables
#[get("/schema/tables")]
pub async fn get_tables(pool: web::Data<PgPool>) -> Result<HttpResponse> {
    match schema::get_all_tables(pool.get_ref(), None).await {
        Ok(tables) => Ok(HttpResponse::Ok().json(ApiResponse::success(json!({
            "tables": tables,
            "count": tables.len(),
        })))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::error(&format!(
            "Failed to fetch tables: {}",
            e
        )))),
    }
}

/// 获取指定表的结构信息
///
/// GET /schema/tables/{table_name}
#[get("/schema/tables/{table_name}")]
pub async fn get_table_info(
    pool: web::Data<PgPool>,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let table_name = path.into_inner();

    match schema::get_table_schema(pool.get_ref(), &table_name, None).await {
        Ok(schema) => Ok(HttpResponse::Ok().json(ApiResponse::success(schema))),
        Err(e) => Ok(HttpResponse::NotFound().json(ApiResponse::error(&format!(
            "Table '{}' not found: {}",
            table_name, e
        )))),
    }
}

/// 获取 Schema 概览
///
/// GET /schema/overview
#[get("/schema/overview")]
pub async fn get_schema_overview(pool: web::Data<PgPool>) -> Result<HttpResponse> {
    match schema::get_schema_overview(pool.get_ref(), None).await {
        Ok(overview) => Ok(HttpResponse::Ok().json(ApiResponse::success(overview))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::error(&format!(
            "Failed to fetch schema overview: {}",
            e
        )))),
    }
}

/// 使用缓存获取表结构信息
///
/// GET /schema/cached/tables/{table_name}
#[get("/schema/cached/tables/{table_name}")]
pub async fn get_cached_table_info(
    cache: web::Data<SchemaCache>,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let table_name = path.into_inner();

    match cache.get_table_schema(&table_name, None).await {
        Ok(schema) => Ok(HttpResponse::Ok().json(ApiResponse::success(schema))),
        Err(e) => Ok(HttpResponse::NotFound().json(ApiResponse::error(&format!(
            "Table '{}' not found: {}",
            table_name, e
        )))),
    }
}

/// 获取缓存统计信息
///
/// GET /schema/cache/stats
#[get("/schema/cache/stats")]
pub async fn get_cache_stats(cache: web::Data<SchemaCache>) -> Result<HttpResponse> {
    let stats = cache.stats().await;
    Ok(HttpResponse::Ok().json(ApiResponse::success(stats)))
}

/// 清空缓存
///
/// POST /schema/cache/clear
#[actix_web::post("/schema/cache/clear")]
pub async fn clear_cache(cache: web::Data<SchemaCache>) -> Result<HttpResponse> {
    cache.clear().await;
    Ok(HttpResponse::Ok().json(ApiResponse::success("Cache cleared")))
}

/// 预加载所有表的 schema 到缓存
///
/// POST /schema/cache/preload
#[actix_web::post("/schema/cache/preload")]
pub async fn preload_cache(cache: web::Data<SchemaCache>) -> Result<HttpResponse> {
    match cache.preload(None).await {
        Ok(()) => Ok(HttpResponse::Ok().json(ApiResponse::success("Cache preloaded"))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::error(&format!(
            "Failed to preload cache: {}",
            e
        )))),
    }
}
