# Authentication Example

这是一个使用 Orpheus 构建的用户认证系统示例。

## 概述

此示例展示了如何使用 Orpheus BaaS 平台构建一个完整的用户认证系统，包括：

- 用户注册 (signup)
- 用户登录 (login)
- 用户登出 (logout)
- 会话管理 (session management)
- 密码重置 (password reset)
- 用户删除 (user deletion)

## 技术栈

- **后端**: Actix-Web + PostgreSQL + Redis
- **密码哈希**: Argon2
- **会话存储**: Redis
- **认证方式**: Bearer Token

## 文件结构

```
examples/authentication/
├── src/
│   ├── handlers/
│   │   ├── user_handler.rs      # 用户相关操作（注册、登录等）
│   │   └── session_handler.rs   # 会话管理（登出、获取用户信息）
│   ├── models/
│   │   ├── user.rs               # 用户数据模型
│   │   └── session.rs            # 会话数据模型
│   ├── auth/
│   │   ├── session_store.rs      # Redis 会话存储
│   │   └── status.rs             # 认证状态检查
│   ├── middlewares/
│   │   └── session.rs            # 会话验证中间件
│   └── config.rs                 # 配置常量
└── README.md                     # 本文件
```

## 数据库 Schema

```sql
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(255) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_username ON users(username);
```

## API 端点

### 公开端点

- `POST /signup` - 用户注册
  ```json
  {
    "username": "string",
    "email": "string",
    "password": "string"
  }
  ```

- `POST /login` - 用户登录
  ```json
  {
    "email": "string",
    "password": "string"
  }
  ```

- `POST /reset-password` - 密码重置
  ```json
  {
    "email": "string",
    "new_password": "string"
  }
  ```

- `DELETE /delete-user` - 用户删除
  ```json
  {
    "email": "string",
    "password": "string"
  }
  ```

### 受保护端点（需要 Bearer Token）

- `POST /logout` - 用户登出
  - Header: `Authorization: Bearer <session_id>`

- `GET /api/profile` - 获取用户信息
  - Header: `Authorization: Bearer <session_id>`

- `GET /auth/status` - 检查认证状态
  - Header: `Authorization: <session_id>`

## 使用方法

### 1. 集成到 Orpheus 主项目

这些代码可以直接集成到 Orpheus 主项目中作为认证模块使用。

### 2. 作为独立服务运行

创建一个新的 `main.rs` 并引用这些模块：

```rust
// examples/authentication/src/main.rs

mod handlers;
mod models;
mod auth;
mod middlewares;
mod config;

use actix_web::{web, App, HttpServer};
// ... 其他导入

#[tokio::main]
async fn main() -> Result<()> {
    // 数据库连接
    let pool = PgPool::connect(&database_url).await?;
    
    // Redis 连接
    let client = redis::Client::open(redis_url)?;
    
    // 启动服务器
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(client.clone()))
            .service(handlers::user_handler::user_signup)
            .service(handlers::user_handler::user_login)
            .service(handlers::session_handler::user_logout)
            // ... 其他路由
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

## 会话管理

### 会话存储

会话使用 Redis 存储，格式：
- Key: `session:user_{uuid}`
- Value: JSON 格式的用户 ID
- TTL: 3600 秒（1 小时）

### 会话刷新

每次有效的 API 调用都会刷新会话过期时间。

## 安全特性

1. **密码哈希**: 使用 Argon2 算法 + 随机 salt
2. **会话过期**: 自动过期机制（1 小时）
3. **Bearer Token**: 基于 Token 的认证
4. **输入验证**: 验证邮箱格式、密码强度等

## 扩展建议

### 添加邮箱验证

1. 创建 `email_verifications` 表
2. 注册时发送验证邮件
3. 用户点击链接验证邮箱

### 添加 OAuth 支持

1. 集成 GitHub OAuth
2. 集成 Google OAuth
3. 统一的用户表（支持多种登录方式）

### 添加双因素认证 (2FA)

1. TOTP 生成器
2. 备用恢复码
3. 2FA 验证中间件

## 测试

相关的测试文件位于项目根目录的 `tests/` 目录：

- `user_handler_tests.rs` - 用户操作测试
- `session_handler_tests.rs` - 会话管理测试
- `session_store_tests.rs` - 会话存储测试
- `session_middleware_tests.rs` - 中间件测试

运行测试：
```bash
cargo test user_handler
cargo test session_handler
```

## 注意事项

1. 此示例是从 Orpheus 主项目中提取的，**不是 Orpheus 的核心功能**
2. Orpheus 的核心是 **自动 REST API 生成**、**实时订阅**、**对象存储** 等 BaaS 功能
3. 用户认证可以基于 Orpheus 的核心功能实现，而不需要手写这些代码

## 许可证

MIT License
