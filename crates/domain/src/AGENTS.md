# AGENTS.md - Domain Layer

> Domain层核心类型规范 - 零依赖业务实体层

**Generated:** 2026-02-04 10:00:33 PM
**Parent:** `../../AGENTS.md`

---

## OVERVIEW

**Domain Layer** - 核心业务实体和规则定义层，**零外部依赖**（仅允许serde/chrono/uuid/async-trait/thiserror）

---

## STRUCTURE

```
crates/domain/src/
├── post.rs          # Post, PostMetadata, PostStatus
├── user.rs          # User, UserRole
├── comment.rs       # Comment, CommentAuthor
├── session.rs       # Session
├── file.rs          # File, FileMetadata
├── stats.rs         # Stats, VisitStats
├── config.rs        # 纯类型定义
├── permission.rs    # 位标志权限常量 + 检查函数
└── error.rs         # Error枚举（Validation, NotFound, Internal）
```

---

## WHERE TO LOOK

| Task     | File            | Purpose                                      |
| -------- | --------------- | -------------------------------------------- |
| 权限系统 | `permission.rs` | POST_CREATE, POST_UPDATE, check_permission() |
| 错误类型 | `error.rs`      | Error enum, Result<T> alias                  |
| 文章实体 | `post.rs`       | Post结构体 + PostStatus枚举                  |

---

## CRITICAL VIOLATION

**🚨 Domain → Config dependency违规**

- **Location:** `Cargo.toml:14` + `config.rs:145-217`
- **Issue:** Domain层依赖config crate（违反零依赖原则）
- **Fix:** 移除config依赖，将`From<config::AppConfig>`转换逻辑移到Service层

---

## UNIQUE STYLES

- **UUID主键** - 所有实体使用`id: Uuid`（非自增ID）
- **Utc时间戳** - `DateTime<Utc>`统一时区
- **枚举优于布尔** - `PostStatus`代替`is_published: bool`
- **位标志权限** - `pub const POST_CREATE: u64 = 1 << 0`

---

_Last updated: 2026-02-04_
