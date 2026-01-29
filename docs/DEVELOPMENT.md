# Peng Blog 开发指南

本文档面向贡献者和开发者，提供详细的开发指南、代码规范和最佳实践。

## 目录

- [开发环境设置](#开发环境设置)
- [项目架构](#项目架构)
- [开发工作流](#开发工作流)
- [代码规范](#代码规范)
- [测试策略](#测试策略)
- [添加新功能](#添加新功能)
- [调试技巧](#调试技巧)
- [性能优化](#性能优化)
- [部署指南](#部署指南)
- [常见问题](#常见问题)

---

## 开发环境设置

### 必需工具

- **Rust 1.70+** - 主编程语言
- **Cargo** - 包管理器和构建工具
- **Git** - 版本控制
- **SQLite 3** - 数据库

### 推荐工具

- **VS Code** + **rust-analyzer** - 代码编辑器和 IDE 支持
- **cargo-watch** - 自动重新编译和运行
- **cargo-tarpaulin** - 代码覆盖率
- **cargo-audit** - 安全审计

### 安装步骤

```bash
# 1. 克隆仓库
git clone <repository-url>
cd peng-blog

# 2. 安装 Rust 工具链（如果未安装）
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 3. 安装开发工具
cargo install cargo-watch
cargo install cargo-tarpaulin
cargo install cargo-audit

# 4. 配置环境变量
cp .env.example .env
# 编辑 .env 文件

# 5. 构建项目
cargo build

# 6. 运行测试
cargo test
```

### VS Code 配置

创建 `.vscode/settings.json`：

```json
{
  "rust-analyzer.checkOnSave.command": "clippy",
  "rust-analyzer.cargo.features": "all",
  "rust-analyzer.cargo.loadOutDirsFromCheck": true,
  "rust-analyzer.inlayHints.typeHints.enable": true,
  "rust-analyzer.inlayHints.parameterHints.enable": true,
  "editor.formatOnSave": true,
  "editor.defaultFormatter": "rust-lang.rust-analyzer"
}
```

---

## 项目架构

### 分层架构

Peng Blog 采用经典的四层架构：

```
┌─────────────────────────────────────┐
│      App Layer (crates/app)        │  应用入口
├─────────────────────────────────────┤
│       API Layer (crates/api)       │  HTTP 处理
├─────────────────────────────────────┤
│     Service Layer (crates/service) │  业务逻辑
├─────────────────────────────────────┤
│    Domain Layer (crates/domain)     │  领域类型
├─────────────────────────────────────┤
│ Infrastructure Layer               │  数据访问
│ (crates/infrastructure)            │
└─────────────────────────────────────┘
```

### 依赖规则

**重要原则：依赖必须单向流动**

```
App → API → Service → Domain
              ↓
        Infrastructure → Domain
```

- **Domain 层**：不依赖任何其他层
- **Service 层**：只依赖 Domain
- **API 层**：依赖 Service 和 Domain
- **Infrastructure 层**：只依赖 Domain
- **App 层**：组合所有层

### 各层职责

#### Domain 层 (`crates/domain`)

**职责：**
- 定义核心业务实体和值对象
- 定义错误类型
- 定义业务规则和常量
- 提供统一的类型系统

**特点：**
- 纯数据结构（PORS - Plain Old Rust Structs）
- 无外部依赖
- 无 I/O 操作
- 可在前后端共享

**示例：**
```rust
// Post entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub content: String,
    pub published: bool,
    pub views: u64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub published_at: Option<DateTime<Utc>>,
}

// DTO for creating a post
#[derive(Debug, Deserialize)]
pub struct CreatePost {
    pub title: String,
    pub content: String,
    pub published: bool,
}
```

#### Service 层 (`crates/service`)

**职责：**
- 定义 Repository 接口（Trait）
- 实现业务逻辑
- 协调多个 Repository 操作
- 验证业务规则
- 编排工作流

**特点：**
- 依赖 Domain 类型
- 定义 Repository Traits（依赖倒置）
- 无 I/O（完全可测试）
- 短小专注的方法

**示例：**
```rust
#[async_trait]
pub trait PostRepository: Send + Sync {
    async fn create_post(&self, post: Post) -> Result<Post>;
    async fn get_post(&self, id: Uuid) -> Result<Option<Post>>;
    async fn update_post(&self, post: Post) -> Result<Post>;
    async fn delete_post(&self, id: Uuid) -> Result<()>;
    async fn list_posts(&self, limit: u64, offset: u64) -> Result<Vec<Post>>;
}

pub struct PostService<R: PostRepository> {
    repo: Arc<R>,
}

impl<R: PostRepository> PostService<R> {
    pub fn new(repo: Arc<R>) -> Self {
        Self { repo }
    }

    pub async fn create_post(&self, user_id: Uuid, request: CreatePost) -> Result<Post> {
        // 验证
        let title = request.title.trim();
        if title.is_empty() {
            return Err(Error::Validation("标题不能为空".to_string()));
        }
        
        // 业务逻辑
        let post = Post {
            id: Uuid::new_v4(),
            user_id,
            title: title.to_string(),
            content: request.content,
            published: request.published,
            views: 0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            published_at: if request.published { Some(Utc::now()) } else { None },
        };
        
        // 持久化
        self.repo.create_post(post).await
    }
}
```

#### Infrastructure 层 (`crates/infrastructure`)

**职责：**
- 实现 Repository Traits
- 处理数据库操作
- 管理数据库连接
- 数据库迁移

**特点：**
- 实现 Service 层定义的接口
- 使用 SeaORM 进行数据库操作
- 错误转换为 Domain 错误
- 简单直接，无抽象

**示例：**
```rust
pub struct PostRepositoryImpl {
    db: Arc<DatabaseConnection>,
}

impl PostRepositoryImpl {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl PostRepository for PostRepositoryImpl {
    async fn create_post(&self, post: Post) -> Result<Post> {
        let post_model = entity::post::ActiveModel {
            id: Set(post.id),
            user_id: Set(post.user_id),
            title: Set(post.title),
            content: Set(post.content),
            published: Set(post.published),
            views: Set(post.views),
            created_at: Set(post.created_at.naive_utc()),
            updated_at: Set(post.updated_at.naive_utc()),
            published_at: Set(post.published_at.map(|dt| dt.naive_utc())),
        };
        
        let result = post_model.insert(&*self.db).await
            .map_err(|e| Error::Internal(e.to_string()))?;
        
        Ok(result.into())
    }
    
    // 其他方法实现...
}
```

#### API 层 (`crates/api`)

**职责：**
- 定义 HTTP 路由
- 处理请求和响应
- 请求验证
- 认证和授权
- 错误转换

**特点：**
- 使用 Axum 框架
- 类型安全的路由
- 中间件支持
- 统一的错误处理

**示例：**
```rust
pub fn routes<PR, UR, SR, FR, CR, STR>() -> axum::Router<
    AppState<PR, UR, SR, FR, CR, STR>
>
where
    PR: PostRepository + Send + Sync + 'static + Clone,
    UR: UserRepository + Send + Sync + 'static + Clone,
    SR: SessionRepository + Send + Sync + 'static + Clone,
    FR: FileRepository + Send + Sync + 'static + Clone,
    CR: CommentRepository + Send + Sync + 'static + Clone,
    STR: StatsRepository + Send + Sync + 'static + Clone,
{
    axum::Router::new()
        .route("/posts", get(list_posts).post(create_post))
        .route("/posts/:id", get(get_post).put(update_post).delete(delete_post))
}

pub async fn create_post<PR, UR, SR, FR, CR, STR>(
    State(state): State<AppState<PR, UR, SR, FR, CR, STR>>,
    claims: Claims,
    Json(request): Json<CreatePost>,
) -> ApiResult<Json<Post>>
where
    PR: PostRepository + Send + Sync + 'static + Clone,
    UR: UserRepository + Send + Sync + 'static + Clone,
    SR: SessionRepository + Send + Sync + 'static + Clone,
    FR: FileRepository + Send + Sync + 'static + Clone,
    CR: CommentRepository + Send + Sync + 'static + Clone,
    STR: StatsRepository + Send + Sync + 'static + Clone,
{
    let post = state.post_service
        .create_post(claims.user_id, request)
        .await
        .map_err(ApiError::Domain)?;
    
    Ok(Json(post))
}
```

#### App 层 (`crates/app`)

**职责：**
- 应用程序入口
- 依赖注入
- 服务组装
- 服务器启动
- 配置管理

**示例：**
```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载配置
    dotenvy::dotenv().ok();
    
    // 初始化日志
    tracing_subscriber::fmt().init();
    
    // 建立数据库连接
    let db = establish_connection(&database_url).await?;
    
    // 运行迁移
    Migrator::up(&*db, None).await?;
    
    // 创建 Repositories
    let post_repo = Arc::new(PostRepositoryImpl::new(Arc::clone(&db)));
    let user_repo = Arc::new(UserRepositoryImpl::new(Arc::clone(&db)));
    
    // 创建 Services
    let post_service = PostService::new(post_repo);
    let user_service = UserService::new(user_repo);
    
    // 创建 Application State
    let state = AppState::builder()
        .post_service(post_service)
        .user_service(user_service)
        // ... 其他服务
        .build();
    
    // 创建路由
    let app = Router::new()
        .nest("/api", routes())
        .layer(TraceLayer::new_for_http())
        .with_state(state);
    
    // 启动服务器
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;
    
    Ok(())
}
```

---

## 开发工作流

### 分支策略

```bash
main           # 生产分支，只接受合并
└── develop     # 开发分支
    ├── feature/*  # 功能分支
    ├── bugfix/*   # 修复分支
    └── hotfix/*   # 紧急修复
```

### 提交规范

使用 Conventional Commits 规范：

```
<type>(<scope>): <subject>

<body>

<footer>
```

**类型 (type)：**
- `feat`: 新功能
- `fix`: 修复 bug
- `docs`: 文档更新
- `style`: 代码格式（不影响功能）
- `refactor`: 重构
- `test`: 测试相关
- `chore`: 构建/工具相关

**示例：**
```bash
feat(posts): add search functionality
fix(auth): resolve token expiration issue
docs(readme): update installation instructions
refactor(service): extract validation logic
```

### Pull Request 流程

1. **创建功能分支**
   ```bash
   git checkout -b feature/add-tag-system
   ```

2. **开发和测试**
   ```bash
   # 开发
   cargo build
   cargo test
   cargo clippy
   ```

3. **提交代码**
   ```bash
   git add .
   git commit -m "feat(posts): add tag system"
   git push origin feature/add-tag-system
   ```

4. **创建 Pull Request**
   - 填写 PR 模板
   - 关联相关 Issue
   - 请求 Code Review

5. **处理反馈**
   - 根据 Review 意见修改
   - 更新 PR

6. **合并**
   - 通过 CI 检查
   - 获得 LGTM (Looks Good To Me)
   - 合并到 develop

### 代码审查清单

在提交 PR 前，确保：

- [ ] 代码通过 `cargo clippy` 检查
- [ ] 所有测试通过 (`cargo test`)
- [ ] 添加了必要的测试
- [ ] 更新了相关文档
- [ ] 没有引入新的警告
- [ ] 遵循项目代码风格
- [ ] 提交信息符合规范

---

## 代码规范

### Rust 代码风格

遵循 [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) 和官方 [RFC](https://rust-lang.github.io/rfcs/)。

#### 命名规范

```rust
// 结构体和枚举：PascalCase
struct PostService;
enum Error { NotFound, Validation }

// 函数和方法：snake_case
fn create_post() {}
fn get_user_by_id() {}

// 常量：SCREAMING_SNAKE_CASE
const MAX_FILE_SIZE: u64 = 10 * 1024 * 1024;
const DEFAULT_PAGE_SIZE: u64 = 10;

// 模块：snake_case
mod post_service;
mod user_repository;

// 类型参数：简短、大写
impl<T> Service<T> {}
fn process<R>(repo: R) {}
```

#### 文档注释

```rust
/// 创建新文章
///
/// # 参数
///
/// * `user_id` - 创建文章的用户 ID
/// * `request` - 文章创建请求
///
/// # 返回
///
/// 返回创建成功的文章对象
///
/// # 错误
///
/// 如果验证失败，返回 `Error::Validation`
/// 如果数据库错误，返回 `Error::Internal`
///
/// # 示例
///
/// ```no_run
/// use service::{PostService, CreatePost};
/// # use std::sync::Arc;
/// # let service = PostService::new(/* ... */);
/// # let user_id = uuid::Uuid::new_v4();
/// let request = CreatePost {
///     title: "我的文章".to_string(),
///     content: "文章内容".to_string(),
///     published: false,
/// };
/// let post = service.create_post(user_id, request).await?;
/// # Ok::<(), domain::Error>(())
/// ```
pub async fn create_post(&self, user_id: Uuid, request: CreatePost) -> Result<Post> {
    // 实现
}
```

#### 错误处理

```rust
// 使用 Domain 层定义的统一错误类型
use domain::{Error, Result};

// 验证错误
if input.is_empty() {
    return Err(Error::Validation("输入不能为空".to_string()));
}

// 使用 ? 操作符传播错误
let user = self.repo.get_user(user_id).await?;

// 转换错误
self.repo.create_post(post).await
    .map_err(|e| Error::Internal(e.to_string()))?;
```

#### 异步代码

```rust
// 使用 async/await
pub async fn create_post(&self, request: CreatePost) -> Result<Post> {
    let post = validate_and_build(request)?;
    self.repo.create_post(post).await
}

// 使用 Arc 共享状态
pub struct AppState {
    post_service: Arc<PostService<PR>>,
    user_service: Arc<UserService<UR>>,
}

// 使用 Send + Sync 约束
#[async_trait]
pub trait PostRepository: Send + Sync {
    async fn create_post(&self, post: Post) -> Result<Post>;
}
```

#### 测试代码

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    
    #[tokio::test]
    async fn test_create_post_validates_title() {
        let mut mock_repo = MockPostRepository::new();
        mock_repo.expect_create_post()
            .never();  // 不应该调用创建方法
        
        let service = PostService::new(Arc::new(mock_repo));
        let request = CreatePost {
            title: "".to_string(),  // 空标题
            content: "内容".to_string(),
            published: false,
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

### 项目特定规范

#### Service 层

```rust
// Service 构造函数接受 Arc<Repository>
pub struct PostService<R: PostRepository> {
    repo: Arc<R>,
}

impl<R: PostRepository> PostService<R> {
    pub fn new(repo: Arc<R>) -> Self {
        Self { repo }
    }
    
    // 公共方法验证 + 业务逻辑
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
        let title = request.title.trim();
        if title.len() < 3 || title.len() > 200 {
            return Err(Error::Validation("标题长度必须在3-200个字符之间".to_string()));
        }
        Ok(CreatePost {
            title: title.to_string(),
            ..request
        })
    }
}
```

#### API 层

```rust
// 使用统一的状态类型
pub async fn handler<PR, UR, SR, FR, CR, STR>(
    State(state): State<AppState<PR, UR, SR, FR, CR, STR>>,
    Json(input): Json<InputType>,
) -> ApiResult<Json<OutputType>>
where
    // Repository 约束
    PR: PostRepository + Send + Sync + 'static + Clone,
    // ... 其他约束
{
    // 调用 Service
    let output = state.post_service
        .some_operation(input)
        .await
        .map_err(ApiError::Domain)?;
    
    // 返回结果
    Ok(Json(output))
}
```

#### Infrastructure 层

```rust
// 简单直接，无额外抽象
impl PostRepository for PostRepositoryImpl {
    async fn create_post(&self, post: Post) -> Result<Post> {
        let active_model = entity::post::ActiveModel {
            id: Set(post.id),
            // ... 其他字段
        };
        
        let result = active_model.insert(&*self.db).await
            .map_err(|e| Error::Internal(e.to_string()))?;
        
        Ok(result.into())
    }
}
```

---

## 测试策略

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
    async fn test_create_post_success() {
        let mut mock_repo = MockPostRepo::new();
        
        mock_repo.expect_create_post()
            .times(1)
            .with(predicate::always())
            .returning(|post| Ok(post));
        
        let service = PostService::new(Arc::new(mock_repo));
        let request = CreatePost {
            title: "测试文章".to_string(),
            content: "测试内容".to_string(),
            published: false,
        };
        
        let result = service.create_post(user_id, request).await;
        
        assert!(result.is_ok());
    }
}
```

### 集成测试（API 层）

```rust
// tests/integration_test.rs
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;
use serde_json::json;

#[tokio::test]
async fn test_create_post_endpoint() {
    // 创建测试应用
    let app = create_test_app().await;
    
    // 准备请求数据
    let request = Request::builder()
        .method("POST")
        .uri("/api/posts")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", test_token()))
        .body(Body::from(json!({
            "title": "测试文章",
            "content": "测试内容",
            "published": false
        }).to_string()))
        .unwrap();
    
    // 发送请求
    let response = app.oneshot(request).await.unwrap();
    
    // 验证响应
    assert_eq!(response.status(), StatusCode::CREATED);
}
```

### 运行测试

```bash
# 运行所有测试
cargo test

# 运行特定模块的测试
cargo test -p service

# 运行特定测试函数
cargo test test_create_post_success

# 显示测试输出
cargo test -- --nocapture

# 并行运行测试（更快）
cargo test -- --test-threads=4

# 生成覆盖率报告
cargo tarpaulin --out Html
```

### 测试覆盖率

```bash
# 安装 tarpaulin
cargo install cargo-tarpaulin

# 生成覆盖率
cargo tarpaulin --workspace --out Html --output-dir ./coverage

# 查看覆盖率报告
open coverage/index.html  # macOS
xdg-open coverage/index.html  # Linux
```

目标覆盖率：
- Service 层：≥ 90%
- API 层：≥ 80%
- Domain 层：≥ 95%

---

## 添加新功能

### 完整示例：添加标签功能

#### 1. Domain 层：定义类型

```rust
// crates/domain/src/tag.rs
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Tag {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTag {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTag {
    pub name: String,
}

impl Tag {
    pub fn new(name: String) -> Self {
        let slug = name
            .to_lowercase()
            .replace(" ", "-")
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '-')
            .collect();
        
        Self {
            id: Uuid::new_v4(),
            name,
            slug,
            created_at: Utc::now(),
        }
    }
}

// crates/domain/src/lib.rs
pub mod tag;
pub use tag::{Tag, CreateTag, UpdateTag};
```

#### 2. Service 层：定义接口和实现

```rust
// crates/service/src/repository.rs
#[async_trait]
pub trait TagRepository: Send + Sync {
    async fn create_tag(&self, tag: Tag) -> Result<Tag>;
    async fn get_tag(&self, id: Uuid) -> Result<Option<Tag>>;
    async fn list_tags(&self) -> Result<Vec<Tag>>;
    async fn update_tag(&self, tag: Tag) -> Result<Tag>;
    async fn delete_tag(&self, id: Uuid) -> Result<()>;
}

// crates/service/src/tag.rs
use crate::repository::TagRepository;
use domain::{Tag, CreateTag, UpdateTag, Error, Result};
use std::sync::Arc;

pub struct TagService<R: TagRepository> {
    repo: Arc<R>,
}

impl<R: TagRepository> TagService<R> {
    pub fn new(repo: Arc<R>) -> Self {
        Self { repo }
    }
    
    pub async fn create_tag(&self, request: CreateTag) -> Result<Tag> {
        // 验证
        let name = request.name.trim();
        if name.is_empty() {
            return Err(Error::Validation("标签名称不能为空".to_string()));
        }
        if name.len() > 50 {
            return Err(Error::Validation("标签名称不能超过50个字符".to_string()));
        }
        
        // 创建
        let tag = Tag::new(name.to_string());
        self.repo.create_tag(tag).await
    }
    
    pub async fn get_tag(&self, id: Uuid) -> Result<Tag> {
        self.repo.get_tag(id).await?
            .ok_or_else(|| Error::NotFound("标签不存在".to_string()))
    }
    
    pub async fn list_tags(&self) -> Result<Vec<Tag>> {
        self.repo.list_tags().await
    }
    
    pub async fn update_tag(&self, id: Uuid, request: UpdateTag) -> Result<Tag> {
        let name = request.name.trim();
        if name.is_empty() {
            return Err(Error::Validation("标签名称不能为空".to_string()));
        }
        
        let mut tag = self.get_tag(id).await?;
        tag.name = name.to_string();
        tag.slug = name.to_lowercase().replace(" ", "-");
        
        self.repo.update_tag(tag).await
    }
    
    pub async fn delete_tag(&self, id: Uuid) -> Result<()> {
        self.get_tag(id).await?;  // 检查是否存在
        self.repo.delete_tag(id).await
    }
}

// crates/service/src/lib.rs
pub mod tag;
pub use tag::TagService;
```

#### 3. Infrastructure 层：实现数据库操作

```rust
// crates/infrastructure/src/entity/tag.rs
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "tag")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

// crates/infrastructure/src/tag.rs
use sea_orm::*;
use domain::{Tag, Error, Result};
use service::TagRepository;
use crate::entity::tag;

pub struct TagRepositoryImpl {
    db: Arc<DatabaseConnection>,
}

impl TagRepositoryImpl {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl TagRepository for TagRepositoryImpl {
    async fn create_tag(&self, tag: Tag) -> Result<Tag> {
        let tag_model = tag::ActiveModel {
            id: Set(tag.id),
            name: Set(tag.name),
            slug: Set(tag.slug),
            created_at: Set(tag.created_at.naive_utc()),
        };
        
        let result = tag_model.insert(&*self.db).await
            .map_err(|e| Error::Internal(e.to_string()))?;
        
        Ok(Tag {
            id: result.id,
            name: result.name,
            slug: result.slug,
            created_at: DateTime::from_utc(result.created_at, Utc),
        })
    }
    
    async fn get_tag(&self, id: Uuid) -> Result<Option<Tag>> {
        let result = tag::Entity::find_by_id(id)
            .one(&*self.db)
            .await
            .map_err(|e| Error::Internal(e.to_string()))?;
        
        Ok(result.map(|model| Tag {
            id: model.id,
            name: model.name,
            slug: model.slug,
            created_at: DateTime::from_utc(model.created_at, Utc),
        }))
    }
    
    async fn list_tags(&self) -> Result<Vec<Tag>> {
        let results = tag::Entity::find()
            .order_by_desc(tag::Column::CreatedAt)
            .all(&*self.db)
            .await
            .map_err(|e| Error::Internal(e.to_string()))?;
        
        Ok(results.into_iter().map(|model| Tag {
            id: model.id,
            name: model.name,
            slug: model.slug,
            created_at: DateTime::from_utc(model.created_at, Utc),
        }).collect())
    }
    
    async fn update_tag(&self, tag: Tag) -> Result<Tag> {
        let tag_model = tag::ActiveModel {
            id: Set(tag.id),
            name: Set(tag.name),
            slug: Set(tag.slug),
            created_at: Set(tag.created_at.naive_utc()),
        };
        
        let result = tag_model.update(&*self.db).await
            .map_err(|e| Error::Internal(e.to_string()))?;
        
        Ok(Tag {
            id: result.id,
            name: result.name,
            slug: result.slug,
            created_at: DateTime::from_utc(result.created_at, Utc),
        })
    }
    
    async fn delete_tag(&self, id: Uuid) -> Result<()> {
        tag::Entity::delete_by_id(id)
            .exec(&*self.db)
            .await
            .map_err(|e| Error::Internal(e.to_string()))?;
        Ok(())
    }
}

// crates/infrastructure/src/lib.rs
pub mod tag;
```

#### 4. 创建数据库迁移

```rust
// crates/infrastructure/src/migrations/src/m20250129_000001_create_tag.rs
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Tag::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Tag::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Tag::Name).string().not_null())
                    .col(ColumnDef::new(Tag::Slug).string().not_null().unique_key())
                    .col(ColumnDef::new(Tag::CreatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Tag::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Tag {
    Table,
    Id,
    Name,
    Slug,
    CreatedAt,
}
```

#### 5. API 层：添加路由和处理器

```rust
// crates/api/src/tag.rs
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use domain::{Tag, CreateTag, UpdateTag};

pub fn routes<PR, UR, SR, FR, CR, STR>() -> axum::Router<
    AppState<PR, UR, SR, FR, CR, STR>
>
where
    PR: PostRepository + Send + Sync + 'static + Clone,
    UR: UserRepository + Send + Sync + 'static + Clone,
    SR: SessionRepository + Send + Sync + 'static + Clone,
    FR: FileRepository + Send + Sync + 'static + Clone,
    CR: CommentRepository + Send + Sync + 'static + Clone,
    STR: StatsRepository + Send + Sync + 'static + Clone,
{
    axum::Router::new()
        .route("/tags", get(list_tags).post(create_tag))
        .route("/tags/:id", get(get_tag).put(update_tag).delete(delete_tag))
}

pub async fn create_tag<PR, UR, SR, FR, CR, STR>(
    State(state): State<AppState<PR, UR, SR, FR, CR, STR>>,
    Json(request): Json<CreateTag>,
) -> ApiResult<Json<Tag>>
where
    PR: PostRepository + Send + Sync + 'static + Clone,
    UR: UserRepository + Send + Sync + 'static + Clone,
    SR: SessionRepository + Send + Sync + 'static + Clone,
    FR: FileRepository + Send + Sync + 'static + Clone,
    CR: CommentRepository + Send + Sync + 'static + Clone,
    STR: StatsRepository + Send + Sync + 'static + Clone,
{
    let tag = state.tag_service
        .create_tag(request)
        .await
        .map_err(ApiError::Domain)?;
    
    Ok(Json(tag))
}

pub async fn list_tags<PR, UR, SR, FR, CR, STR>(
    State(state): State<AppState<PR, UR, SR, FR, CR, STR>>,
) -> ApiResult<Json<Vec<Tag>>>
where
    PR: PostRepository + Send + Sync + 'static + Clone,
    UR: UserRepository + Send + Sync + 'static + Clone,
    SR: SessionRepository + Send + Sync + 'static + Clone,
    FR: FileRepository + Send + Sync + 'static + Clone,
    CR: CommentRepository + Send + Sync + 'static + Clone,
    STR: StatsRepository + Send + Sync + 'static + Clone,
{
    let tags = state.tag_service
        .list_tags()
        .await
        .map_err(ApiError::Domain)?;
    
    Ok(Json(tags))
}

pub async fn get_tag<PR, UR, SR, FR, CR, STR>(
    State(state): State<AppState<PR, UR, SR, FR, CR, STR>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<Tag>>
where
    PR: PostRepository + Send + Sync + 'static + Clone,
    UR: UserRepository + Send + Sync + 'static + Clone,
    SR: SessionRepository + Send + Sync + 'static + Clone,
    FR: FileRepository + Send + Sync + 'static + Clone,
    CR: CommentRepository + Send + Sync + 'static + Clone,
    STR: StatsRepository + Send + Sync + 'static + Clone,
{
    let tag = state.tag_service
        .get_tag(id)
        .await
        .map_err(ApiError::Domain)?;
    
    Ok(Json(tag))
}

pub async fn update_tag<PR, UR, SR, FR, CR, STR>(
    State(state): State<AppState<PR, UR, SR, FR, CR, STR>>,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateTag>,
) -> ApiResult<Json<Tag>>
where
    PR: PostRepository + Send + Sync + 'static + Clone,
    UR: UserRepository + Send + Sync + 'static + Clone,
    SR: SessionRepository + Send + Sync + 'static + Clone,
    FR: FileRepository + Send + Sync + 'static + Clone,
    CR: CommentRepository + Send + Sync + 'static + Clone,
    STR: StatsRepository + Send + Sync + 'static + Clone,
{
    let tag = state.tag_service
        .update_tag(id, request)
        .await
        .map_err(ApiError::Domain)?;
    
    Ok(Json(tag))
}

pub async fn delete_tag<PR, UR, SR, FR, CR, STR>(
    State(state): State<AppState<PR, UR, SR, FR, CR, STR>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>>
where
    PR: PostRepository + Send + Sync + 'static + Clone,
    UR: UserRepository + Send + Sync + 'static + Clone,
    SR: SessionRepository + Send + Sync + 'static + Clone,
    FR: FileRepository + Send + Sync + 'static + Clone,
    CR: CommentRepository + Send + Sync + 'static + Clone,
    STR: StatsRepository + Send + Sync + 'static + Clone,
{
    state.tag_service
        .delete_tag(id)
        .await
        .map_err(ApiError::Domain)?;
    
    Ok(Json(serde_json::json!({"message": "标签删除成功"})))
}

// crates/api/src/lib.rs
pub mod tag;

pub fn routes<PR, UR, SR, FR, CR, STR>() -> Router<...> {
    Router::new()
        .nest("/tags", tag::routes::<PR, UR, SR, FR, CR, STR>())
        // ... 其他路由
}
```

#### 6. App 层：依赖注入

```rust
// crates/app/src/main.rs
use infrastructure::TagRepositoryImpl;
use service::TagService;

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ... 现有代码 ...
    
    // 创建 Tag Repository
    let tag_repo = Arc::new(TagRepositoryImpl::new(Arc::clone(&db)));
    
    // 创建 Tag Service
    let tag_service = TagService::new(tag_repo);
    
    // 更新 AppState
    let state = AppState::builder()
        // ... 现有服务
        .tag_service(tag_service)
        .build();
    
    // ... 其余代码 ...
}
```

---

## 调试技巧

### 日志调试

```rust
use tracing::{info, debug, error, instrument};

#[instrument(skip(self))]
pub async fn create_post(&self, user_id: Uuid, request: CreatePost) -> Result<Post> {
    debug!("Creating post for user {}", user_id);
    
    let post = self.validate_and_build(user_id, request)?;
    info!("Post validated: {}", post.title);
    
    let result = self.repo.create_post(post.clone()).await
        .map_err(|e| {
            error!("Failed to create post: {}", e);
            Error::Internal(e.to_string())
        })?;
    
    info!("Post created successfully with ID {}", result.id);
    Ok(result)
}
```

设置日志级别：

```bash
# 开发环境 - 详细日志
RUST_LOG=debug cargo run

# 生产环境 - 仅重要日志
RUST_LOG=info cargo run

# 特定模块
RUST_LOG=peng_blog::service=debug,tower_http=info cargo run
```

### 数据库调试

```bash
# 连接到数据库
sqlite3 blog.db

# 查看所有表
.tables

# 查看表结构
.schema post

# 查看数据
SELECT * FROM post LIMIT 10;

# 查看索引
.indices post

# 退出
.quit
```

启用 SeaORM SQL 日志：

```rust
let db = Database::connect(&url).await?;
// 在开发环境中启用 SQL 日志
if cfg!(debug_assertions) {
    db.set_debug(true);
}
```

### 使用调试器

```rust
// 在代码中添加断点
#[cfg(debug_assertions)]
eprintln!("DEBUG: user_id = {:?}, post_id = {:?}", user_id, post_id);

// 使用 dbg! 宏（自动打印）
let result = dbg!(some_complex_calculation());
```

### 性能分析

```bash
# 使用 flamegraph
cargo install flamegraph
cargo flamegraph

# 使用 tokio-console（需要启用 feature）
tokio-console

# 使用 tracing-distributed
tracing-distributed-export
```

### 错误处理最佳实践

```rust
// 1. 使用具体的错误类型
match error {
    Error::Validation(msg) => {
        return Err(ApiError::Validation(msg));
    }
    Error::NotFound(msg) => {
        return Err(ApiError::NotFound(msg));
    }
    Error::Internal(msg) => {
        tracing::error!("Internal error: {}", msg);
        return Err(ApiError::Internal(msg));
    }
}

// 2. 使用 context 提供更多上下文
self.repo.get_post(id).await
    .map_err(|e| Error::Internal(format!("Failed to get post {}: {}", id, e)))?;

// 3. 早期返回避免嵌套
pub async fn get_post(&self, id: Uuid) -> Result<Post> {
    let post = self.repo.get_post(id).await?;
    let post = post.ok_or(Error::NotFound("Post not found".to_string()))?;
    
    if !post.published {
        return Err(Error::NotFound("Post not published".to_string()));
    }
    
    Ok(post)
}
```

---

## 性能优化

### 数据库优化

```rust
// 1. 使用索引
#[derive(DeriveIden)]
enum Post {
    Table,
    Id,
    UserId,  // 添加索引
    CreatedAt,  // 添加索引
}

// 在迁移中创建索引
manager.create_index(
    Index::create()
        .name("idx_post_user_id")
        .table(Post::Table)
        .col(Post::UserId)
        .to_owned()
).await?;

// 2. 使用 JOIN 避免N+1查询
let posts_with_users: Vec<_> = Post::find()
    .find_also_related(User)  // 自动JOIN
    .all(&db)
    .await?;

// 3. 分页查询
let page = 1;
let page_size = 10;
let posts = Post::find()
    .order_by_desc(post::Column::CreatedAt)
    .paginate(&db, page_size)
    .fetch_page(page - 1)
    .await?;
```

### 异步优化

```rust
// 1. 并发执行多个独立任务
let (posts, comments, stats) = tokio::join!(
    post_service.list_posts(),
    comment_service.list_comments(),
    stats_service.get_stats()
);

// 2. 使用 try_join 处理结果
let (posts, comments) = tokio::try_join!(
    post_service.list_posts(),
    comment_service.list_comments()
)?;

// 3. 批量操作
let post_ids = vec![id1, id2, id3];
let posts = Post::find()
    .filter(post::Column::Id.is_in(post_ids))
    .all(&db)
    .await?;
```

### 缓存策略

```rust
use std::collections::HashMap;
use std::sync::RwLock;

pub struct CachedPostService<R: PostRepository> {
    repo: Arc<R>,
    cache: Arc<RwLock<HashMap<Uuid, Post>>>,
}

impl<R: PostRepository> CachedPostService<R> {
    pub async fn get_post(&self, id: Uuid) -> Result<Post> {
        // 尝试从缓存读取
        {
            let cache = self.cache.read().await;
            if let Some(post) = cache.get(&id) {
                return Ok(post.clone());
            }
        }
        
        // 从数据库加载
        let post = self.repo.get_post(id).await?
            .ok_or(Error::NotFound("Post not found".to_string()))?;
        
        // 写入缓存
        {
            let mut cache = self.cache.write().await;
            cache.insert(id, post.clone());
        }
        
        Ok(post)
    }
}
```

### 内存优化

```rust
// 1. 使用 Box 减少栈空间
struct LargeData {
    content: Box<String>,  // 堆分配
}

// 2. 使用 Cow 避免不必要的克隆
use std::borrow::Cow;

fn process(input: Cow<str>) -> String {
    input.to_uppercase()  // Cow 自动处理借用/拥有
}

// 3. 使用 String::from 和 into() 减少拷贝
let s = String::from("hello");  // 比 "hello".to_string() 更快
```

---

## 部署指南

### Docker 部署

```dockerfile
# Dockerfile
FROM rust:1.70-slim as builder

WORKDIR /app

# 复制依赖配置
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates

# 构建发布版本
RUN cargo build --release

# 运行时镜像
FROM debian:bookworm-slim

WORKDIR /app

# 安装运行时依赖
RUN apt-get update && apt-get install -y \
    ca-certificates \
    sqlite3 \
    && rm -rf /var/lib/apt/lists/*

# 复制二进制文件
COPY --from=builder /app/target/release/app /app/peng-blog

# 创建目录
RUN mkdir -p /app/uploads /app/data

# 设置权限
RUN chmod +x /app/peng-blog

# 暴露端口
EXPOSE 3000

# 健康检查
HEALTHCHECK --interval=30s --timeout=3s \
    CMD curl -f http://localhost:3000/api/stats/visits || exit 1

# 启动应用
CMD ["/app/peng-blog"]
```

```yaml
# docker-compose.yml
version: '3.8'

services:
  app:
    build: .
    ports:
      - "3000:3000"
    environment:
      - DATABASE_URL=sqlite:///data/blog.db
      - JWT_SECRET=${JWT_SECRET}
      - UPLOAD_DIR=/app/uploads
      - BASE_URL=https://yourdomain.com
      - GITHUB_CLIENT_ID=${GITHUB_CLIENT_ID}
      - GITHUB_CLIENT_SECRET=${GITHUB_CLIENT_SECRET}
      - RUST_LOG=info
    volumes:
      - ./data:/app/data
      - ./uploads:/app/uploads
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:3000/api/stats/visits"]
      interval: 30s
      timeout: 10s
      retries: 3
```

### 生产环境配置

```env
# .env.production
DATABASE_URL=postgresql://user:pass@db-host:5432/blog
JWT_SECRET=<使用 openssl rand -base64 32 生成>
UPLOAD_DIR=/var/lib/peng-blog/uploads
BASE_URL=https://blog.yourdomain.com
GITHUB_CLIENT_ID=your-github-client-id
GITHUB_CLIENT_SECRET=your-github-client-secret

RUST_LOG=warn,peng_blog=info
RUST_BACKTRACE=1

# 性能调优
TOKIO_WORKER_THREADS=4
TOKIO_MAX_BLOCKING_THREADS=512
```

### 反向代理配置

```nginx
# /etc/nginx/sites-available/peng-blog
server {
    listen 80;
    server_name blog.yourdomain.com;
    return 301 https://$server_name$request_uri;
}

server {
    listen 443 ssl http2;
    server_name blog.yourdomain.com;

    ssl_certificate /etc/ssl/certs/blog.yourdomain.com.crt;
    ssl_certificate_key /etc/ssl/private/blog.yourdomain.com.key;
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers HIGH:!aNULL:!MD5;

    # 上传文件大小限制
    client_max_body_size 10M;

    # 代理到应用
    location / {
        proxy_pass http://127.0.0.1:3000;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_cache_bypass $http_upgrade;
    }

    # 静态文件
    location /static/ {
        alias /var/lib/peng-blog/static/;
        expires 1y;
        add_header Cache-Control "public, immutable";
    }

    # 上传文件
    location /uploads/ {
        alias /var/lib/peng-blog/uploads/;
        expires 1y;
        add_header Cache-Control "public, immutable";
    }
}
```

### 监控和日志

```bash
# 使用 systemd 管理
# /etc/systemd/system/peng-blog.service
[Unit]
Description=Peng Blog API
After=network.target

[Service]
Type=simple
User=peng-blog
WorkingDirectory=/var/lib/peng-blog
Environment="PATH=/usr/bin"
EnvironmentFile=/var/lib/peng-blog/.env
ExecStart=/var/lib/peng-blog/peng-blog
Restart=always
RestartSec=10

# 日志
StandardOutput=append:/var/log/peng-blog/app.log
StandardError=append:/var/log/peng-blog/error.log

# 安全
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=/var/lib/peng-blog/uploads /var/lib/peng-blog/data

[Install]
WantedBy=multi-user.target

# 启动服务
sudo systemctl daemon-reload
sudo systemctl enable peng-blog
sudo systemctl start peng-blog
sudo systemctl status peng-blog
```

---

## 常见问题

### 编译问题

**Q: 编译时出现 "feature is required" 错误**

A: 检查 Cargo.toml 中的 feature 配置：
```toml
[features]
default = ["sqlite"]
postgresql = ["sea-orm/sqlx-postgres"]
mysql = ["sea-orm/sqlx-mysql"]
```

**Q: 依赖版本冲突**

A: 使用 cargo update 更新依赖：
```bash
cargo update
# 或者更新特定包
cargo update -p sea-orm
```

### 运行时问题

**Q: 数据库连接失败**

A: 检查以下内容：
1. DATABASE_URL 格式是否正确
2. 数据库服务是否运行
3. 连接字符串中的用户名密码是否正确
4. 防火墙是否阻止连接

**Q: 内存占用过高**

A: 尝试以下优化：
1. 减少连接池大小
2. 使用分页查询
3. 启用缓存
4. 使用 `cargo flamegraph` 分析热点

### 开发问题

**Q: 如何快速测试 API？**

A: 使用以下工具：
```bash
# 安装 httpie
pip install httpie

# 使用 httpie
http POST localhost:3000/api/auth/register username=test password=pass123

# 或使用 curl
curl -X POST http://localhost:3000/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{"username":"test","password":"pass123"}'
```

**Q: 如何调试异步代码？**

A: 使用以下方法：
```rust
// 启用 tracing
use tracing::{info, debug, instrument};

#[instrument(skip(self))]
pub async fn some_async_function(&self) -> Result<()> {
    debug!("Starting async operation");
    // ...
}

// 使用 tokio-console
tokio-console
```

### 性能问题

**Q: 数据库查询慢**

A: 优化建议：
1. 添加合适的索引
2. 使用 EXPLAIN ANALYZE 分析查询
3. 避免N+1查询
4. 使用分页
5. 考虑缓存热点数据

**Q: 响应时间过长**

A: 排查步骤：
1. 检查数据库查询时间
2. 检查网络延迟
3. 启用性能分析
4. 考虑使用 CDN
5. 优化数据序列化

---

## 资源和链接

### 官方文档

- [Rust 官方文档](https://doc.rust-lang.org/)
- [Tokio 异步运行时](https://tokio.rs/)
- [Axum Web 框架](https://github.com/tokio-rs/axum)
- [SeaORM 文档](https://www.sea-orm.org/)

### 学习资源

- [Rust 程序设计语言](https://kaisery.github.io/trpl-zh-cn/)
- [Rust by Example](https://rustwiki.org/zh-CN/rust-by-example/)
- [Async Rust](https://rust-lang.github.io/async-book/)

### 工具

- [rust-analyzer](https://github.com/rust-analyzer/rust-analyzer) - LSP 实现
- [cargo-watch](https://github.com/passcod/cargo-watch) - 自动运行测试
- [cargo-expand](https://github.com/dtolnay/cargo-expand) - 宏展开
- [cargo-edit](https://github.com/killercup/cargo-edit) - Cargo 扩展

---

## 贡献

我们欢迎所有形式的贡献！

### 如何贡献

1. Fork 项目
2. 创建功能分支
3. 提交代码（遵循提交规范）
4. 推送到分支
5. 创建 Pull Request

### 代码审查

所有代码都需要经过 Code Review，确保：
- 代码符合项目规范
- 包含适当的测试
- 文档完整
- 没有引入技术债务

### 问题报告

报告问题时，请提供：
- 清晰的问题描述
- 复现步骤
- 预期行为
- 实际行为
- 环境信息（Rust 版本、操作系统等）

---

**文档版本：** 1.0.0  
**最后更新：** 2026-01-29  
**维护者：** Peng Blog Team