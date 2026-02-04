# Peng Blog

一个使用 Rust 构建的现代化博客系统，采用分层架构设计，提供完整的博客功能和优秀的开发体验。

## 📋 功能特性

### 核心功能
- **用户认证系统**：基于 JWT 的安全认证，支持注册、登录和会话管理
- **文章管理**：创建、编辑、删除和发布文章，支持 Markdown 格式
- **评论系统**：支持匿名评论和 GitHub OAuth 登录评论
- **文件上传**：支持图片和文件上传管理
- **访问统计**：记录全局访问量和文章阅读量统计
- **权限管理**：基于位标志的细粒度权限控制系统

### 技术特性
- **异步高性能**：基于 Tokio 异步运行时，提供出色的并发性能
- **类型安全**：充分利用 Rust 类型系统，编译时捕获错误
- **RESTful API**：标准化的 REST API 设计
- **数据库迁移**：内置数据库迁移工具
- **结构化日志**：使用 tracing 进行完善的日志记录
- **CORS 支持**：跨域资源共享配置

## 🏗️ 技术栈

### 后端框架
- **Tokio** - 异步运行时
- **Axum** - 现代化 Web 框架
- **Tower** - 服务抽象和中间件

### 数据库
- **SeaORM** - 异步 ORM
- **PostgreSQL** - 关系型数据库
- **SeaORM Migration** - 数据库迁移工具

### 安全与认证
- **JWT** - JSON Web Token 认证
- **Argon2** - 密码哈希算法

### 开发工具
- **Tracing** - 结构化日志
- **Serde** - 序列化/反序列化
- **Anyhow/Thiserror** - 错误处理

## 📁 项目结构

```
peng-blog/
├── crates/
│   ├── app/              # 应用入口层
│   │   └── src/
│   │       └── main.rs   # 主程序入口
│   ├── api/              # API 路由和处理器
│   │   └── src/
│   │       ├── auth.rs   # 认证路由
│   │       ├── post.rs   # 文章路由
│   │       ├── user.rs   # 用户路由
│   │       ├── comment.rs # 评论路由
│   │       ├── file.rs   # 文件路由
│   │       ├── stats.rs  # 统计路由
│   │       ├── state.rs  # 应用状态管理
│   │       └── error.rs  # API 错误处理
│   ├── domain/           # 领域层（类型定义）
│   │   └── src/
│   │       ├── post.rs   # 文章类型
│   │       ├── user.rs   # 用户类型
│   │       ├── comment.rs # 评论类型
│   │       ├── session.rs # 会话类型
│   │       ├── file.rs   # 文件类型
│   │       ├── stats.rs  # 统计类型
│   │       └── error.rs  # 领域错误
│   ├── service/          # 业务逻辑层
│   │   └── src/
│   │       ├── post.rs   # 文章服务
│   │       ├── user.rs   # 用户服务
│   │       ├── comment.rs # 评论服务
│   │       ├── file.rs   # 文件服务
│   │       ├── stats.rs  # 统计服务
│   │       └── repository.rs # 仓储接口
│   └── infrastructure/   # 基础设施层
│       └── src/
│           ├── entity/   # 数据库实体
│           ├── post.rs   # 文章仓储实现
│           ├── user.rs   # 用户仓储实现
│           ├── comment.rs # 评论仓储实现
│           ├── file.rs   # 文件仓储实现
│           └── stats.rs  # 统计仓储实现
├── docs/                 # 项目文档
├── Cargo.toml           # Workspace 配置
└── README.md            # 本文件
```

## 🚀 快速开始

### 环境要求

- Rust 1.70 或更高版本
- PostgreSQL 12+ (需要先安装并启动 PostgreSQL 服务)
- Git

### 安装步骤

1. **克隆仓库**
   ```bash
   git clone <repository-url>
   cd peng-blog
   ```

2. **配置环境变量**

   创建 `.env` 文件：
   ```env
   # PostgreSQL 连接字符串格式: postgresql://username:password@hostname:port/database_name
   DATABASE_URL=postgresql://postgres:postgres@localhost:5432/peng_blog

   # 服务器配置
   # HOST: 服务器绑定地址（使用 0.0.0.0 监听所有网络接口）
   # 重要: 不要使用域名，会导致 "Cannot assign requested address" 错误
   # 有效值: 0.0.0.0（所有接口）, 127.0.0.1（仅本地）, 或具体 IP
   HOST=0.0.0.0
   PORT=3000

   # BASE_URL: 公网访问地址（用于 OAuth 回调和链接）
   # 开发环境: http://localhost:3000
   # 生产环境: https://yourdomain.com 或 https://your-subdomain.example.com
   BASE_URL=http://localhost:3000

   # 认证配置
   JWT_SECRET=your-secret-key-here

   # 存储配置
   UPLOAD_DIR=./uploads

   # GitHub OAuth 配置
   # 在 https://github.com/settings/developers 注册应用
   # Authorization callback URL: {BASE_URL}/api/comments/github/callback
   GITHUB_CLIENT_ID=your-github-client-id
   GITHUB_CLIENT_SECRET=your-github-client-secret
   ```

3. **配置 GitHub OAuth 应用**（用于评论功能）

   访问 https://github.com/settings/developers，创建新的 OAuth App：

   - **Application name**: 你的博客名称
   - **Homepage URL**: 你的 `BASE_URL`（如 `https://yourdomain.com`）
   - **Authorization callback URL**: `BASE_URL/api/comments/github/callback`
     - 开发环境: `http://localhost:3000/api/comments/github/callback`
     - 生产环境: `https://yourdomain.com/api/comments/github/callback`
   - **Application description**: （可选）

   创建后，复制 **Client ID** 和生成 **Client Secret**，填入上面的 `.env` 文件。

   **重要说明**：
   - `HOST`: 服务器监听地址，使用 `0.0.0.0` 以允许外网访问
   - `BASE_URL`: 公网访问地址，GitHub OAuth 回调会使用此地址
   - 生产环境部署时，只需修改 `BASE_URL` 为实际域名，`HOST` 保持 `0.0.0.0`
   - GitHub 应用设置中的回调 URL 必须与 `BASE_URL/api/comments/github/callback` 完全匹配

3. **安装依赖**
   ```bash
   cargo build
   ```

4. **运行数据库迁移**
   ```bash
   cargo run
   ```
   迁移会在首次启动时自动执行。

5. **启动服务**
   ```bash
   cargo run
   ```

服务将在 `http://localhost:3000` 启动。

## 🔧 开发指南

### 运行开发服务器

```bash
cargo run
```

### 运行测试

```bash
cargo test
```

### 代码格式化

```bash
cargo fmt
```

### 代码检查

```bash
cargo clippy
```

### CLI 工具

peng-blog 提供了强大的 CLI 工具用于用户和数据库管理：

#### 用户管理

```bash
# 查看所有用户
cargo run --package cli -- user list

# 查看用户详情
cargo run --package cli -- user show <user_id>

# 创建新用户（交互式）
cargo run --package cli -- user create

# 创建管理员用户
cargo run --package cli -- user create --username admin --password "admin123" --admin

# 删除用户
cargo run --package cli -- user delete <user_id>

# 重置用户密码（交互式）
cargo run --package cli -- user reset-password <user_id>

# 提升用户为管理员
cargo run --package cli -- user promote <user_id>

# 降级管理员为普通用户
cargo run --package cli -- user demote <user_id>
```

#### 数据库管理

```bash
# 运行数据库迁移
cargo run --package cli -- db migrate

# 重置数据库（警告：会删除所有数据）
cargo run --package cli -- db reset

# 查看数据库状态
cargo run --package cli -- db status
```

#### 非交互模式

CLI 工具支持非交互模式，适合脚本使用：

```bash
# 创建用户（非交互）
cargo run --package cli -- user create --username testuser --password "test123" --non-interactive

# 重置密码（非交互）
cargo run --package cli -- user reset-password <user_id> --password "newpass123" --non-interactive
```

## 📚 API 文档

### 认证相关
- `POST /api/auth/register` - 用户注册
- `POST /api/auth/login` - 用户登录
- `POST /api/auth/logout` - 用户登出

### 文章管理
- `GET /api/posts` - 获取文章列表
- `GET /api/posts/:id` - 获取单篇文章
- `POST /api/posts` - 创建文章（需认证）
- `PUT /api/posts/:id` - 更新文章（需认证）
- `DELETE /api/posts/:id` - 删除文章（需认证）

### 评论管理
- `GET /api/posts/:id/comments` - 获取文章评论
- `POST /api/posts/:id/comments` - 创建评论
- `POST /api/comments/github/auth` - GitHub 认证

### 文件管理
- `POST /api/files/upload` - 上传文件（需认证）
- `GET /api/files/:id` - 获取文件信息
- `DELETE /api/files/:id` - 删除文件（需认证）

### 统计信息
- `GET /api/stats/visits` - 获取访问统计
- `POST /api/stats/visits` - 记录访问
- `GET /api/stats/posts/:id/views` - 获取文章阅读量
- `POST /api/stats/posts/:id/views` - 记录文章阅读

## 🏛️ 架构设计

### 分层架构

项目采用经典的四层架构：

1. **Domain 层**：定义核心业务实体和规则
2. **Service 层**：实现业务逻辑和仓储接口
3. **Infrastructure 层**：实现数据访问和外部服务集成
4. **API 层**：处理 HTTP 请求和响应

### 依赖方向

```
App → API → Service → Domain
              ↓
        Infrastructure
```

### 权限系统

使用位标志实现高效的权限控制：

- `POST_CREATE` (1<<0) - 创建文章
- `POST_UPDATE` (1<<1) - 更新文章
- `POST_DELETE` (1<<2) - 删除文章
- `POST_PUBLISH` (1<<3) - 发布文章
- `USER_MANAGE` (1<<4) - 管理用户

普通用户默认权限：`POST_CREATE | POST_UPDATE | POST_PUBLISH`
管理员权限：所有权限的组合

## 🤝 贡献指南

欢迎贡献！请遵循以下步骤：

1. Fork 本仓库
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

## 📄 许可证

本项目采用 MIT 许可证。详见 [LICENSE](LICENSE) 文件。

## 👨‍💻 作者

Linus Torvalds

## 🙏 致谢

感谢所有为本项目做出贡献的开发者。

---

**注意**：本项目仍在积极开发中，API 可能会有变动。建议在生产环境使用前进行充分的测试。