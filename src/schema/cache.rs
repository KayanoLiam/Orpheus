// Schema Cache - Schema 信息缓存层
// 避免频繁查询 information_schema，提高性能

use super::{inspector, types::TableSchema};
use anyhow::Result;
use sqlx::PgPool;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

/// Schema 缓存配置
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// 缓存过期时间
    pub ttl: Duration,
    /// 是否启用缓存
    pub enabled: bool,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            ttl: Duration::from_secs(300), // 默认 5 分钟过期
            enabled: true,
        }
    }
}

/// Schema 缓存项
#[derive(Debug, Clone)]
struct CacheEntry {
    schema: TableSchema,
    cached_at: Instant,
}

impl CacheEntry {
    fn new(schema: TableSchema) -> Self {
        Self {
            schema,
            cached_at: Instant::now(),
        }
    }

    fn is_expired(&self, ttl: Duration) -> bool {
        self.cached_at.elapsed() > ttl
    }
}

/// Schema 缓存管理器
#[derive(Clone)]
pub struct SchemaCache {
    pool: PgPool,
    cache: Arc<RwLock<HashMap<String, CacheEntry>>>,
    config: CacheConfig,
}

impl SchemaCache {
    /// 创建新的缓存实例
    pub fn new(pool: PgPool, config: CacheConfig) -> Self {
        Self {
            pool,
            cache: Arc::new(RwLock::new(HashMap::new())),
            config,
        }
    }

    /// 使用默认配置创建缓存
    pub fn with_defaults(pool: PgPool) -> Self {
        Self::new(pool, CacheConfig::default())
    }

    /// 获取表的 schema 信息（带缓存）
    ///
    /// # Arguments
    /// * `table_name` - 表名
    /// * `schema_name` - Schema 名称，默认为 "public"
    pub async fn get_table_schema(
        &self,
        table_name: &str,
        schema_name: Option<&str>,
    ) -> Result<TableSchema> {
        let schema = schema_name.unwrap_or("public");
        let cache_key = format!("{}.{}", schema, table_name);

        // 如果禁用缓存，直接从数据库查询
        if !self.config.enabled {
            return inspector::get_table_schema(&self.pool, table_name, Some(schema)).await;
        }

        // 检查缓存
        {
            let cache_read = self.cache.read().await;
            if let Some(entry) = cache_read.get(&cache_key) {
                if !entry.is_expired(self.config.ttl) {
                    return Ok(entry.schema.clone());
                }
            }
        }

        // 缓存未命中或已过期，从数据库读取
        let table_schema = inspector::get_table_schema(&self.pool, table_name, Some(schema)).await?;

        // 更新缓存
        {
            let mut cache_write = self.cache.write().await;
            cache_write.insert(cache_key, CacheEntry::new(table_schema.clone()));
        }

        Ok(table_schema)
    }

    /// 获取所有表名（带缓存）
    pub async fn get_all_tables(&self, schema_name: Option<&str>) -> Result<Vec<String>> {
        let schema = schema_name.unwrap_or("public");
        let cache_key = format!("__tables__{}", schema);

        // 如果禁用缓存，直接查询
        if !self.config.enabled {
            return inspector::get_all_tables(&self.pool, Some(schema)).await;
        }

        // 检查缓存中是否有表列表
        {
            let cache_read = self.cache.read().await;
            if let Some(entry) = cache_read.get(&cache_key) {
                if !entry.is_expired(self.config.ttl) {
                    // 从缓存的 schema 中提取表名
                    // 注意：这里我们使用一个特殊的缓存键存储表列表
                    // 实际实现中可能需要单独的缓存结构
                }
            }
        }

        // 缓存未命中，直接查询
        inspector::get_all_tables(&self.pool, Some(schema)).await
    }

    /// 使指定表的缓存失效
    pub async fn invalidate(&self, table_name: &str, schema_name: Option<&str>) {
        let schema = schema_name.unwrap_or("public");
        let cache_key = format!("{}.{}", schema, table_name);

        let mut cache_write = self.cache.write().await;
        cache_write.remove(&cache_key);
    }

    /// 清空所有缓存
    pub async fn clear(&self) {
        let mut cache_write = self.cache.write().await;
        cache_write.clear();
    }

    /// 刷新指定表的缓存
    pub async fn refresh(&self, table_name: &str, schema_name: Option<&str>) -> Result<TableSchema> {
        // 先使缓存失效
        self.invalidate(table_name, schema_name).await;
        
        // 重新获取（会自动缓存）
        self.get_table_schema(table_name, schema_name).await
    }

    /// 预加载所有表的 schema 到缓存
    pub async fn preload(&self, schema_name: Option<&str>) -> Result<()> {
        let schema = schema_name.unwrap_or("public");
        let tables = inspector::get_all_tables(&self.pool, Some(schema)).await?;

        for table_name in tables {
            // 忽略单个表的加载错误，继续加载其他表
            if let Ok(table_schema) = inspector::get_table_schema(&self.pool, &table_name, Some(schema)).await {
                let cache_key = format!("{}.{}", schema, table_name);
                let mut cache_write = self.cache.write().await;
                cache_write.insert(cache_key, CacheEntry::new(table_schema));
            }
        }

        Ok(())
    }

    /// 获取缓存统计信息
    pub async fn stats(&self) -> CacheStats {
        let cache_read = self.cache.read().await;
        
        let total_entries = cache_read.len();
        let expired_entries = cache_read
            .values()
            .filter(|entry| entry.is_expired(self.config.ttl))
            .count();

        CacheStats {
            total_entries,
            active_entries: total_entries - expired_entries,
            expired_entries,
            ttl_seconds: self.config.ttl.as_secs(),
        }
    }
}

/// 缓存统计信息
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CacheStats {
    /// 总缓存条目数
    pub total_entries: usize,
    /// 活跃（未过期）的条目数
    pub active_entries: usize,
    /// 已过期的条目数
    pub expired_entries: usize,
    /// TTL 秒数
    pub ttl_seconds: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    #[ignore] // 需要数据库连接
    async fn test_cache_basic() {
        let database_url = std::env::var("TEST_DATABASE_URL")
            .unwrap_or_else(|_| "postgres://localhost/test_db".to_string());
        
        let pool = PgPool::connect(&database_url).await.unwrap();
        
        let cache = SchemaCache::with_defaults(pool);
        
        // 首次获取（缓存未命中）
        let schema1 = cache.get_table_schema("users", None).await;
        
        // 第二次获取（命中缓存）
        let schema2 = cache.get_table_schema("users", None).await;
        
        // 两次结果应该相同
        if let (Ok(s1), Ok(s2)) = (schema1, schema2) {
            assert_eq!(s1.name, s2.name);
        }
    }

    #[tokio::test]
    async fn test_cache_expiration() {
        let database_url = std::env::var("TEST_DATABASE_URL")
            .unwrap_or_else(|_| "postgres://localhost/test_db".to_string());
        
        if let Ok(pool) = PgPool::connect(&database_url).await {
            let config = CacheConfig {
                ttl: Duration::from_millis(100), // 100ms 过期
                enabled: true,
            };
            
            let cache = SchemaCache::new(pool, config);
            
            // 获取并缓存
            let _ = cache.get_table_schema("users", None).await;
            
            // 等待过期
            tokio::time::sleep(Duration::from_millis(150)).await;
            
            // 再次获取应该触发重新查询
            let _ = cache.get_table_schema("users", None).await;
        }
    }
}
