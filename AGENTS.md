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
│   ├── api/             # HTTP路由 - 51个端点
│   ├── service/         # 业务逻辑 - Repository Traits
│   ├── domain/          # 核心类型 - 零依赖
│   ├── infrastructure/  # 数据访问 - SeaORM实现
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
- Domain: 不依赖任何其他层
- Service: 定义接口，依赖Domain
- Infrastructure: 实现接口，依赖Domain
- API: 依赖Service+Domain

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
cargo test test_name           # 单个测试
cargo test test_name -- --exact  # 精确匹配
cargo test -- --nocapture      # 显示输出
cargo test -- --test-threads=1 # 单线程

# 检查
cargo fmt                      # 格式化
cargo clippy                   # Lint
cargo check                    # 快速类型检查
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

### Rust规范

**命名:**
```rust
struct PostService;            // PascalCase
enum Error { NotFound }        // PascalCase
fn create_post() {}            // snake_case
const MAX_SIZE: u64 = 100;     // SCREAMING_SNAKE_CASE
mod post_service;              // snake_case
```

**导入顺序:**
```rust
// 1. 标准库
use std::sync::Arc;

// 2. 第三方库
use async_trait::async_trait;
use uuid::Uuid;

// 3. 本地crate
use domain::{Error, Result};
use crate::models::Post;
```

**错误处理:**
```rust
use domain::{Error, Result};

// 验证错误
if input.is_empty() {
    return Err(Error::Validation("输入不能为空".to_string()));
}

// 传播错误
let user = self.repo.get_user(id).await?;

// 转换错误
self.repo.create(post).await
    .map_err(|e| Error::Internal(e.to_string()))?;
```

**Service层模式:**
```rust
pub struct PostService<R: PostRepository> {
    repo: Arc<R>,
}

impl<R: PostRepository> PostService<R> {
    pub fn new(repo: Arc<R>) -> Self {
        Self { repo }
    }

    pub async fn create(&self, request: CreatePost) -> Result<Post> {
        // 1. 验证
        self.validate(&request)?;

        // 2. 业务逻辑
        let post = self.build_post(request);

        // 3. 持久化
        self.repo.create(post).await
    }
}
```

### TypeScript规范

**组件结构:**
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
const [isLoading, setIsLoading] = useState(false);  // camelCase
function PostList() {}                              // PascalCase
const MAX_POSTS = 10;                              // SCREAMING_SNAKE_CASE
interface PostData {}                              // PascalCase
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

## 测试规范

**Service层测试 (mockall):**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use mockall::*;

    mock! {
        PostRepo {}
        #[async_trait]
        impl PostRepository for PostRepo {
            async fn create(&self, post: Post) -> Result<Post>;
        }
    }

    #[tokio::test]
    async fn test_create_post_validates() {
        let mock = MockPostRepo::new();
        let service = PostService::new(Arc::new(mock));

        let result = service.create(CreatePost {
            title: "".to_string(),
            content: "test".to_string(),
        }).await;

        assert!(matches!(result, Err(Error::Validation(_))));
    }
}
```

## 权限系统

```rust
// 位标志权限
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

## 重要规则

**DO:**
- ✅ Domain层保持零外部依赖（除了serde/chrono/uuid）
- ✅ Service层定义Repository Trait
- ✅ Infrastructure层实现Repository
- ✅ 使用Domain层的Error类型
- ✅ 为所有新功能写测试

**DON'T:**
- ❌ 绕过Repository直接操作数据库
- ❌ 在API层写业务逻辑
- ❌ 在Service层直接I/O操作
- ❌ 违反依赖方向（Domain不能依赖其他层）
- ❌ 硬编码配置（使用环境变量）

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

*Last updated: 2026-02-02*
