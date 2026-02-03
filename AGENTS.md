# AGENTS.md - Peng Blog

> AI编码代理工作指南 - 项目架构、构建命令和代码规范

## 项目概述

**Peng Blog** - Rust + React博客系统，分层架构（四层）

**技术栈:**
- 后端: Tokio + Axum + SeaORM + PostgreSQL
- 前端: React + TypeScript + Vite + FluentUI
- 安全: JWT + Argon2，位标志权限系统

## 项目结构

```
peng-blog/
├── crates/
│   ├── app/             # 应用入口 - 依赖注入
│   ├── api/             # HTTP路由 - 处理器
│   ├── service/         # 业务逻辑 - Repository Traits
│   ├── domain/          # 核心类型 - 零依赖
│   ├── infrastructure/  # 数据访问 - SeaORM实现
│   ├── config/          # 配置管理
│   └── cli/             # CLI工具
├── frontend/            # React前端
└── docs/api/            # API文档
```

**架构依赖规则:**
```
App → API → Service → Domain
              ↓
        Infrastructure → Domain
```

**关键原则:**
- Domain: 不依赖任何其他层（除了serde/chrono/uuid/async-trait）
- Service: 定义Repository Trait，依赖Domain
- Infrastructure: 实现Repository，依赖Domain
- API: 依赖Service+Domain，不直接访问Infrastructure

## 构建命令

### 后端 (Rust)

```bash
# 构建
cargo build                    # 开发构建
cargo build --release          # 生产构建

# 运行
cargo run                      # 启动服务器
cargo run --release            # 生产模式

# 测试
cargo test                     # 所有测试
cargo test -p service          # 单个crate
cargo test test_name           # 单个测试（模糊匹配）
cargo test test_name -- --exact  # 精确匹配
cargo test -- --nocapture      # 显示测试输出
cargo test -- --test-threads=1 # 单线程运行
cargo test service::tests::test_name  # 特定测试

# 快速检查
cargo check                    # 类型检查（不构建）
cargo clippy                   # Lint检查
cargo fmt                      # 格式化代码
cargo fmt --check              # 检查格式（不修改）
```

### 前端 (TypeScript)

```bash
cd frontend
npm run dev                    # 开发服务器
npm run build                  # 生产构建
npm run type-check             # TypeScript检查
npm run lint                   # ESLint
npm run format                 # Prettier格式化
```

### CLI工具

```bash
cargo run -- user list
cargo run -- user create --username admin --password pass --admin
cargo run -- db migrate
cargo run -- db reset --force
```

## 代码风格

### Rust命名规范

```rust
struct PostService;            // PascalCase - 结构体
enum Error { NotFound }        // PascalCase - 枚举
fn create_post() {}            // snake_case - 函数
const MAX_SIZE: u64 = 100;     // SCREAMING_SNAKE_CASE - 常量
mod post_service;              // snake_case - 模块
type Result<T> = ...;          // PascalCase - 类型别名
```

### Rust导入顺序

```rust
// 1. 标准库
use std::sync::Arc;

// 2. 第三方库（按字母顺序）
use async_trait::async_trait;
use uuid::Uuid;

// 3. 本地crate（按字母顺序）
use domain::{Error, Result, User};
use service::UserService;

// 4. 同crate内
use crate::error::ApiError;
use crate::models::Post;
```

### 错误处理模式

**Domain层错误:**
```rust
use domain::{Error, Result};

// 验证错误
if input.is_empty() {
    return Err(Error::Validation("输入不能为空".to_string()));
}

// 资源未找到
Err(Error::NotFound("User not found".to_string()))

// 传播错误（使用?操作符）
let user = self.repo.get_user(id).await?;

// 转换错误类型
self.repo.create(post).await
    .map_err(|e| Error::Internal(e.to_string()))?;
```

**API层错误转换:**
```rust
// Domain错误自动转换为API错误
let user = self.user_service.get(id).await
    .map_err(ApiError::Domain)?;
```

### Repository Trait定义（在Service层）

```rust
use async_trait::async_trait;
use domain::{Result, User};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>>;
    async fn create_user(&self, username: String, password: String, permissions: u64) -> Result<User>;
    async fn find_by_username(&self, username: &str) -> Result<Option<User>>;
}
```

### Service层模式

```rust
use domain::{Result, User, DEFAULT_USER_PERMISSIONS};
use std::sync::Arc;

pub struct UserService {
    repo: Arc<dyn UserRepository>,
    allow_registration: bool,
}

impl UserService {
    pub fn new(repo: Arc<dyn UserRepository>, allow_registration: bool) -> Self {
        Self { repo, allow_registration }
    }

    pub async fn register(&self, username: String, password: String) -> Result<User> {
        // 1. 业务规则验证
        if !self.allow_registration {
            return Err(Error::Validation("Registration is disabled".to_string()));
        }
        self.validate_username(&username)?;
        self.validate_password(&password)?;

        // 2. 检查唯一性
        if self.repo.find_by_username(&username).await?.is_some() {
            return Err(Error::Validation("Username already exists".to_string()));
        }

        // 3. 业务逻辑（如：第一个用户是管理员）
        let is_first_user = self.repo.list_users(1).await?.is_empty();
        let permissions = if is_first_user {
            domain::ADMIN_PERMISSIONS
        } else {
            DEFAULT_USER_PERMISSIONS
        };

        // 4. 持久化
        self.repo.create_user(username, password, permissions).await
    }
}
```

### API层处理器模式

```rust
use axum::{extract::State, response::IntoResponse, Json};
use crate::{error::ApiError, state::AppState};

/// GET /users/{id}
async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    let user = state.user_service.get(id).await
        .map_err(ApiError::Domain)?;

    Ok(resp::ok(user))
}
```

### Infrastructure层Repository实现

```rust
use domain::UserRepository;
use sea_orm::DatabaseConnection;

pub struct UserRepositoryImpl {
    db: Arc<DatabaseConnection>,
}

impl UserRepositoryImpl {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>> {
        // SeaORM查询实现
        let result = users::Entity::find_by_id(id)
            .one(&*self.db)
            .await
            .map_err(|e| Error::Internal(e.to_string()))?;

        Ok(result.map(|entity| entity.into()))
    }
}
```

## 测试规范

### Service层测试（使用mockall）

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use mockall::*;
    use domain::User;

    mock! {
        UserRepo {}
        #[async_trait]
        impl UserRepository for UserRepo {
            async fn find_by_username(&self, username: &str) -> Result<Option<User>>;
            async fn create_user(&self, username: String, password: String, permissions: u64) -> Result<User>;
        }
    }

    #[tokio::test]
    async fn test_register_validates_username() {
        let mut mock = MockUserRepo::new();
        mock.expect_find_by_username()
            .returning(|_| Ok(None));

        let service = UserService::new(Arc::new(mock), true);

        let result = service.register("".to_string(), "password123".to_string()).await;

        assert!(matches!(result, Err(Error::Validation(_))));
    }
}
```

### Infrastructure层测试

```rust
#[tokio::test]
async fn test_user_repository_impl() {
    // 使用测试数据库
    let db = establish_test_connection().await;
    let repo = UserRepositoryImpl::new(db);

    // 测试CRUD操作
    let user = repo.create_user("test".to_string(), "pass".to_string(), 0).await;
    assert!(user.is_ok());
}
```

## 权限系统

```rust
// 位标志权限常量
pub const POST_CREATE: u64 = 1 << 0;   // 1
pub const POST_UPDATE: u64 = 1 << 1;   // 2
pub const POST_DELETE: u64 = 1 << 2;   // 4
pub const POST_PUBLISH: u64 = 1 << 3;  // 8
pub const USER_MANAGE: u64 = 1 << 4;   // 16

// 检查权限
domain::check_permission(user.permissions, POST_DELETE)?;

// 检查所有权或管理员
domain::check_ownership_or_admin(
    resource_owner_id,
    requester_id,
    requester_permissions,
    USER_MANAGE
)?;
```

## TypeScript规范

**导入顺序:**
```tsx
// 1. React导入
import { useState, useEffect } from 'react';

// 2. 第三方库
import { Button } from '@fluentui/react-components';

// 3. 本地模块
import { api } from '../api';
import type { Post } from '../types';

// 4. 样式
import './styles.css';
```

**命名:**
```tsx
const [isLoading, setIsLoading] = useState(false);  // camelCase - 变量
function PostList() {}                              // PascalCase - 组件
const MAX_POSTS = 10;                              // SCREAMING_SNAKE_CASE - 常量
interface PostData {}                              // PascalCase - 接口/类型
```

**错误处理:**
```tsx
try {
  const response = await api.getPost(id);
  setPost(response.data);
} catch (error) {
  console.error('Failed to fetch post:', error);
  // 显示用户友好的错误消息
}
```

## 重要规则

**DO:**
- ✅ Domain层保持零外部依赖（除了serde/chrono/uuid/async-trait）
- ✅ Service层定义Repository Trait
- ✅ Infrastructure层实现Repository
- ✅ 使用Domain层的Error类型
- ✅ 为所有新功能写测试
- ✅ 使用`#[async_trait]`为trait添加async支持
- ✅ 使用`Arc<dyn Trait>`进行依赖注入

**DON'T:**
- ❌ 绕过Repository直接操作数据库
- ❌ 在API层写业务逻辑
- ❌ 在Service层直接I/O操作
- ❌ 违反依赖方向（Domain不能依赖其他层）
- ❌ 硬编码配置（使用环境变量）
- ❌ 使用`Rc<RefCell<>>`（在async环境用Arc）
- ❌ 使用阻塞I/O（用tokio::task::spawn_blocking包装）

## Workspace配置

- **Edition:** 2021
- **Resolver:** 2
- **依赖管理:** workspace.dependencies统一管理版本
- **编译优化:** release启用lto和codegen-units=1

## 环境变量

```env
DATABASE_URL=postgresql://postgres:postgres@localhost:5432/peng_blog
HOST=0.0.0.0
PORT=3000
JWT_SECRET=change-this-in-production
UPLOAD_DIR=./uploads
BASE_URL=http://localhost:3000
RUST_LOG=debug
```

## 文档

- API文档: `docs/api/INDEX.md`
- 架构: 四层分层架构，Repository模式
- CLI: `cargo run -- help`

---

*Last updated: 2026-02-03*
