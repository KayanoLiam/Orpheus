# Orpheus 前端应用

[English Version](README.md) | 中文版本

## 概述

这是 Orpheus 项目的前端应用，一个使用 Next.js 16、React 19 和 TypeScript 构建的现代化全栈应用。前端提供了简洁、响应式的用户界面，与 Orpheus 后端 API 进行交互，实现身份认证、用户管理和数据可视化功能。

## 技术栈

| 技术 | 版本 | 描述 |
|------|------|------|
| Next.js | 16.0.3 | React 全栈框架 |
| React | 19.2.0 | UI 库 |
| TypeScript | 5 | 类型安全的 JavaScript |
| Tailwind CSS | 4 | 实用优先的 CSS 框架 |
| Radix UI | - | 无样式 UI 组件库 |
| Lucide React | 0.553.0 | 现代化图标库 |
| Axios | 1.13.2 | HTTP 请求客户端 |
| pnpm | - | 包管理器 |

## 功能特性

- **现代化 UI/UX**：使用 Tailwind CSS 打造简洁、响应式的设计
- **组件化架构**：模块化、可复用的 React 组件
- **类型安全**：完整的 TypeScript 实现
- **API 集成**：与 Orpheus 后端的无缝通信
- **GitHub 集成**：实时显示仓库星标数量
- **导航系统**：直观的导航和响应式菜单
- **身份认证**：用户认证和会话管理

## 项目结构

```
frontend/
├── app/                    # Next.js App Router 目录
│   ├── layout.tsx         # 根布局组件
│   ├── page.tsx           # 首页组件
│   └── globals.css        # 全局样式
├── components/            # React 组件
│   └── ui/                # UI 组件库
│       ├── Navbar.tsx     # 导航栏组件
│       ├── main.tsx       # 主内容组件
│       ├── card.tsx       # 卡片组件
│       ├── button.tsx     # 按钮组件
│       └── alert-dialog.tsx # 警告对话框组件
├── lib/                   # 工具库
│   └── utils.ts           # 工具函数
├── public/                # 静态资源
│   ├── file.svg           # SVG 图标
│   ├── globe.svg          # SVG 图标
│   ├── next.svg           # Next.js 图标
│   ├── vercel.svg         # Vercel 图标
│   └── window.svg         # SVG 图标
├── .gitignore             # Git 忽略文件
├── components.json        # Shadcn/ui 配置
├── eslint.config.mjs      # ESLint 配置
├── next.config.ts         # Next.js 配置
├── package.json           # 依赖和脚本
├── pnpm-lock.yaml         # pnpm 锁文件
├── postcss.config.mjs     # PostCSS 配置
└── tsconfig.json          # TypeScript 配置
```

## 快速开始

### 环境要求

- Node.js 20+
- pnpm 包管理器

### 安装步骤

1. **安装依赖**
   ```bash
   pnpm install
   ```

2. **启动开发服务器**
   ```bash
   pnpm dev
   ```

3. **打开浏览器**
   访问 [http://localhost:3000](http://localhost:3000) 查看应用。

### 可用脚本

- `pnpm dev` - 启动开发服务器
- `pnpm build` - 构建生产版本
- `pnpm start` - 启动生产服务器
- `pnpm lint` - 运行 ESLint

## 开发指南

### 组件开发

组件组织在 `components/ui/` 目录中，遵循以下约定：

- **TypeScript**：所有组件都使用 TypeScript 编写
- **Props 接口**：使用 TypeScript 接口定义清晰的属性
- **文档**：复杂组件包含 JSDoc 注释
- **样式**：使用 Tailwind CSS 进行样式设计
- **响应式**：移动端优先的响应式设计

### API 集成

前端使用 Axios 与 Orpheus 后端 API 进行 HTTP 请求：

```typescript
import axios from 'axios';

// API 调用示例
const response = await axios.get('http://127.0.0.1:8080/api/endpoint');
```

### 样式设计

- **Tailwind CSS**：实用优先的 CSS 框架
- **响应式设计**：移动端优先的方法
- **组件变体**：使用 class-variance-authority 处理组件变体
- **暗色模式**：已配置支持未来的暗色模式

## 核心组件

### 导航栏组件

主导航组件，包含以下功能：
- 响应式设计和移动端菜单
- GitHub 仓库星标数量集成
- 用户认证链接
- 现代化样式和悬停效果

### 主内容组件

主要内容容器组件，提供：
- 一致的布局结构
- 响应式内容区域
- 与其他 UI 组件的集成

### UI 组件

可复用的 UI 组件集合：
- **Button**：可定制的按钮组件
- **Card**：灵活的卡片组件
- **AlertDialog**：模态对话框组件

## 配置

### 环境变量

在根目录创建 `.env.local` 文件：

```env
NEXT_PUBLIC_API_URL=http://127.0.0.1:8080
```

### TypeScript 配置

项目使用严格的 TypeScript 配置：
- 严格类型检查
- 路径映射实现清晰的导入
- Next.js 优化

### ESLint 配置

ESLint 配置包含：
- Next.js 推荐规则
- TypeScript 支持
- 一致的代码格式化

## 部署

### 构建生产版本

```bash
pnpm build
```

### 启动生产服务器

```bash
pnpm start
```

### 环境特定构建

应用支持不同环境：
- 开发环境：`pnpm dev`
- 生产环境：`pnpm build && pnpm start`

## 贡献指南

1. **组件创建**：遵循现有组件模式
2. **类型安全**：确保所有代码都是类型安全的
3. **文档**：为复杂逻辑添加注释
4. **测试**：在不同屏幕尺寸下测试组件
5. **代码风格**：遵循现有代码风格和约定

## 最佳实践

- **组件组合**：从简单组件构建复杂 UI
- **Props 接口**：始终定义清晰的属性接口
- **错误处理**：为 API 调用实现适当的错误处理
- **性能**：使用 React 优化技术（memo、callback 等）
- **可访问性**：确保组件可访问（ARIA 标签、键盘导航）

## 故障排除

### 常见问题

1. **端口已被占用**
   ```bash
   # 终止端口 3000 上的进程
   lsof -ti:3000 | xargs kill -9
   ```

2. **依赖问题**
   ```bash
   # 清除缓存并重新安装
   rm -rf node_modules pnpm-lock.yaml
   pnpm install
   ```

3. **TypeScript 错误**
   ```bash
   # 检查 TypeScript 配置
   pnpm run type-check
   ```

## 支持

对于相关问题：
- **前端**：在仓库中创建 issue
- **后端**：参考后端文档
- **一般问题**：查看主项目 README

---

**注意**：此前端应用是 Orpheus 全栈应用的一部分。请确保后端服务器正在运行以获得完整功能。