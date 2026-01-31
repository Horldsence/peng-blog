# API 设计改进方案

## 现有设计存在的问题

### 1. HTTP 方法使用不一致

| 问题 | 当前设计 | 改进方案 |
|------|----------|----------|
| 发布/取消发布文章 | `POST /posts/{id}/publish` / `POST /posts/{id}/unpublish` | `PATCH /posts/{id}` with `{"status": "published"}` |
| 设置文章分类 | `PUT /posts/{id}/category` | `PATCH /posts/{id}` with `{"category_id": "xxx"}` |
| 修改用户权限 | `PATCH /users/{id}/permissions` | `PATCH /users/{id}` with `{"permissions": xxx}` |

### 2. URI 设计不一致

| 问题 | 当前设计 | 改进方案 |
|------|----------|----------|
| 标签关联操作 | `POST /posts/{id}/tags/{tag_id}` / `DELETE /posts/{id}/tags/{tag_id}` | `POST /posts/{id}/tags` with body / `DELETE /posts/{id}/tags/{tag_id}` |

### 3. 资源层级关系不清晰

| 问题 | 当前设计 | 改进方案 |
|------|----------|----------|
| 获取用户文章 | `GET /users/{id}/posts` 返回所有文章 | `GET /users/{id}/posts` 只返回已发布，`GET /users/{id}/posts?include=drafts` 包含草稿 |
| 获取文章评论 | 可能在 comments 模块 | `GET /posts/{id}/comments` 更直观 |

### 4. 响应格式不统一

| 问题 | 当前设计 | 改进方案 |
|------|----------|----------|
| 成功响应 | 有些返回 `{ "success": true }`，有些返回完整对象 | 统一返回操作后的资源或标准消息 |
| 列表响应 | 直接返回数组 | 包装为 `{ "data": [], "pagination": {} }` |

### 5. 分页参数不统一

| 问题 | 当前设计 | 改进方案 |
|------|----------|----------|
| 分页参数 | 有的用 `limit`，有的缺少 `offset` | 统一使用 `page` + `per_page` 或 `limit` + `offset` |

---

## 改进后的 API 设计

### 基础规范

1. **使用 RESTful 资源导向设计**
2. **HTTP 方法语义化**: GET(获取), POST(创建), PUT(全量更新), PATCH(部分更新), DELETE(删除)
3. **统一响应格式**: 所有响应包装在标准结构中
4. **一致的命名**: 使用 kebab-case 的 URI

### 统一响应格式

```json
// 成功响应 (单资源)
{
  "code": 200,
  "message": "success",
  "data": { ... }
}

// 成功响应 (列表)
{
  "code": 200,
  "message": "success",
  "data": [ ... ],
  "pagination": {
    "page": 1,
    "per_page": 20,
    "total": 100,
    "total_pages": 5
  }
}

// 创建成功
{
  "code": 201,
  "message": "created",
  "data": { ... }
}

// 删除成功 (204 No Content，无响应体)

// 错误响应
{
  "code": 400,
  "message": "Validation failed",
  "errors": {
    "field": ["error message"]
  }
}
```

---

## 重构后的端点列表

### 认证 (Auth)

| 方法 | 端点 | 描述 |
|------|------|------|
| POST | `/auth/register` | 注册用户 |
| POST | `/auth/login` | 登录 |
| POST | `/auth/logout` | 登出 |
| GET | `/auth/me` | 获取当前用户信息 |

### 用户 (Users)

| 方法 | 端点 | 描述 |
|------|------|------|
| GET | `/users` | 列出用户（管理员） |
| GET | `/users/{id}` | 获取用户信息 |
| PATCH | `/users/{id}` | 更新用户信息（包括权限） |
| DELETE | `/users/{id}` | 删除用户 |
| GET | `/users/{id}/posts` | 获取用户的已发布文章 |
| GET | `/users/{id}/posts?include=drafts` | 获取用户的所有文章（自己/管理员） |

### 文章 (Posts)

| 方法 | 端点 | 描述 |
|------|------|------|
| GET | `/posts` | 列出已发布文章 |
| GET | `/posts?author={user_id}` | 按作者筛选 |
| GET | `/posts?category={category_id}` | 按分类筛选 |
| GET | `/posts?tag={tag_id}` | 按标签筛选 |
| GET | `/posts?status=draft` | 按状态筛选（自己/管理员） |
| GET | `/posts/search?q={query}` | 搜索文章 |
| POST | `/posts` | 创建文章 |
| GET | `/posts/{id}` | 获取文章详情 |
| PUT | `/posts/{id}` | 全量更新文章 |
| PATCH | `/posts/{id}` | 部分更新文章（标题、内容、分类、状态） |
| DELETE | `/posts/{id}` | 删除文章 |
| GET | `/posts/{id}/comments` | 获取文章评论 |
| POST | `/posts/{id}/comments` | 添加评论 |
| GET | `/posts/{id}/tags` | 获取文章标签 |
| POST | `/posts/{id}/tags` | 添加标签到文章 |
| DELETE | `/posts/{id}/tags/{tag_id}` | 从文章移除标签 |

### 分类 (Categories)

| 方法 | 端点 | 描述 |
|------|------|------|
| GET | `/categories` | 列出所有分类 |
| POST | `/categories` | 创建分类（管理员） |
| GET | `/categories/{id}` | 获取分类详情 |
| GET | `/categories/{id}/posts` | 获取分类下的文章 |
| PATCH | `/categories/{id}` | 更新分类（管理员） |
| DELETE | `/categories/{id}` | 删除分类（管理员） |

### 标签 (Tags)

| 方法 | 端点 | 描述 |
|------|------|------|
| GET | `/tags` | 列出所有标签 |
| POST | `/tags` | 创建标签（管理员） |
| GET | `/tags/{id}` | 获取标签详情 |
| GET | `/tags/{id}/posts` | 获取标签下的文章 |
| DELETE | `/tags/{id}` | 删除标签（管理员） |

### 评论 (Comments)

| 方法 | 端点 | 描述 |
|------|------|------|
| GET | `/comments/{id}` | 获取评论详情 |
| PATCH | `/comments/{id}` | 更新评论 |
| DELETE | `/comments/{id}` | 删除评论 |

### 文件 (Files)

| 方法 | 端点 | 描述 |
|------|------|------|
| GET | `/files` | 列出文件（自己的/管理员全部） |
| POST | `/files` | 上传文件 |
| GET | `/files/{id}` | 获取文件信息 |
| GET | `/files/{id}/content` | 下载文件内容 |
| DELETE | `/files/{id}` | 删除文件 |

### 统计 (Stats)

| 方法 | 端点 | 描述 |
|------|------|------|
| GET | `/stats` | 获取整体统计 |
| GET | `/stats/visits` | 访问统计 |
| POST | `/stats/visits` | 记录访问 |
| GET | `/stats/posts/{id}` | 文章统计 |
| POST | `/stats/posts/{id}/views` | 记录文章浏览 |

---

## 关键改进点

### 1. 状态管理改进

**之前:**
```bash
POST /posts/{id}/publish
POST /posts/{id}/unpublish
```

**之后:**
```bash
PATCH /posts/{id}
{
  "status": "published"  // 或 "draft"
}
```

### 2. 标签管理改进

**之前:**
```bash
POST /posts/{id}/tags/{tag_id}
```

**之后:**
```bash
POST /posts/{id}/tags
{
  "tag_id": "xxx"
}
```

### 3. 列表响应改进

**之前:**
```json
[
  { "id": "...", "title": "..." },
  { "id": "...", "title": "..." }
]
```

**之后:**
```json
{
  "code": 200,
  "message": "success",
  "data": [
    { "id": "...", "title": "..." },
    { "id": "...", "title": "..." }
  ],
  "pagination": {
    "page": 1,
    "per_page": 20,
    "total": 100
  }
}
```

### 4. 筛选参数改进

**之前:**
```bash
GET /posts?user_id=xxx&category_id=yyy&tag_id=zzz
```

**之后:**
```bash
GET /posts?author=xxx&category=yyy&tag=zzz&status=published
```

---

## 实现计划

1. **创建响应包装器**: 统一的 API 响应格式
2. **重构文章 API**: 使用 PATCH 替代 publish/unpublish 操作
3. **重构标签关联 API**: 使用请求体而非路径参数
4. **重构列表响应**: 添加分页包装
5. **更新文档**: 同步更新 API 文档
