# Orpheus

## 项目概述

Orpheus 是一个基于 Rust 生态构建的类 Supabase 的后端服务系统，旨在为 Web 应用提供开箱即用的身份认证、会话管理与数据访问能力。项目采用 Actix-Web 作为核心 Web 框架，PostgreSQL 作为主数据存储，Redis 用于会话存储与缓存加速，并提供清晰的 API 接口设计，便于前端应用直接调用。

本项目的目标是构建一个可扩展、可自托管、具备良好安全性和性能的后端服务基础设施。

## 核心功能

### 用户系统
- 用户注册、登录、注销
- 支持密码哈希与安全验证（基于 Argon2）
- 支持 Token / Session 两种会话模式（可扩展）

### 会话管理
- Redis 存储会话，提高性能与可恢复性
- 会话自动过期、刷新机制
- 中间件统一鉴权处理

### 数据库层
- PostgreSQL 作为主数据库
- 统一的连接池与错误处理抽象
- 可平滑迁移数据库（预留 migration 支持）

### API 设计
- RESTful 风格接口
- 结构化 JSON 响应格式
- 清晰的错误与状态码规范

### 代码质量保证
- 严格的类型与错误处理策略
- 完整测试套件（单元测试 + 集成测试）
- 模块化架构，易于开发与维护

## 技术栈

| 组件 | 描述 |
|------|------|
| Rust | 主语言，强调安全与性能 |
| Actix-Web | 高性能 Web 框架 |
| PostgreSQL | 主数据存储（关系型数据库） |
| Redis | 会话 / 缓存存储 |
| Argon2 | 密码哈希算法 |
| Docker（可选） | 部署与环境隔离 |

## 项目结构

```
orpheus/
├── src/
│   ├── main.rs              # 应用程序入口点
│   ├── config.rs            # 配置常量
│   ├── auth/                # 认证模块
│   │   └── session_store.rs # Redis 会话管理
│   ├── handlers/            # HTTP 请求处理器
│   │   ├── user_handler.rs  # 用户相关接口
│   │   └── session_handler.rs # 会话相关接口
│   ├── middlewares/         # 中间件
│   │   └── session.rs       # 会话验证中间件
│   └── models/              # 数据模型
│       ├── user.rs          # 用户模型
│       ├── session.rs       # 会话模型
│       └── response.rs      # API 响应模型
├── tests/                   # 测试套件
│   ├── user_handler_tests.rs
│   ├── session_handler_tests.rs
│   ├── session_store_tests.rs
│   └── session_middleware_tests.rs
├── .env                     # 环境变量配置
├── Cargo.toml               # 项目依赖配置
└── README.md               # 项目说明文档
```

## 快速开始

### 环境要求
- Rust 1.70+
- PostgreSQL 12+
- Redis 6+

### 安装步骤

1. **克隆项目**
   ```bash
   git clone https://github.com/yourusername/orpheus.git
   cd orpheus
   ```

2. **配置环境变量**
   创建 `.env` 文件并配置以下变量：
   ```bash
   DATABASE_URL=postgres://用户名:密码@localhost:5432/数据库名
   REDIS_URL=redis://localhost:6379
   ```

3. **构建项目**
   ```bash
   cargo build --release
   ```

4. **运行服务**
   ```bash
   cargo run
   ```

   服务将在 `http://127.0.0.1:8080` 启动

   > **推荐**：对于本地开发，请使用 `cargo run` 而不是 Docker，以获得更好的开发体验。

## API 文档

### 公开端点

#### 用户注册
```http
POST /signup
Content-Type: application/json

{
  "username": "string",
  "email": "string",
  "password": "string"
}
```

#### 用户登录
```http
POST /login
Content-Type: application/json

{
  "email": "string",
  "password": "string"
}
```

### 认证端点

#### 用户登出
```http
POST /logout
Authorization: Bearer <token>
```

#### 获取用户信息
```http
GET /api/profile
Authorization: Bearer <token>
```

## 开发指南

### 代码规范
- 严格的安全编码策略（禁用 unwrap、expect、panic 等）
- 使用 `anyhow` 进行统一错误处理
- 异步函数优先，提高并发性能
- 明确的类型注解，增强代码可读性

### 测试
```bash
# 运行所有测试
cargo test

# 运行特定测试模块
cargo test user_handler

# 显示测试输出
cargo test -- --nocapture
```

### 代码检查
```bash
# 格式化代码
cargo fmt

# 运行 Clippy 检查
cargo clippy

# 检查代码（不构建）
cargo check
```

## 部署

### 生产环境构建
```bash
cargo build --release
```

### Docker 部署（实验性）

⚠️ **注意**：Docker 部署目前处于实验阶段，存在构建时间长,兼容性,数据库事务问题。**推荐开发者使用 `cargo run` 进行本地开发**。

Orpheus 支持 Docker 容器化部署，提供了完整的 Docker Compose 配置，包含应用、PostgreSQL 和 Redis 服务。

#### 快速启动

1. **使用 Docker Compose 启动服务**
   ```bash
   docker compose up --build
   ```

2. **后台运行**
   ```bash
   docker compose up --build -d
   ```

3. **停止服务**
   ```bash
   docker compose down
   ```

#### 配置说明

**docker-compose.yml** 配置了以下服务：
- **app**: Orpheus 应用服务（端口 8080）
- **postgres**: PostgreSQL 数据库（端口 5432）
- **redis**: Redis 缓存服务（端口 6379）

**环境变量配置**：
应用会自动使用 `docker.env` 文件中的配置，包括：
- 数据库连接信息
- Redis 连接信息
- 应用端口设置

#### 架构兼容性

为了确保在不同平台上的兼容性，Docker 配置使用了 `platform: linux/amd64` 设置。这确保了在 Apple Silicon (M1/M2) 和其他架构上都能正常运行。

#### SQLx 查询缓存

项目使用 SQLx 的编译时查询检查功能。为了在 Docker 构建时不需要数据库连接，我们预先生成了查询缓存文件：

1. **在本地生成查询缓存**：
   ```bash
   export DATABASE_URL="postgres://orpheus_user:secret@localhost:5432/orpheus_db"
   cargo sqlx prepare
   ```

2. **查询缓存文件**：
   - `.sqlx/` 目录包含了所有 SQL 查询的缓存
   - 这些文件会被自动拷贝到 Docker 镜像中
   - 确保了离线构建的能力

#### 数据库初始化

首次启动时，PostgreSQL 容器会自动：
- 创建数据库用户 `orpheus_user`
- 创建数据库 `orpheus_db`
- 启用 UUID 扩展
- 创建 `users` 表（使用 UUID 主键）

#### 故障排除

**GLIBC 版本不兼容**：
如果遇到 `GLIBC_2.33` not found 错误，通常是因为架构不匹配。解决方案：
- 使用 `platform: linux/amd64` 配置（已设置）
- 或者使用更新的基础镜像

**构建缓慢**：
首次构建可能需要较长时间，因为需要：
- 下载 Rust 依赖
- 编译所有代码
- 后续构建会利用 Docker 缓存加速

**端口冲突**：
如果端口被占用，可以修改 `docker-compose.yml` 中的端口映射。

## 贡献指南

1. Fork 项目
2. 创建功能分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 创建 Pull Request

## 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情

## 联系方式

- 项目链接：[https://github.com/KayanoLiam/orpheus](https://github.com/yourusername/orpheus)
- 问题反馈：[Issues](https://github.com/yourusername/orpheus/issues)

---

**注意**：这是一个学习项目，旨在展示 Rust 在后端开发中的应用。请勿在生产环境中使用未经充分测试的代码。

## 致谢

感谢在本项目开发过程中给予支持的朋友们。你们的帮助（无论是物质支持还是精神鼓励）使我能够持续投入时间与精力推进项目。在此列出部分贡献者，以致以诚挚谢意：

| 名称/昵称 | 支持内容 | 备注 |
|-------|----------|------|
| 冉甜甜   | 资金支持 3 USD | 开发初期提供实际支持 |


> 支持类型不限，包括但不限于：资金、硬件、测试协助、推广、建议、耐心倾听、持续鼓励。
