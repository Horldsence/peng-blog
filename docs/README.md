# Peng Blog

一个现代化的、基于 Rust 构建的博客系统，采用分层架构设计，注重代码质量和可维护性。

## 目录

- [项目概述](#项目概述)
- [功能特性](#功能特性)
- [架构设计](#架构设计)
- [技术栈](#技术栈)
- [快速开始](#快速开始)
- [配置](#配置)
- [API 文档](#api-文档)
- [开发指南](#开发指南)
- [测试](#测试)
- [部署](#部署)
- [贡献指南](#贡献指南)
- [许可证](#许可证)

## 项目概述

Peng Blog 是一个功能完整的博客系统，采用 Rust 编程语言构建，使用了现代化的 Web 开发技术栈。项目遵循清晰架构原则，将应用程序分为多个层次，每个层次都有明确的职责。

**设计原则：**
- **分层架构**：领域层、业务逻辑层、基础设施层、API 层分离
- **依赖倒置**：高层模块不依赖低层模块，都依赖于抽象
- **单一职责**：每个模块和函数都有明确的单一职责
- **测试优先**：完善的单元测试覆盖业务逻辑
- **类型安全**：充分利用 Rust 的类型系统确保代码安全

## 功能特性

### 用户管理
- 用户注册和登录
- JWT 令牌认证
- 基于角色的权限管理（RBAC）
- 用户资料管理
- 密码安全（Argon2 哈希）

### 文章管理
- 创建、编辑、删除文章
- 文章发布和草稿功能
- 文章列表和详情查询
- 按用户查询文章
- 权限控制和所有权验证

### 评论系统
- 文章评论功能
- 支持注册用户评论
- GitHub OAuth 登录评论
- 评论 CRUD 操作
- 用户和评论关联

### 文件管理
- 文件上传功能
- 文件存储和访问
- 文件列表和删除
- 支持多种文件类型

### 会话管理
- 基于 Cookie 的会话
- 会话创建、验证、销毁
- "记住我"功能
- 自动过期清理

### 统计功能
- 网站访问统计
- 文章阅读量统计
- 每日访问记录
- 统计数据查询

## 架构设计

```
peng-blog/
├── crates/
│   ├── domain/          # 领域层 - 核心业务实体和规则
│   ├── service/         # 业务逻辑层 - 仓库接口和业务服务
│   ├── infrastructure/  # 基础设施层 - 数据库实现
│   ├── api/            # API 层 - HTTP 处理和路由
│   └── app/            # 应用层 - 应用启动和配置
├── docs/               # 文档
├── static/             # 静态文件
└── uploads/            # 上传文件目录
```

### 分层说明

#### 1. Domain（领域层）
**职责：** 定义核心业务实体、错误类型和常量

**特点：**
- 不依赖任何其他层
- 包含纯数据结构和业务规则
- 定义权限系统（位标志实现）
- 提供统一的错误处理

**主要模块：**
- `post`: 文章实体
- `user`: 用户实体
- `session`: 会话实体
- `file`: 文件实体
- `comment`: 评论实体
- `stats`: 统计实体

#### 2. Service（业务逻辑层）
**职责：** 定义仓库接口和实现业务规则

**特点：**
- 定义仓库 Trait（接口）
- 实现具体的业务服务
- 编排多个仓库操作
- 验证业务规则
- 完全可测试（无 I/O）

**主要服务：**
- `PostService`: 文章业务逻辑
- `UserService`: 用户业务逻辑
- `SessionService`: 会话业务逻辑
- `FileService`: 文件业务逻辑
- `CommentService`: 评论业务逻辑
- `StatsService`: 统计业务逻辑

#### 3. Infrastructure（基础设施层）
**职责：** 实现数据访问和外部服务集成

**特点：**
- 实现 Service 层定义的仓库接口
- 使用 SeaORM 进行数据库操作
- 处理数据库迁移
- 管理数据库连接

**主要组件：**
- 数据库实体定义
- 仓库实现
- 数据库迁移

#### 4. API（API 层）
**职责：** 处理 HTTP 请求和响应

**特点：**
- 使用 Axum 框架
- 处理请求验证
- 格式化响应
- 认证中间件
- 错误处理

**主要路由：**
- `/api/auth`: 认证相关
- `/api/posts`: 文章相关
- `/api/users`: 用户相关
- `/api/sessions`: 会话相关
- `/api/files`: 文件相关
- `/api/comments`: 评论相关
- `/api/stats`: 统计相关

#### 5. App（应用层）
**职责：** 应用程序启动和配置

**特点：**
- 依赖注入
- 配置管理
- 服务组装
- Web 服务器启动

## 技术栈

### 核心框架
- **Rust 2021 Edition**: 现代 Rust 特性
- **Tokio**: 异步运行时
- **Axum**: Web 框架
- **Tower**: 服务抽象和中间件

### 数据库
- **SeaORM**: ORM 框架
- **SQLite**: 数据库（可配置其他数据库）
- **SeaORM Migrations**: 数据库迁移

### 认证和安全
- **JWT**: JSON Web Token 认证
- **Argon2**: 密码哈希
- **UUID**: 唯一标识符

### 序列化
- **Serde**: 序列化/反序列化
- **Serde JSON**: JSON 支持

### 工具库
- **Chrono**: 日期时间处理
- **Reqwest**: HTTP 客户端
- **Thiserror**: 错误处理
- **Anyhow**: 错误传播
- **Tracing**: 日志和追踪
- **Mockall**: 模拟测试

### 开发工具
- **Cargo**: 包管理器
- **Rustfmt**: 代码格式化
- **Clippy**: 代码检查

## 快速开始

### 前置要求

- Rust 1.70 或更高版本
- Cargo
- SQLite 3

### 安装

1. **克隆仓库**
```bash
git clone https://github.com/yourusername/peng-blog.git
cd peng-blog
```

2. **安装依赖**
```bash
cargo build
```

3. **运行数据库迁移**
```bash
cargo run --bin migrator
```

4. **配置环境变量**

创建 `.env` 文件：
```env
DATABASE_URL=sqlite://blog.db
JWT_SECRET=your-secret-key-here
UPLOAD_DIR=./uploads
BASE_URL=http://localhost:3000
GITHUB_CLIENT_ID=your-github-client-id
GITHUB_CLIENT_SECRET=your-github-client-secret
```

5. **启动应用**
```bash
cargo run
```

应用将在 `http://localhost:3000` 启动。

### Docker 部署

```bash
docker build -t peng-blog .
docker run -p 3000:3000 -e DATABASE_URL=sqlite:///data/blog.db peng-blog
```

## 配置

### 环境变量

| 变量名 | 必需 | 默认值 | 描述 |
|--------|------|--------|------|
| `DATABASE_URL` | 是 | 无 | 数据库连接字符串 |
| `JWT_SECRET` | 是 | 无 | JWT 签名密钥 |
| `UPLOAD_DIR` | 否 | `./uploads` | 文件上传目录 |
| `BASE_URL` | 否 | `http://localhost:3000` | 应用基础 URL |
| `GITHUB_CLIENT_ID` | 否 | 空字符串 | GitHub OAuth 客户端 ID |
| `GITHUB_CLIENT_SECRET` | 否 | 空字符串 | GitHub OAuth 客户端密钥 |

### 数据库配置

默认使用 SQLite，可以通过修改 `DATABASE_URL` 切换到其他数据库：

```env
# PostgreSQL
DATABASE_URL=postgresql://user:password@localhost/blog

# MySQL
DATABASE_URL=mysql://user:password@localhost/blog
```

## API 文档

### 认证接口

#### 用户注册
```http
POST /api/auth/register
Content-Type: application/json

{
  "username": "testuser",
  "password": "SecurePassword123!"
}
```

#### 用户登录
```http
POST /api/auth/login
Content-Type: application/json

{
  "username": "testuser",
  "password": "SecurePassword123!"
}
```

响应：
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "user": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "username": "testuser",
    "permissions": 15
  }
}
```

### 文章接口

#### 创建文章
```http
POST /api/posts
Authorization: Bearer <token>
Content-Type: application/json

{
  "title": "我的第一篇文章",
  "content": "这是文章内容..."
}
```

#### 获取文章列表
```http
GET /api/posts?limit=10
```

#### 获取文章详情
```http
GET /api/posts/<post_id>
```

#### 更新文章
```http
PUT /api/posts/<post_id>
Authorization: Bearer <token>
Content-Type: application/json

{
  "title": "更新的标题",
  "content": "更新的内容"
}
```

#### 删除文章
```http
DELETE /api/posts/<post_id>
Authorization: Bearer <token>
```

### 评论接口

#### 创建评论
```http
POST /api/comments
Content-Type: application/json

{
  "post_id": "550e8400-e29b-41d4-a716-446655440000",
  "content": "评论内容",
  "user_token": "eyJhbGci..."  // JWT token 或 GitHub token
}
```

#### 获取文章评论
```http
GET /api/posts/<post_id>/comments
```

### 文件接口

#### 上传文件
```http
POST /api/files
Authorization: Bearer <token>
Content-Type: multipart/form-data

file: <binary data>
```

#### 获取文件列表
```http
GET /api/files?user_id=<user_id>
```

### 统计接口

#### 记录访问
```http
POST /api/stats/visit
Content-Type: application/json

{
  "post_id": "550e8400-e29b-41d4-a716-446655440000"  // 可选
}
```

#### 获取统计信息
```http
GET /api/stats
```

## 开发指南

### 项目结构理解

建议的开发流程：
1. 在 `domain` 层定义新的实体类型
2. 在 `service` 层定义仓库 Trait 和业务逻辑
3. 在 `infrastructure` 层实现仓库
4. 在 `api` 层添加路由和处理器
5. 在 `app` 层配置依赖注入

### 添加新功能

假设我们要添加"标签"功能：

1. **Domain 层**
```rust
// crates/domain/src/tag.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub id: uuid::Uuid,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTag {
    pub name: String,
}
```

2. **Service 层**
```rust
// crates/service/src/repository.rs
#[async_trait::async_trait]
pub trait TagRepository: Send + Sync {
    async fn create_tag(&self, name: String) -> Result<Tag>;
    async fn get_tag(&self, id: uuid::Uuid) -> Result<Option<Tag>>;
    async fn list_tags(&self) -> Result<Vec<Tag>>;
}

// crates/service/src/tag.rs
pub struct TagService<TR: TagRepository> {
    tag_repo: Arc<TR>,
}

impl<TR: TagRepository> TagService<TR> {
    pub fn new(tag_repo: Arc<TR>) -> Self {
        Self { tag_repo }
    }
    
    pub async fn create_tag(&self, request: CreateTag) -> Result<Tag> {
        // 验证逻辑
        if request.name.trim().is_empty() {
            return Err(Error::Validation("Tag name cannot be empty".into()));
        }
        self.tag_repo.create_tag(request.name).await
    }
}
```

3. **Infrastructure 层**
```rust
// crates/infrastructure/src/tag.rs
use sea_orm::*;
use domain::Tag;

pub struct TagRepositoryImpl {
    db: Arc<DatabaseConnection>,
}

impl TagRepositoryImpl {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl service::TagRepository for TagRepositoryImpl {
    async fn create_tag(&self, name: String) -> Result<Tag> {
        let tag = entity::tag::ActiveModel {
            id: Set(Uuid::new_v4()),
            name: Set(name),
            created_at: Set(chrono::Utc::now().naive_utc()),
        };
        let result = tag.insert(&*self.db).await?;
        Ok(Tag::from(result))
    }
    
    // 其他方法实现...
}
```

4. **API 层**
```rust
// crates/api/src/tag.rs
use axum::{
    Json, extract::State,
    response::IntoResponse,
};

pub async fn create_tag<PR, UR, SR, FR, CR, STR>(
    State(state): State<AppState<PR, UR, SR, FR, CR, STR>>,
    Json(request): Json<CreateTag>,
) -> ApiResult<Json<Tag>> {
    let tag = state.tag_service.create_tag(request).await?;
    Ok(Json(tag))
}

pub fn routes<PR, UR, SR, FR, CR, STR>() -> axum::Router<AppState<PR, UR, SR, FR, CR, STR>> {
    axum::Router::new()
        .route("/tags", post(create_tag))
        .route("/tags", get(list_tags))
}
```

5. **App 层**
```rust
// crates/app/src/main.rs
let tag_repo = Arc::new(TagRepositoryImpl::new(Arc::clone(&db)));
let tag_service = TagService::new(tag_repo);

let state = AppState::new(
    // 其他服务...
    tag_service,
    // ...
);
```

### 代码规范

- 使用 `cargo fmt` 格式化代码
- 使用 `cargo clippy` 检查代码
- 为公共 API 编写文档注释
- 为业务逻辑编写单元测试
- 遵循 Rust 命名约定

### 错误处理

项目使用统一的错误处理：

```rust
use domain::{Error, Result};

pub async fn some_operation(&self) -> Result<Type> {
    // 验证错误
    if condition {
        return Err(Error::Validation("Invalid input".into()));
    }
    
    // 数据库错误（会自动转换）
    let result = self.repo.do_something().await?;
    
    Ok(result)
}
```

## 测试

### 运行所有测试
```bash
cargo test --workspace
```

### 运行特定测试
```bash
cargo test -p service post::tests::test_create_post
```

### 查看测试覆盖率
```bash
cargo tarpaulin --workspace --out Html
```

### 测试策略

1. **单元测试**：在 `service` 层测试业务逻辑
2. **集成测试**：测试 API 端点
3. **Mock 测试**：使用 `mockall` 模拟依赖

**示例测试：**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    
    #[tokio::test]
    async fn test_create_post_validates_title() {
        let mut mock_repo = MockPostRepository::new();
        mock_repo.expect_create_post()
            .with(always())
            .returning(|_| Ok(Post::new(...)));
        
        let service = PostService::new(Arc::new(mock_repo));
        let result = service.create(user_id, "".to_string(), "content".to_string()).await;
        
        assert!(result.is_err());
        match result {
            Err(Error::Validation(msg)) => assert!(msg.contains("empty")),
            _ => panic!("Expected validation error"),
        }
    }
}
```

## 部署

### 生产环境配置

1. **使用强密钥**
```env
JWT_SECRET=$(openssl rand -base64 32)
```

2. **配置数据库**
```env
DATABASE_URL=postgresql://user:$(openssl rand -base64 16)@db-host:5432/blog
```

3. **启用日志**
```env
RUST_LOG=info,peng_blog=debug
```

### Docker Compose

```yaml
version: '3.8'
services:
  app:
    build: .
    ports:
      - "3000:3000"
    environment:
      - DATABASE_URL=postgresql://postgres:password@db:5432/blog
      - JWT_SECRET=${JWT_SECRET}
      - BASE_URL=https://yourdomain.com
    depends_on:
      - db
    volumes:
      - ./uploads:/uploads
  
  db:
    image: postgres:15
    environment:
      - POSTGRES_DB=blog
      - POSTGRES_USER=postgres
      - POSTGRES_PASSWORD=password
    volumes:
      - postgres_data:/var/lib/postgresql/data
      - ./migrations:/docker-entrypoint-initdb.d

volumes:
  postgres_data:
```

### 性能优化

- 启用 Rust 编译优化
```bash
cargo build --release
```

- 使用连接池（SeaORM 默认支持）
- 启用 HTTP 缓存
- 使用 CDN 提供静态文件
- 配置适当的数据库索引

## 贡献指南

我们欢迎所有形式的贡献！

### 如何贡献

1. Fork 本仓库
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

### 代码审查标准

- 代码必须通过所有测试
- 代码必须通过 `cargo clippy` 检查
- 新功能必须有测试覆盖
- 公共 API 必须有文档
- 遵循项目代码风格

### 报告问题

如果你发现 bug 或有功能建议，请在 GitHub Issues 中报告。

## 许可证

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件。

## 致谢

- Rust 团队和社区
- Axum 框架的贡献者
- SeaORM 团队
- 所有为本项目做出贡献的开发者

## 联系方式

- 作者: Linus Torvalds
- 邮箱: torvalds@linux-foundation.org
- 项目主页: https://github.com/yourusername/peng-blog

---

**注意**: 这是一个示例项目，用于展示 Rust Web 开发的最佳实践。