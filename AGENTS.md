# AGENTS.md - Peng Blog

> This file contains essential information for AI coding agents working on the Peng Blog project.
> 本文档为 AI 编码代理提供项目关键信息。

## 项目概述 (Project Overview)

Peng Blog 是一个使用 Rust 构建的现代化博客系统，采用分层架构设计，提供完整的博客功能和优秀的开发体验。

**核心功能:**
- 用户认证系统（基于 JWT）
- 文章管理（Markdown 支持）
- 评论系统（支持匿名和 GitHub OAuth）
- 文件上传管理
- 访问统计
- 基于位标志的权限控制

**技术栈:**
- **后端框架**: Tokio (异步运行时) + Axum (Web 框架) + Tower (中间件)
- **数据库**: SeaORM (异步 ORM) + SQLite
- **安全**: JWT 认证 + Argon2 密码哈希
- **前端**: React + TypeScript + Vite
- **日志**: Tracing (结构化日志)

---

## 项目结构 (Project Structure)

```
peng-blog/
├── Cargo.toml           # Workspace 配置
├── crates/              # Rust 后端代码
│   ├── app/             # 应用入口层 - 服务组装和启动
│   ├── api/             # API 层 - HTTP 路由和处理器 (51个端点)
│   ├── service/         # 业务逻辑层 - Repository Traits 和业务规则
│   ├── domain/          # 领域层 - 核心类型和错误定义
│   ├── infrastructure/  # 基础设施层 - SeaORM 实现和数据库操作
│   └── cli/             # CLI 工具 - 服务器启动和管理命令
├── frontend/            # React + TypeScript 前端
│   ├── src/
│   │   ├── api/         # API 客户端
│   │   ├── components/  # React 组件
│   │   ├── pages/       # 页面组件
│   │   ├── types/       # TypeScript 类型
│   │   └── utils/       # 工具函数
│   └── package.json
├── docs/                # 项目文档
│   └── api/                 # API 文档 (完整)
│       ├── INDEX.md         # API 总览和快速参考
│       ├── AUTH.md          # 认证 API 详细文档
│       ├── POSTS.md         # 文章 API 详细文档
│       ├── CATEGORIES.md    # 分类 API 详细文档
│       ├── TAGS.md          # 标签 API 详细文档
│       ├── USERS.md         # 用户 API 详细文档
│       ├── COMMENTS.md      # 评论 API 详细文档
│       ├── FILES.md         # 文件 API 详细文档
│       ├── STATS.md         # 统计 API 详细文档
│       ├── SESSIONS.md      # 会话 API 详细文档
│       ├── API_IMPROVEMENTS.md  # API v2 改进方案
│       └── CHANGES_v2.md    # API v2 变更记录
├── test/                # Python 测试工具
├── uploads/             # 文件上传目录
├── blog.db              # SQLite 数据库文件
└── .env                 # 环境变量配置
```

---

## 架构设计 (Architecture)

### 分层架构 (四层架构)

```
┌─────────────────────────────────────┐
│      App Layer (crates/app)        │  应用入口和依赖注入
├─────────────────────────────────────┤
│       API Layer (crates/api)       │  HTTP 处理和路由
├─────────────────────────────────────┤
│     Service Layer (crates/service) │  业务逻辑和 Repository Traits
├─────────────────────────────────────┤
│    Domain Layer (crates/domain)    │  领域类型和错误
├─────────────────────────────────────┤
│ Infrastructure Layer               │  数据访问 (SeaORM)
│ (crates/infrastructure)            │
└─────────────────────────────────────┘
```

### 依赖规则 (Dependency Rules)

**关键原则：依赖必须单向流动**

```
App → API → Service → Domain
              ↓
        Infrastructure → Domain
```

- **Domain**: 不依赖任何其他层（纯数据结构）
- **Service**: 只依赖 Domain，定义 Repository Traits
- **Infrastructure**: 依赖 Domain，实现 Service 层的 Traits
- **API**: 依赖 Service 和 Domain，处理 HTTP
- **App**: 组合所有层

### 设计模式

1. **Repository 模式**: 数据访问抽象，接口定义在 Service 层，实现放在 Infrastructure 层
2. **依赖注入**: Service 通过构造函数接收 Repository，便于测试
3. **DTO 模式**: API 使用 DTO 而非直接暴露领域实体
4. **Service 层模式**: 业务逻辑封装在 Service 中，协调 Repository 操作

---

## 构建和运行 (Build and Run)

### 环境要求

- Rust 1.70+
- SQLite 3
- Node.js 18+ (前端开发)

### 环境变量配置

创建 `.env` 文件（参考 `.env.example`）:

```env
# Database
DATABASE_URL=sqlite://blog.db

# Server
HOST=0.0.0.0
PORT=3000

# JWT Secret (修改默认值!)
JWT_SECRET=change-this-secret-in-production

# Upload
UPLOAD_DIR=./uploads
BASE_URL=http://localhost:3000

# GitHub OAuth (可选)
GITHUB_CLIENT_ID=your-client-id
GITHUB_CLIENT_SECRET=your-client-secret

# Log
RUST_LOG=debug
```

### 后端命令

```bash
# 构建项目
cargo build

# 运行开发服务器
cargo run

# 生产模式运行
cargo run --release

# 代码格式化
cargo fmt

# 代码检查
cargo clippy

# 运行测试
cargo test
cargo test -p service  # 仅测试 service 包
cargo test -- --nocapture  # 显示测试输出

# 生成覆盖率报告
cargo install cargo-tarpaulin
cargo tarpaulin --out Html --output-dir ./coverage
```

### 前端命令

```bash
cd frontend

# 安装依赖
npm install

# 开发服务器
npm run dev

# 构建
npm run build

# 代码检查
npm run lint
```

---

## CLI 工具使用 (CLI Usage)

CLI 工具既是服务器启动器，也是管理工具。

```bash
# 启动服务器
cargo run
./target/release/peng-blog

# 用户管理
cargo run -- user list
cargo run -- user show <user-id>
cargo run -- user create --username admin --password secret --admin
cargo run -- user delete <user-id> --force
cargo run -- user reset-password <user-id> --password newpass
cargo run -- user promote <user-id>
cargo run -- user demote <user-id>

# 数据库管理
cargo run -- db migrate
cargo run -- db reset --force  # 危险：清空所有数据
cargo run -- db status
```

---

## 代码风格指南 (Code Style Guidelines)

### Rust 命名规范

```rust
// 结构体/枚举: PascalCase
struct PostService;
enum Error { NotFound, Validation }

// 函数/方法: snake_case
fn create_post() {}
fn get_user_by_id() {}

// 常量: SCREAMING_SNAKE_CASE
const MAX_FILE_SIZE: u64 = 10 * 1024 * 1024;
const DEFAULT_PAGE_SIZE: u64 = 10;

// 模块: snake_case
mod post_service;
mod user_repository;
```

### 文档注释规范

```rust
/// 简短描述
///
/// # 参数
///
/// * `param1` - 参数说明
/// * `param2` - 参数说明
///
/// # 返回
///
/// 返回值说明
///
/// # 错误
///
/// 可能返回的错误类型
///
/// # 示例
///
/// ```no_run
/// let result = function_call();
/// ```
pub async fn function_name(param1: Type1, param2: Type2) -> Result<ReturnType> {
    // 实现
}
```

### 错误处理规范

```rust
use domain::{Error, Result};

// 验证错误
if input.is_empty() {
    return Err(Error::Validation("输入不能为空".to_string()));
}

// 使用 ? 传播错误
let user = self.repo.get_user(user_id).await?;

// 转换错误
self.repo.create_post(post).await
    .map_err(|e| Error::Internal(e.to_string()))?;
```

### Service 层规范

```rust
pub struct PostService<R: PostRepository> {
    repo: Arc<R>,
}

impl<R: PostRepository> PostService<R> {
    pub fn new(repo: Arc<R>) -> Self {
        Self { repo }
    }
    
    // 公共方法：验证 + 业务逻辑 + 持久化
    pub async fn create_post(&self, user_id: Uuid, request: CreatePost) -> Result<Post> {
        // 1. 验证
        let validated = self.validate_create_request(request)?;
        
        // 2. 业务逻辑
        let post = self.build_post(user_id, validated);
        
        // 3. 持久化
        self.repo.create_post(post).await
    }
    
    // 私有辅助方法
    fn validate_create_request(&self, request: CreatePost) -> Result<CreatePost> {
        // 验证逻辑
    }
}
```

---

## 测试策略 (Testing Strategy)

### 测试金字塔

```
        E2E Tests
         /    \
      /          \
   Integration  Integration
   /              \
  /                \
Unit Unit Unit Unit Unit
```

### 单元测试（Service 层）

使用 `mockall` 模拟 Repository:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use mockall::{mock, predicate::*};
    
    mock! {
        PostRepo {}
        
        #[async_trait]
        impl PostRepository for PostRepo {
            async fn create_post(&self, post: Post) -> Result<Post>;
            async fn get_post(&self, id: Uuid) -> Result<Option<Post>>;
        }
    }
    
    #[tokio::test]
    async fn test_create_post_validates_empty_title() {
        let mock_repo = MockPostRepo::new();
        let service = PostService::new(Arc::new(mock_repo));
        
        let request = CreatePost {
            title: "".to_string(),
            content: "内容".to_string(),
        };
        
        let result = service.create_post(user_id, request).await;
        
        assert!(result.is_err());
        match result {
            Err(Error::Validation(msg)) => {
                assert!(msg.contains("不能为空"));
            }
            _ => panic!("Expected validation error"),
        }
    }
}
```

### 运行测试

```bash
# 所有测试
cargo test

# 特定包
cargo test -p service
cargo test -p infrastructure

# 特定测试函数
cargo test test_create_post_success

# 显示输出
cargo test -- --nocapture

# 并行运行
cargo test -- --test-threads=4
```

### 覆盖率目标

- Service 层: ≥ 90%
- API 层: ≥ 80%
- Domain 层: ≥ 95%

---

## 权限系统 (Permission System)

使用位标志实现高效权限控制：

```rust
// 权限常量
pub const POST_CREATE: u64 = 1 << 0;   // 1
pub const POST_UPDATE: u64 = 1 << 1;   // 2
pub const POST_DELETE: u64 = 1 << 2;   // 4
pub const POST_PUBLISH: u64 = 1 << 3;  // 8
pub const USER_MANAGE: u64 = 1 << 4;   // 16 (admin only)

// 默认权限
pub const DEFAULT_USER_PERMISSIONS: u64 = POST_CREATE | POST_UPDATE | POST_PUBLISH;
pub const ADMIN_PERMISSIONS: u64 = POST_CREATE | POST_UPDATE | POST_DELETE | POST_PUBLISH | USER_MANAGE;
```

### 权限检查

```rust
// 检查特定权限
domain::check_permission(user.permissions, POST_DELETE)?;

// 检查所有权或管理员权限
domain::check_ownership_or_admin(
    resource_owner_id,
    requester_id,
    requester_permissions,
    USER_MANAGE
)?;
```

---

## 添加新功能指南 (Adding New Features)

按以下顺序添加新功能：

### 1. Domain 层 - 定义类型

```rust
// crates/domain/src/feature.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feature {
    pub id: Uuid,
    pub name: String,
    // ...
}

pub struct CreateFeature {
    pub name: String,
}
```

### 2. Service 层 - 定义接口和实现

```rust
// crates/service/src/repository.rs
#[async_trait]
pub trait FeatureRepository: Send + Sync {
    async fn create(&self, feature: Feature) -> Result<Feature>;
    // ...
}

// crates/service/src/feature.rs
pub struct FeatureService<R: FeatureRepository> {
    repo: Arc<R>,
}

impl<R: FeatureRepository> FeatureService<R> {
    pub async fn create(&self, request: CreateFeature) -> Result<Feature> {
        // 验证 + 业务逻辑
        self.repo.create(feature).await
    }
}
```

### 3. Infrastructure 层 - 实现数据库操作

```rust
// crates/infrastructure/src/feature.rs
pub struct FeatureRepositoryImpl {
    db: Arc<DatabaseConnection>,
}

#[async_trait]
impl FeatureRepository for FeatureRepositoryImpl {
    async fn create(&self, feature: Feature) -> Result<Feature> {
        // SeaORM 实现
    }
}
```

### 4. API 层 - 创建端点

```rust
// crates/api/src/feature.rs
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/features", get(list).post(create))
}

async fn create(
    State(state): State<AppState>,
    claims: Claims,
    Json(input): Json<CreateFeature>,
) -> ApiResult<impl IntoResponse> {
    let feature = state.feature_service.create(input).await
        .map_err(ApiError::Domain)?;
    Ok((StatusCode::CREATED, Json(feature)))
}
```

### 5. App 层 - 组装

```rust
// crates/app/src/lib.rs
let feature_repo = Arc::new(FeatureRepositoryImpl::new(db.clone()));
let feature_service = FeatureService::new(feature_repo);

let state = AppState::builder()
    // ... 其他服务
    .feature_service(feature_service)
    .build();
```

---

## 安全注意事项 (Security Considerations)

1. **JWT Secret**: 生产环境必须修改默认 `JWT_SECRET`
2. **密码哈希**: 使用 Argon2（内存困难型 KDF）
3. **SQL 注入**: SeaORM 使用参数化查询，禁止直接拼接 SQL
4. **权限检查**: 业务逻辑层进行权限验证，不要仅依赖前端
5. **文件上传**: 验证文件类型和大小，存储在 upload_dir 外不可访问路径

---

## 常用开发命令速查 (Command Cheat Sheet)

```bash
# 快速启动
cargo run

# 完整检查
cargo fmt && cargo clippy && cargo test

# 创建管理员用户
cargo run -- user create --username admin --password <pass> --admin

# 数据库迁移
cargo run -- db migrate

# 查看日志
tail -f server.log

# 前端开发
cd frontend && npm run dev

# 生产构建
cargo build --release
npm run build  # in frontend/
```

---

## 文档索引 (Documentation Index)

- `docs/ARCHITECTURE.md` - 详细架构设计文档
- `docs/DEVELOPMENT.md` - 开发指南和最佳实践
- `docs/CLI_USAGE.md` - CLI 工具使用指南

### API 文档 (完整)

Peng Blog API v2 提供 **RESTful API 端点**，完整文档位于 `docs/api/`：

**核心文档:**
- `docs/api/INDEX.md` - API 总览、快速开始、所有端点索引
- `docs/api/AUTH.md` - 认证系统详解（JWT、权限）
- `docs/api/POSTS.md` - 文章管理完整指南（CRUD、发布、分类、标签）
- `docs/api/CATEGORIES.md` - 分层分类系统详解
- `docs/api/TAGS.md` - 标签系统详解
- `docs/api/API_IMPROVEMENTS.md` - API v2 设计改进方案
- `docs/api/CHANGES_v2.md` - API v1 到 v2 迁移指南

**其他文档:**
- `docs/api/USERS.md` - 用户管理 API
- `docs/api/COMMENTS.md` - 评论系统 API（含 GitHub OAuth）
- `docs/api/FILES.md` - 文件上传下载 API
- `docs/api/STATS.md` - 统计分析 API
- `docs/api/SESSIONS.md` - 会话管理 API

**API v2 设计特点:**

1. **统一响应格式** - 所有响应包含 `code`, `message`, `data` 和可选的 `pagination`
2. **HTTP 方法语义化** - 使用 PATCH 进行部分更新，PUT 进行全量更新
3. **RESTful 资源层级** - `/posts/{id}/comments`, `/categories/{id}/posts`
4. **标准分页** - 使用 `page` 和 `per_page` 参数

**快速参考:**

| 模块 | 公开 | 需认证 | 管理员 |
|------|------|--------|--------|
| Auth | 注册、登录 | 用户信息 | - |
| Posts | 列表、详情、评论、标签 | 创建、更新、删除 | - |
| Users | 用户文章列表 | 用户信息 | 用户列表、权限修改 |
| Categories | 列表、详情、文章 | - | 创建、更新、删除 |
| Tags | 列表、详情、文章 | - | 创建、删除 |
| Comments | 详情 | 创建、更新、删除 | - |
| Files | 下载 | 上传、列表 | - |
| Stats | 全部 | - | - |

**主要变更 (v1 → v2):**
- `POST /posts/{id}/publish` → `PATCH /posts/{id}` + `{"status": "published"}`
- `POST /posts/{id}/tags/{tag_id}` → `POST /posts/{id}/tags` + `{"tag_id": "..."}`
- 响应格式统一包装: `{ "code": 200, "message": "success", "data": {...} }`

**权限位标志:**
```rust
POST_CREATE = 1    // 创建文章
POST_UPDATE = 2    // 更新文章
POST_DELETE = 4    // 删除文章
POST_PUBLISH = 8   // 发布文章
USER_MANAGE = 16   // 管理用户/分类/标签

// 普通用户: 11 (0x0B)
// 管理员: 31 (0x1F)
```

---

## 重要提示 (Important Notes)

1. **永远不要** 在 Domain 层引入外部依赖（除了 serde/chrono/uuid）
2. **永远不要** 在 Service 层直接进行 I/O 操作
3. **永远不要** 在 API 层编写业务逻辑
4. **永远不要** 绕过 Repository 模式直接操作数据库
5. **始终** 使用 Domain 层定义的错误类型
6. **始终** 为新功能添加测试

---

*Last updated: 2026-01-31*
*Document language: 中文 / English*
