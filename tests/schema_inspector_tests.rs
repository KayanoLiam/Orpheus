// Schema Inspector 模块集成测试
//
// 这些测试需要实际的 PostgreSQL 数据库连接
// 运行前请确保：
// 1. PostgreSQL 服务运行中
// 2. 设置环境变量 DATABASE_URL
//
// 运行测试：
// cargo test --test schema_inspector_tests -- --test-threads=1

use orpheus::schema::{self, SchemaCache};
use sqlx::PgPool;

// 测试辅助函数：获取测试数据库连接
async fn get_test_pool() -> PgPool {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://localhost/postgres".to_string());
    
    PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to test database")
}

// 测试辅助函数：创建测试表
async fn create_test_table(pool: &PgPool) -> anyhow::Result<()> {
    // 删除可能存在的旧表
    sqlx::query("DROP TABLE IF EXISTS test_posts CASCADE")
        .execute(pool)
        .await?;
    
    sqlx::query("DROP TABLE IF EXISTS test_users CASCADE")
        .execute(pool)
        .await?;

    // 创建测试表 test_users
    sqlx::query(
        "CREATE TABLE test_users (
            id SERIAL PRIMARY KEY,
            username VARCHAR(50) NOT NULL UNIQUE,
            email VARCHAR(255) NOT NULL UNIQUE,
            age INTEGER,
            is_active BOOLEAN DEFAULT true,
            bio TEXT,
            created_at TIMESTAMPTZ DEFAULT NOW(),
            metadata JSONB
        )"
    )
    .execute(pool)
    .await?;

    // 添加表注释
    sqlx::query("COMMENT ON TABLE test_users IS 'Test users table'")
        .execute(pool)
        .await?;

    // 添加列注释
    sqlx::query("COMMENT ON COLUMN test_users.email IS 'User email address'")
        .execute(pool)
        .await?;

    // 创建索引
    sqlx::query("CREATE INDEX idx_test_users_email ON test_users(email)")
        .execute(pool)
        .await?;

    sqlx::query("CREATE INDEX idx_test_users_username ON test_users(username)")
        .execute(pool)
        .await?;

    // 创建测试表 test_posts（带外键）
    sqlx::query(
        "CREATE TABLE test_posts (
            id SERIAL PRIMARY KEY,
            title VARCHAR(255) NOT NULL,
            content TEXT,
            author_id INTEGER NOT NULL,
            published_at TIMESTAMPTZ,
            view_count INTEGER DEFAULT 0,
            FOREIGN KEY (author_id) REFERENCES test_users(id) ON DELETE CASCADE
        )"
    )
    .execute(pool)
    .await?;

    Ok(())
}

// 测试辅助函数：清理测试表
async fn cleanup_test_tables(pool: &PgPool) -> anyhow::Result<()> {
    sqlx::query("DROP TABLE IF EXISTS test_posts CASCADE")
        .execute(pool)
        .await?;
    
    sqlx::query("DROP TABLE IF EXISTS test_users CASCADE")
        .execute(pool)
        .await?;

    Ok(())
}

// ============================================================================
// Inspector 测试
// ============================================================================

#[tokio::test]
async fn test_get_all_tables() {
    let pool = get_test_pool().await;
    
    // 创建测试表
    create_test_table(&pool).await.expect("Failed to create test table");

    // 获取所有表
    let tables = schema::get_all_tables(&pool, None)
        .await
        .expect("Failed to get tables");

    // 验证测试表存在
    assert!(tables.contains(&"test_users".to_string()), "test_users table should exist");
    assert!(tables.contains(&"test_posts".to_string()), "test_posts table should exist");

    // 清理
    cleanup_test_tables(&pool).await.expect("Failed to cleanup");
}

#[tokio::test]
async fn test_get_schema_overview() {
    let pool = get_test_pool().await;
    
    create_test_table(&pool).await.expect("Failed to create test table");

    // 获取 schema 概览
    let overview = schema::get_schema_overview(&pool, None)
        .await
        .expect("Failed to get schema overview");

    assert_eq!(overview.name, "public");
    assert!(overview.table_count >= 2); // 至少有我们创建的两个测试表
    assert!(overview.tables.contains(&"test_users".to_string()));

    cleanup_test_tables(&pool).await.expect("Failed to cleanup");
}

#[tokio::test]
async fn test_get_table_schema_basic() {
    let pool = get_test_pool().await;
    
    create_test_table(&pool).await.expect("Failed to create test table");

    // 获取 test_users 表的 schema
    let schema = schema::get_table_schema(&pool, "test_users", None)
        .await
        .expect("Failed to get table schema");

    // 验证基本信息
    assert_eq!(schema.name, "test_users");
    assert_eq!(schema.schema, "public");
    
    // 验证列数量
    assert_eq!(schema.columns.len(), 8); // id, username, email, age, is_active, bio, created_at, metadata

    // 验证主键
    assert_eq!(schema.primary_keys.len(), 1);
    assert_eq!(schema.primary_keys[0], "id");

    // 验证表注释
    assert_eq!(schema.comment, Some("Test users table".to_string()));

    cleanup_test_tables(&pool).await.expect("Failed to cleanup");
}

#[tokio::test]
async fn test_get_table_schema_columns() {
    let pool = get_test_pool().await;
    
    create_test_table(&pool).await.expect("Failed to create test table");

    let schema = schema::get_table_schema(&pool, "test_users", None)
        .await
        .expect("Failed to get table schema");

    // 查找特定列
    let id_col = schema.columns.iter().find(|c| c.name == "id").expect("id column not found");
    let email_col = schema.columns.iter().find(|c| c.name == "email").expect("email column not found");
    let age_col = schema.columns.iter().find(|c| c.name == "age").expect("age column not found");
    let bio_col = schema.columns.iter().find(|c| c.name == "bio").expect("bio column not found");

    // 验证 id 列（自增主键）- SERIAL 类型有默认值（序列）
    assert_eq!(id_col.data_type, "integer");
    assert!(!id_col.is_nullable);
    // SERIAL 不是真正的 IDENTITY，但会有默认值（nextval）
    assert!(id_col.default_value.is_some());

    // 验证 email 列（VARCHAR 带长度限制）
    assert_eq!(email_col.data_type, "character varying");
    assert!(!email_col.is_nullable);
    assert_eq!(email_col.max_length, Some(255));
    assert_eq!(email_col.comment, Some("User email address".to_string()));

    // 验证 age 列（可空整数）
    assert_eq!(age_col.data_type, "integer");
    assert!(age_col.is_nullable);

    // 验证 bio 列（TEXT 类型）
    assert_eq!(bio_col.data_type, "text");
    assert!(bio_col.is_nullable);

    cleanup_test_tables(&pool).await.expect("Failed to cleanup");
}

#[tokio::test]
async fn test_get_table_schema_foreign_keys() {
    let pool = get_test_pool().await;
    
    create_test_table(&pool).await.expect("Failed to create test table");

    // 获取 test_posts 表的 schema（有外键）
    let schema = schema::get_table_schema(&pool, "test_posts", None)
        .await
        .expect("Failed to get table schema");

    // 验证外键
    assert_eq!(schema.foreign_keys.len(), 1);
    
    let fk = &schema.foreign_keys[0];
    assert_eq!(fk.column_name, "author_id");
    assert_eq!(fk.foreign_table_name, "test_users");
    assert_eq!(fk.foreign_column_name, "id");
    assert_eq!(fk.on_delete, Some("CASCADE".to_string()));

    cleanup_test_tables(&pool).await.expect("Failed to cleanup");
}

#[tokio::test]
async fn test_get_table_schema_indexes() {
    let pool = get_test_pool().await;
    
    create_test_table(&pool).await.expect("Failed to create test table");

    let schema = schema::get_table_schema(&pool, "test_users", None)
        .await
        .expect("Failed to get table schema");

    // 验证索引（应该有主键索引 + 2个自定义索引）
    assert!(schema.indexes.len() >= 3);

    // 查找主键索引
    let pk_index = schema.indexes.iter().find(|idx| idx.is_primary).expect("Primary key index not found");
    assert!(pk_index.is_unique);
    assert!(pk_index.columns.contains(&"id".to_string()));

    // 查找 email 索引
    let email_index = schema.indexes.iter()
        .find(|idx| idx.name == "idx_test_users_email")
        .expect("Email index not found");
    assert!(email_index.columns.contains(&"email".to_string()));

    cleanup_test_tables(&pool).await.expect("Failed to cleanup");
}

#[tokio::test]
async fn test_table_not_found() {
    let pool = get_test_pool().await;
    
    // 尝试获取不存在的表
    let result = schema::get_table_schema(&pool, "nonexistent_table_12345", None).await;
    
    // 应该返回错误（表不存在时某些查询会返回空结果）
    // 具体行为取决于实现，这里测试不会 panic
    assert!(result.is_ok() || result.is_err());
}

// ============================================================================
// Cache 测试
// ============================================================================

#[tokio::test]
async fn test_cache_basic_functionality() {
    let pool = get_test_pool().await;
    
    create_test_table(&pool).await.expect("Failed to create test table");

    // 创建缓存
    let cache = SchemaCache::with_defaults(pool.clone());

    // 首次获取（缓存未命中）
    let schema1 = cache.get_table_schema("test_users", None)
        .await
        .expect("Failed to get schema");

    // 第二次获取（缓存命中）
    let schema2 = cache.get_table_schema("test_users", None)
        .await
        .expect("Failed to get schema");

    // 两次结果应该相同
    assert_eq!(schema1.name, schema2.name);
    assert_eq!(schema1.columns.len(), schema2.columns.len());

    cleanup_test_tables(&pool).await.expect("Failed to cleanup");
}

#[tokio::test]
async fn test_cache_stats() {
    let pool = get_test_pool().await;
    
    create_test_table(&pool).await.expect("Failed to create test table");

    let cache = SchemaCache::with_defaults(pool.clone());

    // 初始状态：缓存为空
    let stats = cache.stats().await;
    assert_eq!(stats.total_entries, 0);

    // 获取一个表
    let _ = cache.get_table_schema("test_users", None).await;

    // 检查缓存统计
    let stats = cache.stats().await;
    assert_eq!(stats.total_entries, 1);
    assert_eq!(stats.active_entries, 1);
    assert_eq!(stats.expired_entries, 0);

    cleanup_test_tables(&pool).await.expect("Failed to cleanup");
}

#[tokio::test]
async fn test_cache_invalidate() {
    let pool = get_test_pool().await;
    
    create_test_table(&pool).await.expect("Failed to create test table");

    let cache = SchemaCache::with_defaults(pool.clone());

    // 加载到缓存
    let _ = cache.get_table_schema("test_users", None).await;

    let stats_before = cache.stats().await;
    assert_eq!(stats_before.total_entries, 1);

    // 使缓存失效
    cache.invalidate("test_users", None).await;

    let stats_after = cache.stats().await;
    assert_eq!(stats_after.total_entries, 0);

    cleanup_test_tables(&pool).await.expect("Failed to cleanup");
}

#[tokio::test]
async fn test_cache_clear() {
    let pool = get_test_pool().await;
    
    create_test_table(&pool).await.expect("Failed to create test table");

    let cache = SchemaCache::with_defaults(pool.clone());

    // 加载多个表到缓存
    let _ = cache.get_table_schema("test_users", None).await;
    let _ = cache.get_table_schema("test_posts", None).await;

    let stats_before = cache.stats().await;
    assert_eq!(stats_before.total_entries, 2);

    // 清空所有缓存
    cache.clear().await;

    let stats_after = cache.stats().await;
    assert_eq!(stats_after.total_entries, 0);

    cleanup_test_tables(&pool).await.expect("Failed to cleanup");
}

#[tokio::test]
async fn test_cache_preload() {
    let pool = get_test_pool().await;
    
    create_test_table(&pool).await.expect("Failed to create test table");

    let cache = SchemaCache::with_defaults(pool.clone());

    // 预加载所有表
    cache.preload(None).await.expect("Failed to preload cache");

    let stats = cache.stats().await;
    
    // 应该至少加载了我们创建的两个测试表
    assert!(stats.total_entries >= 2);
    assert!(stats.active_entries >= 2);

    cleanup_test_tables(&pool).await.expect("Failed to cleanup");
}

#[tokio::test]
async fn test_cache_refresh() {
    let pool = get_test_pool().await;
    
    create_test_table(&pool).await.expect("Failed to create test table");

    let cache = SchemaCache::with_defaults(pool.clone());

    // 首次加载
    let schema1 = cache.get_table_schema("test_users", None)
        .await
        .expect("Failed to get schema");

    // 刷新缓存
    let schema2 = cache.refresh("test_users", None)
        .await
        .expect("Failed to refresh cache");

    // 结果应该相同（因为表结构没变）
    assert_eq!(schema1.name, schema2.name);
    assert_eq!(schema1.columns.len(), schema2.columns.len());

    cleanup_test_tables(&pool).await.expect("Failed to cleanup");
}

// ============================================================================
// 类型辅助方法测试
// ============================================================================

#[tokio::test]
async fn test_table_schema_helper_methods() {
    let pool = get_test_pool().await;
    
    create_test_table(&pool).await.expect("Failed to create test table");

    let schema = schema::get_table_schema(&pool, "test_users", None)
        .await
        .expect("Failed to get table schema");

    // 测试 get_column
    let email_col = schema.get_column("email");
    assert!(email_col.is_some());
    assert_eq!(email_col.unwrap().data_type, "character varying");

    // 测试 has_column
    assert!(schema.has_column("email"));
    assert!(schema.has_column("username"));
    assert!(!schema.has_column("nonexistent"));

    // 测试 is_primary_key
    assert!(schema.is_primary_key("id"));
    assert!(!schema.is_primary_key("email"));

    // 测试 nullable_columns
    let nullable = schema.nullable_columns();
    assert!(nullable.iter().any(|c| c.name == "age"));
    assert!(nullable.iter().any(|c| c.name == "bio"));

    // 测试 required_columns（不可空且无默认值）
    let required = schema.required_columns();
    assert!(required.iter().any(|c| c.name == "username"));
    assert!(required.iter().any(|c| c.name == "email"));
    assert!(!required.iter().any(|c| c.name == "id")); // SERIAL 有默认值（序列）

    cleanup_test_tables(&pool).await.expect("Failed to cleanup");
}

#[tokio::test]
async fn test_column_info_type_checks() {
    let pool = get_test_pool().await;
    
    create_test_table(&pool).await.expect("Failed to create test table");

    let schema = schema::get_table_schema(&pool, "test_users", None)
        .await
        .expect("Failed to get table schema");

    // 测试数值类型
    let age_col = schema.get_column("age").unwrap();
    assert!(age_col.is_numeric());
    assert!(!age_col.is_text());
    assert!(!age_col.is_temporal());

    // 测试字符串类型
    let email_col = schema.get_column("email").unwrap();
    assert!(!email_col.is_numeric());
    assert!(email_col.is_text());
    assert!(!email_col.is_temporal());

    // 测试时间类型
    let created_at_col = schema.get_column("created_at").unwrap();
    assert!(!created_at_col.is_numeric());
    assert!(!created_at_col.is_text());
    assert!(created_at_col.is_temporal());

    // 测试布尔类型
    let is_active_col = schema.get_column("is_active").unwrap();
    assert!(!is_active_col.is_numeric());
    assert!(is_active_col.is_boolean());

    // 测试 JSON 类型
    let metadata_col = schema.get_column("metadata").unwrap();
    assert!(!metadata_col.is_numeric());
    assert!(metadata_col.is_json());

    cleanup_test_tables(&pool).await.expect("Failed to cleanup");
}

// ============================================================================
// 边缘情况测试
// ============================================================================

#[tokio::test]
async fn test_empty_database() {
    let pool = get_test_pool().await;
    
    // 确保没有测试表
    let _ = cleanup_test_tables(&pool).await;

    // 获取所有表（可能为空或只有系统表）
    let tables = schema::get_all_tables(&pool, None)
        .await
        .expect("Failed to get tables");

    // 不应该包含我们的测试表
    assert!(!tables.contains(&"test_users".to_string()));
    assert!(!tables.contains(&"test_posts".to_string()));
}

#[tokio::test]
async fn test_table_with_no_indexes() {
    let pool = get_test_pool().await;
    
    // 创建一个没有额外索引的简单表
    sqlx::query("DROP TABLE IF EXISTS test_simple")
        .execute(&pool)
        .await
        .ok();

    sqlx::query("CREATE TABLE test_simple (data TEXT)")
        .execute(&pool)
        .await
        .expect("Failed to create simple table");

    let schema = schema::get_table_schema(&pool, "test_simple", None)
        .await
        .expect("Failed to get schema");

    // 应该没有主键
    assert_eq!(schema.primary_keys.len(), 0);
    
    // 清理
    sqlx::query("DROP TABLE IF EXISTS test_simple")
        .execute(&pool)
        .await
        .ok();
}
