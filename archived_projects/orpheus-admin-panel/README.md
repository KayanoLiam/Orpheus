# Orpheus Admin Panel (Archived)

这是 Orpheus 的前端管理面板，已归档为独立项目。

## 状态：已归档 🗄️

此项目原本是 Orpheus BaaS 平台的前端 UI，但已被分离出来作为独立项目维护。

## 为什么归档？

Orpheus 的核心定位是 **Backend-as-a-Service (BaaS) 平台**，类似 Supabase：
- 自动 REST API 生成
- 实时数据订阅
- 对象存储服务
- 数据库管理 API

前端管理面板虽然很有用，但不是 BaaS 平台的核心功能。将其分离可以：
1. 保持 Orpheus 核心专注于后端服务
2. 允许任何前端框架连接 Orpheus
3. 简化核心开发流程

## 技术栈

- **框架**: Next.js 16.0.3
- **UI 库**: React 19.2.0
- **语言**: TypeScript 5
- **样式**: Tailwind CSS 4
- **3D 渲染**: Three.js + React Three Fiber
- **动画**: Framer Motion
- **组件**: Radix UI
- **HTTP**: Axios

## 功能

### 已实现
- ✅ 登录页面（含 3D 背景动画）
- ✅ 注册页面
- ✅ 用户仪表板
- ✅ 导航栏组件
- ✅ UI 组件库（Card、Button、Input、Label、AlertDialog）
- ✅ 利用规约和隐私政策页面
- ✅ Awwwards 级别的视觉设计

### 未完成
- ❌ 实际的 API 集成（仅 UI 展示）
- ❌ 数据库管理界面
- ❌ 实时数据订阅 UI
- ❌ 文件存储管理

## 如何使用

### 方案 1：作为独立项目开发

```bash
cd archived_projects/orpheus-admin-panel/frontend

# 安装依赖
pnpm install

# 开发模式
pnpm dev

# 构建生产版本
pnpm build
```

### 方案 2：连接到 Orpheus API

1. 确保 Orpheus 后端运行在 `http://127.0.0.1:8080`
2. 修改 API 端点配置
3. 实现实际的数据操作功能

### 方案 3：构建新的管理面板

如果你想构建一个完整的 Orpheus 管理面板：

**推荐功能**:
1. **数据库管理**
   - 表的 CRUD 操作
   - 列的添加/修改/删除
   - 索引和约束管理
   - SQL 查询编辑器

2. **API 浏览器**
   - 自动生成的 API 文档
   - API 测试工具
   - 请求/响应查看器

3. **实时监控**
   - 实时连接数
   - API 请求统计
   - 性能指标仪表板

4. **存储管理**
   - 文件浏览器
   - 上传/下载
   - Bucket 管理

5. **用户和权限**
   - RLS 策略编辑器
   - 角色管理
   - 权限可视化

## 设计理念

此前端采用 **Awwwards 级别**的设计：

### 视觉特色
- 🌑 **深色主题** - 专业且现代
- ✨ **3D 元素** - Three.js 粒子背景
- 🎬 **流畅动画** - Framer Motion 微交互
- 💎 **Glassmorphism** - 毛玻璃效果
- 🌈 **霓虹色调** - 高对比度配色

### 技术亮点
- 完全响应式设计
- 60fps 流畅动画
- 模块化组件架构
- TypeScript 类型安全

## 相关资源

### Supabase Dashboard (参考)
- [Supabase Studio](https://github.com/supabase/supabase/tree/master/studio)
- 功能完整的管理面板
- 可作为设计参考

### 其他 BaaS 管理面板
- Firebase Console
- AWS Amplify Console
- PlanetScale Dashboard

## 迁移到新前端框架

如果你想用其他框架重建：

**React + Vite**:
```bash
npm create vite@latest orpheus-admin -- --template react-ts
```

**Vue 3**:
```bash
npm create vue@latest orpheus-admin
```

**Svelte**:
```bash
npm create vite@latest orpheus-admin -- --template svelte-ts
```

**Angular**:
```bash
ng new orpheus-admin
```

## 贡献

如果你想继续开发此管理面板，请：
1. Fork 这个项目
2. 创建功能分支
3. 提交 Pull Request

或者将其作为独立项目维护。

## 许可证

MIT License

---

**注意**: 这个前端项目与 Orpheus 核心是分离的。Orpheus 专注于提供 API 和后端服务，前端只是一个可选的管理界面。
