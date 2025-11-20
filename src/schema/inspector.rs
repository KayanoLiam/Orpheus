// Schema Inspector - 数据库结构检查器
// 用于读取 PostgreSQL 数据库的表结构信息

use super::types::{ColumnInfo, ForeignKeyInfo, IndexInfo, SchemaOverview, TableSchema};
use anyhow::{Context, Result};
use sqlx::{PgPool, Row};

/// 获取指定 schema 下的所有表名
///
/// # Arguments
/// * `pool` - PostgreSQL 连接池
/// * `schema_name` - Schema 名称，默认为 "public"
pub async fn get_all_tables(pool: &PgPool, schema_name: Option<&str>) -> Result<Vec<String>> {
    let schema = schema_name.unwrap_or("public");

    let rows = sqlx::query(
        "SELECT table_name 
         FROM information_schema.tables 
         WHERE table_schema = $1 
           AND table_type = 'BASE TABLE'
         ORDER BY table_name",
    )
    .bind(schema)
    .fetch_all(pool)
    .await
    .context("Failed to fetch table names")?;

    let tables = rows
        .iter()
        .map(|row| row.get::<String, _>("table_name"))
        .collect();

    Ok(tables)
}

/// 获取 schema 概览信息
pub async fn get_schema_overview(
    pool: &PgPool,
    schema_name: Option<&str>,
) -> Result<SchemaOverview> {
    let schema = schema_name.unwrap_or("public");
    let tables = get_all_tables(pool, Some(schema)).await?;
    let table_count = tables.len();

    Ok(SchemaOverview {
        name: schema.to_string(),
        tables,
        table_count,
    })
}

/// 获取表的完整结构信息
///
/// # Arguments
/// * `pool` - PostgreSQL 连接池
/// * `table_name` - 表名
/// * `schema_name` - Schema 名称，默认为 "public"
pub async fn get_table_schema(
    pool: &PgPool,
    table_name: &str,
    schema_name: Option<&str>,
) -> Result<TableSchema> {
    let schema = schema_name.unwrap_or("public");

    // 查询列信息
    let columns = get_columns(pool, table_name, schema).await?;

    // 查询主键
    let primary_keys = get_primary_keys(pool, table_name, schema).await?;

    // 查询外键
    let foreign_keys = get_foreign_keys(pool, table_name, schema).await?;

    // 查询索引
    let indexes = get_indexes(pool, table_name, schema).await?;

    // 查询表注释
    let comment = get_table_comment(pool, table_name, schema).await?;

    Ok(TableSchema {
        name: table_name.to_string(),
        schema: schema.to_string(),
        columns,
        primary_keys,
        foreign_keys,
        indexes,
        comment,
    })
}

/// 获取表的所有列信息
async fn get_columns(pool: &PgPool, table_name: &str, schema: &str) -> Result<Vec<ColumnInfo>> {
    let rows = sqlx::query(
        "SELECT 
            column_name,
            data_type,
            udt_name,
            is_nullable,
            column_default,
            is_identity,
            character_maximum_length,
            numeric_precision,
            numeric_scale,
            ordinal_position
        FROM information_schema.columns
        WHERE table_schema = $1 AND table_name = $2
        ORDER BY ordinal_position",
    )
    .bind(schema)
    .bind(table_name)
    .fetch_all(pool)
    .await
    .context("Failed to fetch column information")?;

    let mut columns = Vec::new();

    for row in rows {
        let column_name: String = row.get("column_name");
        
        // 查询列注释
        let comment = get_column_comment(pool, table_name, schema, &column_name).await?;

        columns.push(ColumnInfo {
            name: column_name,
            data_type: row.get("data_type"),
            udt_name: row.get("udt_name"),
            is_nullable: row.get::<String, _>("is_nullable") == "YES",
            default_value: row.get("column_default"),
            is_identity: row.get::<String, _>("is_identity") == "YES",
            max_length: row.get("character_maximum_length"),
            numeric_precision: row.get("numeric_precision"),
            numeric_scale: row.get("numeric_scale"),
            ordinal_position: row.get("ordinal_position"),
            comment,
        });
    }

    Ok(columns)
}

/// 获取表的主键列
async fn get_primary_keys(pool: &PgPool, table_name: &str, schema: &str) -> Result<Vec<String>> {
    let rows = sqlx::query(
        "SELECT kcu.column_name
         FROM information_schema.table_constraints tc
         JOIN information_schema.key_column_usage kcu 
           ON tc.constraint_name = kcu.constraint_name
           AND tc.table_schema = kcu.table_schema
         WHERE tc.constraint_type = 'PRIMARY KEY'
           AND tc.table_schema = $1
           AND tc.table_name = $2
         ORDER BY kcu.ordinal_position",
    )
    .bind(schema)
    .bind(table_name)
    .fetch_all(pool)
    .await
    .context("Failed to fetch primary keys")?;

    let primary_keys = rows
        .iter()
        .map(|row| row.get::<String, _>("column_name"))
        .collect();

    Ok(primary_keys)
}

/// 获取表的外键约束
async fn get_foreign_keys(
    pool: &PgPool,
    table_name: &str,
    schema: &str,
) -> Result<Vec<ForeignKeyInfo>> {
    let rows = sqlx::query(
        "SELECT
            tc.constraint_name,
            kcu.column_name,
            ccu.table_name AS foreign_table_name,
            ccu.column_name AS foreign_column_name,
            rc.delete_rule AS on_delete,
            rc.update_rule AS on_update
        FROM information_schema.table_constraints AS tc
        JOIN information_schema.key_column_usage AS kcu
          ON tc.constraint_name = kcu.constraint_name
          AND tc.table_schema = kcu.table_schema
        JOIN information_schema.constraint_column_usage AS ccu
          ON ccu.constraint_name = tc.constraint_name
          AND ccu.table_schema = tc.table_schema
        JOIN information_schema.referential_constraints AS rc
          ON tc.constraint_name = rc.constraint_name
          AND tc.table_schema = rc.constraint_schema
        WHERE tc.constraint_type = 'FOREIGN KEY'
          AND tc.table_schema = $1
          AND tc.table_name = $2",
    )
    .bind(schema)
    .bind(table_name)
    .fetch_all(pool)
    .await
    .context("Failed to fetch foreign keys")?;

    let foreign_keys = rows
        .iter()
        .map(|row| ForeignKeyInfo {
            constraint_name: row.get("constraint_name"),
            column_name: row.get("column_name"),
            foreign_table_name: row.get("foreign_table_name"),
            foreign_column_name: row.get("foreign_column_name"),
            on_delete: row.get("on_delete"),
            on_update: row.get("on_update"),
        })
        .collect();

    Ok(foreign_keys)
}

/// 获取表的索引信息
async fn get_indexes(pool: &PgPool, table_name: &str, schema: &str) -> Result<Vec<IndexInfo>> {
    let rows = sqlx::query(
        "SELECT
            i.relname AS index_name,
            ix.indisunique AS is_unique,
            ix.indisprimary AS is_primary,
            am.amname AS index_type,
            ARRAY_AGG(a.attname ORDER BY a.attnum) AS column_names
        FROM pg_class t
        JOIN pg_index ix ON t.oid = ix.indrelid
        JOIN pg_class i ON i.oid = ix.indexrelid
        JOIN pg_am am ON i.relam = am.oid
        JOIN pg_namespace n ON n.oid = t.relnamespace
        JOIN pg_attribute a ON a.attrelid = t.oid AND a.attnum = ANY(ix.indkey)
        WHERE t.relkind = 'r'
          AND n.nspname = $1
          AND t.relname = $2
        GROUP BY i.relname, ix.indisunique, ix.indisprimary, am.amname",
    )
    .bind(schema)
    .bind(table_name)
    .fetch_all(pool)
    .await
    .context("Failed to fetch indexes")?;

    let indexes = rows
        .iter()
        .map(|row| IndexInfo {
            name: row.get("index_name"),
            columns: row.get::<Vec<String>, _>("column_names"),
            is_unique: row.get("is_unique"),
            is_primary: row.get("is_primary"),
            index_type: row.get("index_type"),
        })
        .collect();

    Ok(indexes)
}

/// 获取表注释
async fn get_table_comment(
    pool: &PgPool,
    table_name: &str,
    schema: &str,
) -> Result<Option<String>> {
    let row = sqlx::query(
        "SELECT obj_description(pg_class.oid) AS comment
         FROM pg_class
         JOIN pg_namespace ON pg_namespace.oid = pg_class.relnamespace
         WHERE pg_namespace.nspname = $1 AND pg_class.relname = $2",
    )
    .bind(schema)
    .bind(table_name)
    .fetch_optional(pool)
    .await
    .context("Failed to fetch table comment")?;

    Ok(row.and_then(|r| r.get("comment")))
}

/// 获取列注释
async fn get_column_comment(
    pool: &PgPool,
    table_name: &str,
    schema: &str,
    column_name: &str,
) -> Result<Option<String>> {
    let row = sqlx::query(
        "SELECT col_description(
            (SELECT oid FROM pg_class WHERE relname = $2 AND relnamespace = (
                SELECT oid FROM pg_namespace WHERE nspname = $1
            )),
            (SELECT ordinal_position FROM information_schema.columns 
             WHERE table_schema = $1 AND table_name = $2 AND column_name = $3)
         ) AS comment",
    )
    .bind(schema)
    .bind(table_name)
    .bind(column_name)
    .fetch_optional(pool)
    .await
    .context("Failed to fetch column comment")?;

    Ok(row.and_then(|r| r.get("comment")))
}

/// 检查表是否存在
pub async fn table_exists(pool: &PgPool, table_name: &str, schema: &str) -> Result<bool> {
    let row = sqlx::query(
        "SELECT EXISTS (
            SELECT 1 FROM information_schema.tables 
            WHERE table_schema = $1 AND table_name = $2
        ) AS exists",
    )
    .bind(schema)
    .bind(table_name)
    .fetch_one(pool)
    .await
    .context("Failed to check table existence")?;

    Ok(row.get("exists"))
}

#[cfg(test)]
mod tests {
    use super::*;

    // 注意：这些测试需要实际的数据库连接
    // 可以使用环境变量 TEST_DATABASE_URL 来配置测试数据库

    #[tokio::test]
    #[ignore] // 默认忽略，需要数据库时手动运行
    async fn test_get_all_tables() {
        let database_url = std::env::var("TEST_DATABASE_URL")
            .unwrap_or_else(|_| "postgres://localhost/test_db".to_string());
        
        let pool = PgPool::connect(&database_url).await.unwrap();
        
        let tables = get_all_tables(&pool, None).await.unwrap();
        assert!(tables.len() >= 0); // 可能为空数据库
    }

    #[tokio::test]
    #[ignore]
    async fn test_table_exists() {
        let database_url = std::env::var("TEST_DATABASE_URL")
            .unwrap_or_else(|_| "postgres://localhost/test_db".to_string());
        
        let pool = PgPool::connect(&database_url).await.unwrap();
        
        let exists = table_exists(&pool, "nonexistent_table", "public").await.unwrap();
        assert!(!exists);
    }
}
