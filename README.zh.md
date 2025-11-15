# Orpheus

[English Version](README.md) | 中文版本

## 项目概述

Orpheus 是一个基于 Rust 生态构建的类 Supabase 的全栈应用系统，旨在为 Web 应用提供开箱即用的身份认证、会话管理与数据访问能力。项目采用 Actix-Web 作为核心 Web 框架，PostgreSQL 作为主数据存储，Redis 用于会话存储与缓存加速。前端采用 Next.js 16 + React 19 + TypeScript 构建，提供现代化的用户界面。

本项目的目标是构建一个可扩展、可自托管、具备良好安全性和性能的全栈应用基础设施。

## 核心功能

### 用户系统
- 用户注册、登录、注销
- 密码重置功能
- 用户删除功能
- 支持密码哈希与安全验证（基于 Argon2）
- Bearer Token 认证机制

### 会话管理
- Redis 存储会话，提高性能与可恢复性
- 会话自动过期、刷新机制
- 中间件统一鉴权处理

### 数据库层
- PostgreSQL 作为主数据库
- 统一的连接池与错误处理抽象
- 类型安全的数据库操作（SQLx）

### API 设计
- RESTful 风格接口
- 结构化 JSON 响应格式
- 清晰的错误与状态码规范
- GitHub API 集成示例

### 前端界面
- 现代化的 React 19 + Next.js 16 应用
- 响应式设计，支持移动端
- TypeScript 类型安全
- Tailwind CSS 现代化样式
- 组件化架构

### 代码质量保证
- 严格的类型与错误处理策略
- 完整测试套件（单元测试 + 集成测试）
- 模块化架构，易于开发与维护
- 结构化日志记录

## 技术栈

### 后端
| 组件 | 版本 | 描述 |
|------|------|------|
| Rust | 2021 Edition | 主语言，强调安全与性能 |
| Actix-Web | 4.x | 高性能 Web 框架 |
| PostgreSQL | - | 主数据存储（关系型数据库） |
| Redis | 0.23.1 | 会话 / 缓存存储 |
| Argon2 | 0.5 | 密码哈希算法 |
| SQLx | 0.8.6 | 类型安全的数据库操作 |
| Tokio | 1.x | 异步运行时 |
| Tracing | 0.1 | 结构化日志 |

### 前端
| 组件 | 版本 | 描述 |
|------|------|------|
| Next.js | 16.0.3 | React 全栈框架 |
| React | 19.2.0 | UI 库 |
| TypeScript | 5 | 类型安全的 JavaScript |
| Tailwind CSS | 4 | 现代化 CSS 框架 |
| Radix UI | - | 无样式 UI 组件库 |
| Lucide React | 0.553.0 | 现代化图标库 |
| Axios | 1.13.2 | HTTP 客户端 |

## 项目结构

```
orpheus/
├── src/                    # 后端源代码
│   ├── main.rs             # 应用程序入口点
│   ├── config.rs           # 配置常量
│   ├── auth/               # 认证模块
│   │   └── session_store.rs # Redis 会话管理
│   ├── handlers/           # HTTP 请求处理器
│   │   ├── user_handler.rs # 用户相关接口
│   │   ├── session_handler.rs # 会话相关接口
│   │   └── github_handler.rs # GitHub API 集成
│   ├── middlewares/        # 中间件
│   │   └── session.rs      # 会话验证中间件
│   └── models/             # 数据模型
│       ├── user.rs         # 用户模型
│       ├── session.rs      # 会话模型
│       └── response.rs     # API 响应模型
├── frontend/               # 前端源代码
│   ├── app/                # Next.js 应用目录
│   │   ├── layout.tsx      # 应用布局
│   │   ├── page.tsx        # 主页面
│   │   └── globals.css     # 全局样式
│   ├── components/         # React 组件
│   │   └── ui/             # UI 组件库
│   ├── lib/                # 工具库
│   └── public/             # 静态资源
├── tests/                  # 测试套件
│   ├── user_handler_tests.rs
│   ├── session_handler_tests.rs
│   ├── session_store_tests.rs
│   └── session_middleware_tests.rs
├── docker-compose.yml      # Docker 容器编排
├── Dockerfile              # 后端 Docker 镜像构建
├── .env                    # 环境变量配置
├── Cargo.toml              # Rust 项目依赖配置
└── README.md               # 项目说明文档
```

## 快速开始

### 环境要求
- Rust 1.70+
- Node.js 20+
- PostgreSQL 12+
- Redis 6+
- Docker & Docker Compose (可选)

### 安装步骤

1. **克隆项目**
   ```bash
   git clone https://github.com/KayanoLiam/Orpheus.git
   cd Orpheus
   ```

2. **配置环境变量**
   创建 `.env` 文件并配置以下变量：
   ```bash
   DATABASE_URL=postgres://用户名:密码@localhost:5432/数据库名
   REDIS_URL=redis://localhost:6379
   ```

3. **后端设置**
   ```bash
   # 构建项目
   cargo build --release

   # 运行后端服务
   cargo run
   ```

4. **前端设置**
   ```bash
   # 进入前端目录
   cd frontend

   # 安装依赖
   pnpm install

   # 运行开发服务器
   pnpm dev
   ```

   服务将在以下地址启动：
   - 后端 API: `http://127.0.0.1:8080`
   - 前端应用: `http://localhost:3000`

### Docker 部署

```bash
# 使用 Docker Compose 启动所有服务
docker-compose up -d --build

# 查看服务状态
docker-compose ps

# 停止服务
docker-compose down
```

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

#### 密码重置
```http
POST /reset-password
Content-Type: application/json

{
  "email": "string",
  "new_password": "string"
}
```

#### 删除用户
```http
DELETE /delete-user
Content-Type: application/json

{
  "email": "string",
  "password": "string"
}
```

#### 获取 GitHub 仓库星标
```http
GET /github/stars/{owner}/{repo}
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
- 使用 `tracing` 进行结构化日志记录

### 后端开发
```bash
# 运行所有测试
cargo test

# 运行特定测试模块
cargo test user_handler

# 显示测试输出
cargo test -- --nocapture

# 格式化代码
cargo fmt

# 运行 Clippy 检查
cargo clippy

# 检查代码（不构建）
cargo check
```

### 前端开发
```bash
cd frontend

# 安装依赖
pnpm install

# 运行开发服务器
pnpm dev

# 构建生产版本
pnpm build

# 代码检查
pnpm lint

# 类型检查
pnpm type-check
```

## 部署

### 生产环境构建

#### 后端
```bash
cargo build --release
```

#### 前端
```bash
cd frontend
pnpm build
pnpm start
```

### Docker 部署

项目支持完整的容器化部署，包含应用、PostgreSQL 和 Redis 服务。

#### 配置说明

**docker-compose.yml** 配置了以下服务：
- **app**: Orpheus 应用服务（端口 8080）
- **postgres**: PostgreSQL 数据库（端口 5432）
- **redis**: Redis 缓存服务（端口 6379）

#### 架构兼容性

为了确保在不同平台上的兼容性，Docker 配置使用了 `platform: linux/amd64` 设置，确保在 Apple Silicon (M1/M2) 和其他架构上都能正常运行。

#### 故障排除

**构建缓慢**：首次构建可能需要较长时间，后续构建会利用 Docker 缓存加速。

**端口冲突**：如果端口被占用，可以修改 `docker-compose.yml` 中的端口映射。

## 贡献指南

1. Fork 项目
2. 创建功能分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 创建 Pull Request

## 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情

## 联系方式

- 项目链接：[https://github.com/KayanoLiam/Orpheus](https://github.com/KayanoLiam/Orpheus)
- 问题反馈：[Issues](https://github.com/KayanoLiam/Orpheus/issues)

## 致谢

感谢在本项目开发过程中给予支持的朋友们。你们的帮助（无论是物质支持还是精神鼓励）使我能够持续投入时间与精力推进项目。在此列出部分贡献者，以致以诚挚谢意：

| 名称/昵称 | 支持内容 | 备注 |
|-------|----------|------|
| 冉甜甜   | 资金支持 3 USD | 开发初期提供实际支持 |

> 支持类型不限，包括但不限于：资金、硬件、测试协助、推广、建议、耐心倾听、持续鼓励。

---

**注意**：这是一个学习项目，旨在展示 Rust 在后端开发中的应用以及现代前端技术的实践。请勿在生产环境中使用未经充分测试的代码。