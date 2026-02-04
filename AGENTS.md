# AGENTS.md - Peng Blog

> AI编码代理工作指南 - 项目架构、构建命令和代码规范

**Generated:** 2026-02-04 10:00:33 PM
**Commit:** ce7dc03 (2026-02-04 15:57:57 +0800)
**Branch:** main

---

## OVERVIEW

**Peng Blog** - Rust + React博客系统，采用严格的四层分层架构（Clean Architecture模式）

**技术栈:**
- 后端: Tokio + Axum + SeaORM + PostgreSQL
- 前端: React 18 + TypeScript + Vite + FluentUI
- 安全: JWT + Argon2，位标志权限系统

**架构特征:**
- 7个Rust crates (workspace管理)
- 单二进制部署（前端通过rust_embed嵌入）
- Repository模式（Service定义Trait，Infrastructure实现）
- 依赖注入（App层组装所有依赖）

---

## STRUCTURE

```
peng-blog/
├── crates/
│   ├── app/             # 应用入口 - 依赖注入容器
│   ├── api/             # HTTP路由 - 处理器（14文件）
│   ├── service/         # 业务逻辑 - Repository Traits定义
│   ├── domain/          # 核心类型 - 零依赖层（12文件）
│   ├── infrastructure/  # 数据访问 - SeaORM实现（含entity/migrations）
│   ├── config/          # 配置管理
│   └── cli/             # CLI工具（用户/数据库管理）
├── frontend/            # React前端
│   └── src/
│       ├── api/         # API客户端（12文件）
│       ├── pages/       # 页面组件（10文件）
│       └── components/  # UI组件
├── docs/api/            # API文档
└── scripts/             # 构建和CI脚本
```

**架构依赖规则（CRITICAL - 违反会破坏架构）:**
```
App → API → Service → Domain
              ↓
        Infrastructure → Domain
```

**依赖方向原则:**
- ✅ Domain: 不依赖任何其他层（仅允许 serde/chrono/uuid/async-trait）
- ✅ Service: 定义Repository Trait，依赖Domain
- ✅ Infrastructure: 实现Repository，依赖Domain
- ✅ API: 依赖Service+Domain，**禁止**直接访问Infrastructure
- ✅ App: 依赖所有层，负责组装

---

## WHERE TO LOOK

| Task | Location | Notes |
|------|----------|-------|
| 定义业务实体 | `crates/domain/src/*.rs` | Post, User, Comment等核心类型 |
| 定义Repository接口 | `crates/service/src/*.rs` | UserService, PostService等Trait |
| 实现Repository | `crates/infrastructure/src/*.rs` | SeaORM实现 |
| HTTP路由 | `crates/api/src/*.rs` | 各模块的handler函数 |
| 前端API调用 | `frontend/src/api/*.ts` | Axios客户端封装 |
| 数据库迁移 | `crates/infrastructure/src/migrations/` | 13个迁移文件 |
| 数据库实体 | `crates/infrastructure/src/entity/` | 11个SeaORM实体 |
| 依赖注入 | `crates/app/src/lib.rs` | `run_server()`组装所有依赖 |
| 前端构建集成 | `crates/app/build.rs` | npm run build + rust_embed |
| CLI命令 | `crates/cli/src/main.rs` | user/db管理命令 |

---

## ANTI-PATTERNS (CRITICAL VIOLATIONS DETECTED)

### 🚨 Current Architectural Violations

**1. Domain → Config Dependency (CRITICAL)**
- **Location:** `crates/domain/Cargo.toml:14`
- **Issue:** Domain层依赖config crate（违反零依赖原则）
- **Fix Required:** 移除`config = { path = "../config" }`，将`From<config::AppConfig>`转换逻辑移到Service或API层

**2. API → Infrastructure Dependency (MEDIUM)**
- **Location:** `crates/api/Cargo.toml:11`
- **Issue:** API层直接依赖Infrastructure（应通过Service）
- **Current:** 仅在doc comments使用，实际代码未依赖
- **Fix Required:** 移除依赖声明，更新doc注释

### ⚠️ Deprecated Frontend Types

**frontend/src/types/index.ts (Lines 44-58):**
- `ApiResponse<T>` - 迁移到 `ApiResponseV2<T>`
- `PaginatedResponse<T>` - 迁移到 `ApiListResponseV2<T>`
- `ApiError` - 迁移到 `ApiErrorV2`

### 📋 Known Technical Debt

**crates/service/src/stats/mod.rs:54**
- `let is_today = true;` - 简化实现，始终假设今天
- **Impact:** 日期统计功能不准确

---

## CONVENTIONS

### Rust Backend

**导入顺序（CRITICAL - 必须遵守）:**
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

**Repository Trait定义（Service层）:**
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

**Service层模式:**
```rust
pub struct UserService {
    repo: Arc<dyn UserRepository>,  // 使用Trait对象
    allow_registration: bool,
}

impl UserService {
    pub async fn register(&self, username: String, password: String) -> Result<User> {
        // 1. 业务规则验证
        self.validate_username(&username)?;
        self.validate_password(&password)?;

        // 2. 检查唯一性
        if self.repo.find_by_username(&username).await?.is_some() {
            return Err(Error::Validation("Username already exists".to_string()));
        }

        // 3. 业务逻辑
        let permissions = self.determine_permissions()?;

        // 4. 持久化（通过Repository）
        self.repo.create_user(username, password, permissions).await
    }
}
```

**API层处理器模式:**
```rust
use axum::{extract::State, response::IntoResponse, Json};

async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    let user = state.user_service.get(id).await
        .map_err(ApiError::Domain)?;  // Domain错误自动转换

    Ok(resp::ok(user))
}
```

**错误处理模式:**
```rust
// Domain层
if input.is_empty() {
    return Err(Error::Validation("输入不能为空".to_string()));
}

// 传播错误（使用?操作符）
let user = self.repo.get_user(id).await?;

// 转换错误类型
self.repo.create(post).await
    .map_err(|e| Error::Internal(e.to_string()))?;
```

### TypeScript Frontend

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

---

## UNIQUE STYLES (Project-Specific)

### Frontend Build Integration

**Dual-Mode Frontend Serving:**
- **Release模式:** Vite构建的静态资源通过`rust_embed`嵌入二进制
- **Debug模式:** 从文件系统serving（热重载）
- **实现位置:** `crates/app/build.rs` + `crates/app/src/lib.rs` (fallback handler)

### Bit-Flag Permissions

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

### First-User-Is-Admin Pattern

**Service层逻辑:**
```rust
let is_first_user = self.repo.list_users(1).await?.is_empty();
let permissions = if is_first_user {
    domain::ADMIN_PERMISSIONS  // 第一个用户自动成为管理员
} else {
    DEFAULT_USER_PERMISSIONS
};
```

---

## COMMANDS

### Backend (Rust)

```bash
# 构建
cargo build                    # 开发构建
cargo build --release          # 生产构建（启用LTO + 单codegen unit）

# 运行
cargo run                      # 启动服务器
cargo run --release            # 生产模式

# 测试
cargo test                     # 所有测试
cargo test -p service          # 单个crate测试
cargo test test_name -- --exact  # 精确匹配测试名

# 快速检查
cargo check                    # 类型检查（不构建）
cargo clippy                   # Lint检查
cargo fmt                      # 格式化代码
```

### Frontend (TypeScript)

```bash
cd frontend

# 开发
npm run dev                    # Vite watch模式（输出到../dist）

# 构建
npm run build                  # TypeScript检查 + Vite构建
npm run type-check             # 仅TypeScript检查
npm run lint                   # ESLint检查
npm run format                 # Prettier格式化
```

### CLI Tools

```bash
# 用户管理
cargo run -- user list
cargo run -- user create --username admin --password pass --admin
cargo run -- user reset-password <user_id>
cargo run -- user promote <user_id>

# 数据库管理
cargo run -- db migrate
cargo run -- db reset --force  # 警告：删除所有数据
```

### Makefile Targets

```bash
make help          # 显示所有可用命令
make dev           # 启动开发环境
make build         # 完整构建（前端+后端）
make test          # 运行所有测试
make ci            # 运行CI检查
make fmt           # 格式化代码
```

---

## NOTES

### Workspace Configuration

- **Edition:** Rust 2021
- **Resolver:** Version 2
- **Members:** 7 crates (统一版本管理)
- **依赖管理:** workspace.dependencies统一管理版本
- **编译优化:** release启用lto和codegen-units=1

### Build Profiles

```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true

[profile.dev]
opt-level = 0  # 更快的编译速度
```

### Environment Variables

```env
DATABASE_URL=postgresql://postgres:postgres@localhost:5432/peng_blog
HOST=0.0.0.0
PORT=3000
JWT_SECRET=change-this-in-production
UPLOAD_DIR=./uploads
BASE_URL=http://localhost:3000
RUST_LOG=debug
GITHUB_CLIENT_ID=
GITHUB_CLIENT_SECRET=
```

### Testing Patterns

- **Service层:** 使用mockall进行mock测试
- **Infrastructure层:** 使用测试数据库进行集成测试
- **测试配置:** timeout=0 (禁用超时), threads=0 (自动检测)

### Important Reminders

1. **Domain层禁止添加外部依赖**（仅允许serde/chrono/uuid/async-trait）
2. **所有新功能必须写测试**
3. **使用Arc<dyn Trait>进行依赖注入**（避免泛型爆炸）
4. **前端构建输出到../dist**（通过Vite配置）
5. **Release模式包含前端静态资源**（通过rust_embed）
6. **位标志权限系统高效但需注意常量定义**

---

## SUBDIRECTORIES

Hierarchical AGENTS.md files for detailed domain knowledge:
- `crates/domain/src/AGENTS.md` - Domain层核心类型规范
- `frontend/src/api/AGENTS.md` - 前端API客户端模式

---

*Last updated: 2026-02-04*
*Total files: 195 (82 Rust + 43 TypeScript + 70 others)*
*Lines of code: ~19,673*
