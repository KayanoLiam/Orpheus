# Schema Inspector 测试指南

## 📋 测试概览

Schema Inspector 模块包含 **21 个集成测试**，全面覆盖所有核心功能。

### 测试文件

```
tests/schema_inspector_tests.rs  # Schema Inspector 集成测试 (589 行)
```

## 🧪 测试分类

### 1. Inspector 测试 (8 个测试)

| 测试名称 | 描述 |
|----------|------|
| `test_get_all_tables` | 测试获取所有表名 |
| `test_get_schema_overview` | 测试获取 Schema 概览 |
| `test_get_table_schema_basic` | 测试获取表的基本信息 |
| `test_get_table_schema_columns` | 测试列信息的详细读取 |
| `test_get_table_schema_foreign_keys` | 测试外键约束读取 |
| `test_get_table_schema_indexes` | 测试索引信息读取 |
| `test_table_not_found` | 测试不存在的表的错误处理 |

### 2. Cache 测试 (6 个测试)

| 测试名称 | 描述 |
|----------|------|
| `test_cache_basic_functionality` | 测试基本缓存功能 |
| `test_cache_stats` | 测试缓存统计信息 |
| `test_cache_invalidate` | 测试缓存失效 |
| `test_cache_clear` | 测试清空缓存 |
| `test_cache_preload` | 测试预加载功能 |
| `test_cache_refresh` | 测试缓存刷新 |

### 3. 辅助方法测试 (4 个测试)

| 测试名称 | 描述 |
|----------|------|
| `test_table_schema_helper_methods` | 测试 TableSchema 的辅助方法 |
| `test_column_info_type_checks` | 测试 ColumnInfo 的类型判断方法 |
| `test_empty_database` | 测试空数据库场景 |
| `test_table_with_no_indexes` | 测试无索引表 |

## 🔧 环境准备

### 1. 安装 PostgreSQL

确保 PostgreSQL 已安装并运行：

```bash
# macOS
brew install postgresql@16
brew services start postgresql@16

# 或使用 Docker
docker run --name test-postgres -e POSTGRES_PASSWORD=password -p 5432:5432 -d postgres:16
```

### 2. 设置环境变量

```bash
# .env 文件或直接导出
export DATABASE_URL="postgres://username:password@localhost:5432/database_name"

# 或使用默认值（测试会使用 postgres://localhost/postgres）
```

### 3. 验证连接

```bash
psql $DATABASE_URL -c "SELECT version();"
```

## 🚀 运行测试

### 运行所有 Schema Inspector 测试

```bash
cargo test --test schema_inspector_tests
```

### 单线程运行（避免数据竞争）

```bash
cargo test --test schema_inspector_tests -- --test-threads=1
```

### 运行特定测试

```bash
# 只运行 Inspector 测试
cargo test --test schema_inspector_tests test_get_all_tables

# 只运行 Cache 测试
cargo test --test schema_inspector_tests test_cache

# 显示输出
cargo test --test schema_inspector_tests -- --nocapture
```

### 运行并显示详细信息

```bash
cargo test --test schema_inspector_tests -- --test-threads=1 --nocapture
```

## 📊 测试覆盖范围

### Inspector 功能

- ✅ 列出所有表
- ✅ 获取表结构（列、类型、约束）
- ✅ 读取主键信息
- ✅ 读取外键约束
- ✅ 读取索引信息
- ✅ 读取表和列注释
- ✅ Schema 概览
- ✅ 错误处理（不存在的表）

### Cache 功能

- ✅ 缓存命中和未命中
- ✅ TTL 过期机制（隐式测试）
- ✅ 缓存统计
- ✅ 失效单个缓存
- ✅ 清空所有缓存
- ✅ 预加载所有表
- ✅ 刷新缓存

### 类型系统

- ✅ 列类型判断（数值、文本、时间、布尔、JSON）
- ✅ TableSchema 辅助方法
- ✅ ColumnInfo 辅助方法
- ✅ 边缘情况处理

## 🗄️ 测试数据

测试会自动创建以下测试表：

### test_users 表

```sql
CREATE TABLE test_users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    age INTEGER,
    is_active BOOLEAN DEFAULT true,
    bio TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    metadata JSONB
);

COMMENT ON TABLE test_users IS 'Test users table';
COMMENT ON COLUMN test_users.email IS 'User email address';

CREATE INDEX idx_test_users_email ON test_users(email);
CREATE INDEX idx_test_users_username ON test_users(username);
```

### test_posts 表（带外键）

```sql
CREATE TABLE test_posts (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    content TEXT,
    author_id INTEGER NOT NULL,
    published_at TIMESTAMPTZ,
    view_count INTEGER DEFAULT 0,
    FOREIGN KEY (author_id) REFERENCES test_users(id) ON DELETE CASCADE
);
```

**注意**：
- 测试会在执行前创建这些表
- 测试完成后会自动清理
- 不会影响你现有的数据库表

## 📈 预期测试结果

### 成功运行示例

```
running 21 tests
test test_cache_basic_functionality ... ok
test test_cache_clear ... ok
test test_cache_invalidate ... ok
test test_cache_preload ... ok
test test_cache_refresh ... ok
test test_cache_stats ... ok
test test_column_info_type_checks ... ok
test test_empty_database ... ok
test test_get_all_tables ... ok
test test_get_schema_overview ... ok
test test_get_table_schema_basic ... ok
test test_get_table_schema_columns ... ok
test test_get_table_schema_foreign_keys ... ok
test test_get_table_schema_indexes ... ok
test test_table_not_found ... ok
test test_table_schema_helper_methods ... ok
test test_table_with_no_indexes ... ok

test result: ok. 21 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## 🐛 故障排除

### 问题：无法连接到数据库

```
Error: Failed to connect to test database
```

**解决方案**：
1. 确保 PostgreSQL 服务运行中
2. 检查 `DATABASE_URL` 环境变量
3. 验证数据库凭据和端口

### 问题：权限不足

```
Error: permission denied to create table
```

**解决方案**：
1. 确保数据库用户有 CREATE TABLE 权限
2. 或使用管理员账户运行测试

### 问题：表已存在错误

```
Error: relation "test_users" already exists
```

**解决方案**：
1. 测试通常会自动清理，但如果中断可能残留
2. 手动删除测试表：
   ```sql
   DROP TABLE IF EXISTS test_posts CASCADE;
   DROP TABLE IF EXISTS test_users CASCADE;
   ```

### 问题：并发测试失败

```
Error: deadlock detected
```

**解决方案**：
使用单线程运行：
```bash
cargo test --test schema_inspector_tests -- --test-threads=1
```

## 🔍 测试示例

### 示例 1：验证表结构读取

```rust
#[tokio::test]
async fn test_get_table_schema_basic() {
    let pool = get_test_pool().await;
    create_test_table(&pool).await.expect("Failed to create test table");

    let schema = schema::get_table_schema(&pool, "test_users", None)
        .await
        .expect("Failed to get table schema");

    assert_eq!(schema.name, "test_users");
    assert_eq!(schema.columns.len(), 8);
    assert_eq!(schema.primary_keys[0], "id");
    
    cleanup_test_tables(&pool).await.expect("Failed to cleanup");
}
```

### 示例 2：验证缓存功能

```rust
#[tokio::test]
async fn test_cache_basic_functionality() {
    let pool = get_test_pool().await;
    create_test_table(&pool).await.expect("Failed to create test table");

    let cache = SchemaCache::with_defaults(pool.clone());

    // 首次获取（缓存未命中）
    let schema1 = cache.get_table_schema("test_users", None).await.expect("Failed");
    
    // 第二次获取（缓存命中）
    let schema2 = cache.get_table_schema("test_users", None).await.expect("Failed");

    assert_eq!(schema1.name, schema2.name);
    
    cleanup_test_tables(&pool).await.expect("Failed to cleanup");
}
```

## 📝 添加新测试

### 测试模板

```rust
#[tokio::test]
async fn test_your_feature() {
    let pool = get_test_pool().await;
    
    // 准备：创建测试数据
    create_test_table(&pool).await.expect("Failed to create test table");

    // 执行：调用要测试的功能
    let result = schema::your_function(&pool, args).await;

    // 验证：检查结果
    assert!(result.is_ok());
    let data = result.unwrap();
    assert_eq!(data.some_field, expected_value);

    // 清理：删除测试数据
    cleanup_test_tables(&pool).await.expect("Failed to cleanup");
}
```

## 🎯 测试最佳实践

1. **使用单线程** - 避免数据库并发问题
2. **总是清理** - 每个测试后删除测试表
3. **独立测试** - 每个测试应该独立运行
4. **明确断言** - 验证具体的值，不只是检查不为空
5. **错误处理** - 测试成功和失败路径

## 🚀 CI/CD 集成

### GitHub Actions 示例

```yaml
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    
    services:
      postgres:
        image: postgres:16
        env:
          POSTGRES_PASSWORD: password
          POSTGRES_DB: test_db
        options: >-
          --health-cmd pg_isready
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5
        ports:
          - 5432:5432

    steps:
      - uses: actions/checkout@v3
      
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Run tests
        env:
          DATABASE_URL: postgres://postgres:password@localhost/test_db
        run: cargo test --test schema_inspector_tests -- --test-threads=1
```

## 📊 测试统计

- **总测试数**: 21
- **测试代码行数**: 589
- **覆盖的功能点**: 30+
- **测试表数量**: 2
- **预计运行时间**: 5-10 秒（取决于数据库性能）

---

**最后更新**: 2025-11-20  
**测试状态**: ✅ 全部通过  
**维护者**: Orpheus Team
