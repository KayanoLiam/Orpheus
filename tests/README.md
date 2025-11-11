# 测试说明

## 运行测试

### 前置条件
1. 确保 PostgreSQL 数据库运行
2. 确保 Redis 服务器运行
3. 设置测试环境变量（可选，有默认值）

### 环境变量
```bash
# 测试数据库 URL（可选，默认为 postgres://kayano:121381@localhost:5432/test_postgres）
export TEST_DATABASE_URL="postgres://username:password@localhost:5432/test_db"

# 测试 Redis URL（可选，默认为 redis://localhost:6379）
export TEST_REDIS_URL="redis://localhost:6379"
```

### 运行所有测试
```bash
cargo test
```

### 运行特定测试
```bash
# 运行用户处理器测试
cargo test user_handler

# 运行会话处理器测试
cargo test session_handler

# 运行会话存储测试
cargo test session_store

# 运行中间件测试
cargo test session_middleware
```

### 运行单个测试
```bash
cargo test test_user_signup_success
```

### 显示测试输出
```bash
cargo test -- --nocapture
```

## 测试覆盖范围

### 1. 用户处理器测试 (user_handler_tests.rs)
- `test_user_signup_success` - 测试成功注册
- `test_user_signup_duplicate_email` - 测试重复邮箱注册
- `test_user_signup_invalid_json` - 测试无效 JSON
- `test_user_signup_empty_fields` - 测试空字段
- `test_user_login_success` - 测试成功登录
- `test_user_login_invalid_email` - 测试无效邮箱登录
- `test_user_login_invalid_password` - 测试错误密码登录
- `test_user_login_missing_fields` - 测试缺少字段
- `test_user_login_empty_fields` - 测试空字段

### 2. 会话处理器测试 (session_handler_tests.rs)
- `test_user_logout_success` - 测试成功登出
- `test_user_logout_invalid_token` - 测试无效 token 登出
- `test_user_logout_no_token` - 测试无 token 登出
- `test_user_logout_expired_token` - 测试过期 token 登出
- `test_user_profile_success` - 测试成功获取用户资料
- `test_user_profile_invalid_token` - 测试无效 token 获取资料
- `test_user_profile_no_token` - 测试无 token 获取资料
- `test_user_profile_wrong_path` - 测试错误路径

### 3. 会话存储测试 (session_store_tests.rs)
- `test_session_store_new` - 测试创建 SessionStore
- `test_create_session_success` - 测试创建会话
- `test_create_session_multiple` - 测试创建多个会话
- `test_get_user_id_existing_session` - 测试获取存在的会话
- `test_get_user_id_nonexistent_session` - 测试获取不存在的会话
- `test_get_user_id_invalid_session_id` - 测试无效会话 ID
- `test_destroy_session_existing` - 测试销毁存在的会话
- `test_destroy_session_nonexistent` - 测试销毁不存在的会话
- `test_refresh_session_existing` - 测试刷新存在的会话
- `test_refresh_session_nonexistent` - 测试刷新不存在的会话
- `test_session_lifecycle` - 测试完整会话生命周期
- `test_concurrent_session_operations` - 测试并发操作

### 4. 中间件测试 (session_middleware_tests.rs)
- `test_session_validator_valid_token` - 测试有效 token
- `test_session_validator_invalid_token` - 测试无效 token
- `test_session_validator_no_token` - 测试无 token
- `test_session_validator_expired_token` - 测试过期 token
- `test_session_validator_no_redis_client` - 测试无 Redis 客户端
- `test_session_validator_malformed_token` - 测试格式错误的 token
- `test_session_validator_user_id_extension` - 测试用户 ID 扩展

## 测试数据清理

测试会自动清理创建的数据：
- 用户表中的测试邮箱记录（@test.com 结尾）
- Redis 中的测试会话

## 注意事项

1. 测试使用独立的测试数据库，避免影响开发数据
2. 每个测试都会生成唯一的 UUID，避免冲突
3. 测试失败时会保留部分数据以便调试
4. 并发测试确保线程安全性