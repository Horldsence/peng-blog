# Peng Blog API Documentation

完整的 RESTful API 参考文档。

## 目录

- [认证机制](#认证机制)
- [通用规范](#通用规范)
- [错误处理](#错误处理)
- [API 端点](#api-端点)
  - [认证 (Auth)](#认证-auth)
  - [文章 (Posts)](#文章-posts)
  - [用户 (Users)](#用户-users)
  - [会话 (Sessions)](#会话-sessions)
  - [文件 (Files)](#文件-files)
  - [评论 (Comments)](#评论-comments)
  - [统计 (Stats)](#统计-stats)

## 认证机制

Peng Blog 支持两种认证方式：

### 1. JWT Token 认证（推荐）

在请求头中包含 JWT token：

```http
Authorization: Bearer <your-jwt-token>
```

获取方式：登录接口返回 `token` 字段。

**特点：**

- 适用于 RESTful API 调用
- 支持移动端和前端应用
- Token 有效期由服务器配置

### 2. Cookie 会话认证

自动包含在浏览器请求中，无需手动设置。

**特点：**

- 适用于浏览器原生应用
- 支持"记住我"功能（30天 vs 24小时）
- 自动过期管理

## 通用规范

### 基础 URL

```
http://localhost:3000/api
```

### 请求格式

- Content-Type: `application/json`
- 字符编码: UTF-8

### 响应格式

所有响应都遵循统一格式：

**成功响应：**

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "title": "文章标题",
  ...
}
```

**批量响应：**

```json
{
  "data": [...],
  "total": 100,
  "page": 1,
  "page_size": 10
}
```

**操作确认响应：**

```json
{
  "message": "操作成功"
}
```

### 分页

使用查询参数进行分页：

```
GET /api/posts?page=1&page_size=10
```

参数：

- `page`: 页码（从1开始）
- `page_size`: 每页数量（默认10）

### ID 格式

所有资源 ID 使用 UUID v4 格式：

```
550e8400-e29b-41d4-a716-446655440000
```

## 错误处理

### HTTP 状态码

| 状态码 | 说明           |
| ------ | -------------- |
| 200    | 请求成功       |
| 201    | 资源创建成功   |
| 400    | 请求参数错误   |
| 401    | 未认证         |
| 403    | 权限不足       |
| 404    | 资源不存在     |
| 500    | 服务器内部错误 |

### 错误响应格式

```json
{
  "error": "错误类型",
  "message": "详细错误信息",
  "details": {
    "field": "具体字段错误"
  }
}
```

### 常见错误

**400 Bad Request**

```json
{
  "error": "Validation",
  "message": "验证失败：用户名长度必须在3-50个字符之间"
}
```

**401 Unauthorized**

```json
{
  "error": "Unauthorized",
  "message": "缺少认证信息或 token 无效"
}
```

**403 Forbidden**

```json
{
  "error": "Forbidden",
  "message": "权限不足：您不是资源的所有者"
}
```

**404 Not Found**

```json
{
  "error": "NotFound",
  "message": "文章不存在"
}
```

## API 端点

### 认证 (Auth)

#### 用户注册

创建新用户账户。

**端点：**

```
POST /auth/register
```

**请求体：**

```json
{
  "username": "newuser",
  "password": "SecurePassword123!"
}
```

**字段说明：**

- `username`: 用户名（3-50字符，字母数字）
- `password`: 密码（最少8字符）

**响应 (201)：**

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "username": "newuser",
  "permissions": 15,
  "created_at": "2026-01-29T10:00:00Z"
}
```

#### 用户登录

使用用户名和密码登录，返回 JWT token。

**端点：**

```
POST /auth/login
```

**请求体：**

```json
{
  "username": "testuser",
  "password": "password123"
}
```

**响应 (200)：**

```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM...",
  "user": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "username": "testuser",
    "permissions": 15
  }
}
```

#### 用户登出

使当前 token 失效（如支持 token 黑名单）。

**端点：**

```
POST /auth/logout
```

**认证：** 需要

**响应 (200)：**

```json
{
  "message": "登出成功"
}
```

---

### 文章 (Posts)

#### 获取文章列表

获取所有文章，支持分页和过滤。

**端点：**

```
GET /posts
```

**查询参数：**

- `page`: 页码（默认1）
- `page_size`: 每页数量（默认10）
- `user_id`: 按用户过滤（可选）

**示例：**

```
GET /posts?page=1&page_size=10
GET /posts?user_id=550e8400-e29b-41d4-a716-446655440000
```

**响应 (200)：**

```json
{
  "data": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "user_id": "660e8400-e29b-41d4-a716-446655440001",
      "title": "我的第一篇文章",
      "content": "文章内容...",
      "published": true,
      "views": 100,
      "created_at": "2026-01-29T10:00:00Z",
      "updated_at": "2026-01-29T11:00:00Z",
      "published_at": "2026-01-29T10:30:00Z"
    }
  ],
  "total": 100,
  "page": 1,
  "page_size": 10
}
```

#### 获取单篇文章

获取指定 ID 的文章详情。

**端点：**

```
GET /posts/:id
```

**路径参数：**

- `id`: 文章 UUID

**响应 (200)：**

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "user_id": "660e8400-e29b-41d4-a716-446655440001",
  "title": "文章标题",
  "content": "文章完整内容...",
  "published": true,
  "views": 150,
  "created_at": "2026-01-29T10:00:00Z",
  "updated_at": "2026-01-29T11:00:00Z",
  "published_at": "2026-01-29T10:30:00Z"
}
```

#### 创建文章

创建新文章。

**端点：**

```
POST /posts
```

**认证：** 需要

**权限：** `POST_CREATE`

**请求体：**

```json
{
  "title": "新文章标题",
  "content": "文章内容...",
  "published": false
}
```

**字段说明：**

- `title`: 文章标题（必填）
- `content`: 文章内容（必填）
- `published`: 是否立即发布（默认false）

**响应 (201)：**

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "user_id": "660e8400-e29b-41d4-a716-446655440001",
  "title": "新文章标题",
  "content": "文章内容...",
  "published": false,
  "views": 0,
  "created_at": "2026-01-29T12:00:00Z",
  "updated_at": "2026-01-29T12:00:00Z",
  "published_at": null
}
```

#### 更新文章

更新现有文章。

**端点：**

```
PUT /posts/:id
```

**认证：** 需要

**权限：** 文章所有者或 `POST_UPDATE`

**路径参数：**

- `id`: 文章 UUID

**请求体：**

```json
{
  "title": "更新后的标题",
  "content": "更新后的内容...",
  "published": true
}
```

**响应 (200)：**

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "user_id": "660e8400-e29b-41d4-a716-446655440001",
  "title": "更新后的标题",
  "content": "更新后的内容...",
  "published": true,
  "views": 100,
  "created_at": "2026-01-29T10:00:00Z",
  "updated_at": "2026-01-29T13:00:00Z",
  "published_at": "2026-01-29T13:00:00Z"
}
```

#### 删除文章

删除指定文章。

**端点：**

```
DELETE /posts/:id
```

**认证：** 需要

**权限：** 文章所有者或 `POST_DELETE`

**路径参数：**

- `id`: 文章 UUID

**响应 (200)：**

```json
{
  "message": "文章删除成功"
}
```

---

### 用户 (Users)

#### 获取当前用户信息

获取当前认证用户的详细信息。

**端点：**

```
GET /users/me
```

**认证：** 需要

**响应 (200)：**

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "username": "testuser",
  "permissions": 15,
  "created_at": "2026-01-29T10:00:00Z"
}
```

#### 获取用户列表

获取所有用户列表（管理员功能）。

**端点：**

```
GET /users
```

**认证：** 需要

**权限：** `USER_MANAGE`

**查询参数：**

- `page`: 页码
- `page_size`: 每页数量

**响应 (200)：**

```json
{
  "data": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "username": "testuser",
      "permissions": 15,
      "created_at": "2026-01-29T10:00:00Z"
    }
  ],
  "total": 10,
  "page": 1,
  "page_size": 10
}
```

#### 获取指定用户

获取指定 ID 的用户信息。

**端点：**

```
GET /users/:id
```

**路径参数：**

- `id`: 用户 UUID

**响应 (200)：**

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "username": "testuser",
  "permissions": 15,
  "created_at": "2026-01-29T10:00:00Z"
}
```

#### 删除用户

删除指定用户（管理员功能）。

**端点：**

```
DELETE /users/:id
```

**认证：** 需要

**权限：** `USER_MANAGE`

**路径参数：**

- `id`: 用户 UUID

**响应 (200)：**

```json
{
  "message": "用户删除成功"
}
```

---

### 会话 (Sessions)

#### 创建会话

创建新会话（登录）。

**端点：**

```
POST /sessions
```

**请求体：**

```json
{
  "username": "testuser",
  "password": "password123",
  "remember_me": false
}
```

**字段说明：**

- `username`: 用户名
- `password`: 密码
- `remember_me`: 是否记住我（30天 vs 24小时）

**响应 (201)：**

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "user_id": "660e8400-e29b-41d4-a716-446655440001",
  "expires_at": "2026-01-30T10:00:00Z",
  "created_at": "2026-01-29T10:00:00Z"
}
```

#### 删除会话

删除当前会话（登出）。

**端点：**

```
DELETE /sessions
```

**认证：** 需要（Cookie）

**响应 (200)：**

```json
{
  "message": "会话已删除"
}
```

#### 获取当前会话

获取当前用户的会话信息。

**端点：**

```
GET /sessions/me
```

**认证：** 需要（Cookie）

**响应 (200)：**

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "user_id": "660e8400-e29b-41d4-a716-446655440001",
  "expires_at": "2026-01-30T10:00:00Z",
  "created_at": "2026-01-29T10:00:00Z"
}
```

#### GitHub OAuth 回调

处理 GitHub OAuth 认证回调。

**端点：**

```
POST /sessions/github
```

**请求体：**

```json
{
  "code": "github-authorization-code"
}
```

**响应 (201)：**

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "user_id": "660e8400-e29b-41d4-a716-446655440001",
  "expires_at": "2026-01-30T10:00:00Z",
  "created_at": "2026-01-29T10:00:00Z"
}
```

---

### 文件 (Files)

#### 上传文件

上传新文件。

**端点：**

```
POST /files
```

**认证：** 需要

**请求格式：** `multipart/form-data`

**请求体：**

```
file: <binary>
```

**限制：**

- 最大文件大小：10MB
- 允许类型：image/jpeg, image/png, image/gif, image/webp, application/pdf, text/plain, text/markdown

**响应 (201)：**

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "user_id": "660e8400-e29b-41d4-a716-446655440001",
  "filename": "a1b2c3d4-e5f6-7890-abcd-ef1234567890.jpg",
  "original_filename": "photo.jpg",
  "content_type": "image/jpeg",
  "size_bytes": 1024000,
  "url": "http://localhost:3000/files/a1b2c3d4-e5f6-7890-abcd-ef1234567890.jpg",
  "created_at": "2026-01-29T10:00:00Z"
}
```

#### 获取文件信息

获取指定文件的元数据。

**端点：**

```
GET /files/:id
```

**路径参数：**

- `id`: 文件 UUID

**响应 (200)：**

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "user_id": "660e8400-e29b-41d4-a716-446655440001",
  "filename": "a1b2c3d4-e5f6-7890-abcd-ef1234567890.jpg",
  "original_filename": "photo.jpg",
  "content_type": "image/jpeg",
  "size_bytes": 1024000,
  "url": "http://localhost:3000/files/a1b2c3d4-e5f6-7890-abcd-ef1234567890.jpg",
  "created_at": "2026-01-29T10:00:00Z"
}
```

#### 下载文件

下载指定文件。

**端点：**

```
GET /files/:id/download
```

**路径参数：**

- `id`: 文件 UUID

**响应 (200)：** 文件二进制流

**Headers：**

```
Content-Type: image/jpeg
Content-Disposition: attachment; filename="photo.jpg"
Content-Length: 1024000
```

#### 获取用户文件列表

获取当前用户上传的所有文件。

**端点：**

```
GET /files
```

**认证：** 需要

**查询参数：**

- `page`: 页码
- `page_size`: 每页数量

**响应 (200)：**

```json
{
  "data": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "user_id": "660e8400-e29b-41d4-a716-446655440001",
      "filename": "a1b2c3d4-e5f6-7890-abcd-ef1234567890.jpg",
      "original_filename": "photo.jpg",
      "content_type": "image/jpeg",
      "size_bytes": 1024000,
      "url": "http://localhost:3000/files/a1b2c3d4-e5f6-7890-abcd-ef1234567890.jpg",
      "created_at": "2026-01-29T10:00:00Z"
    }
  ],
  "total": 20,
  "page": 1,
  "page_size": 10
}
```

#### 删除文件

删除指定文件。

**端点：**

```
DELETE /files/:id
```

**认证：** 需要

**权限：** 文件所有者

**路径参数：**

- `id`: 文件 UUID

**响应 (200)：**

```json
{
  "message": "文件删除成功"
}
```

---

### 评论 (Comments)

#### 获取文章评论列表

获取指定文章的所有评论。

**端点：**

```
GET /posts/:post_id/comments
```

**路径参数：**

- `post_id`: 文章 UUID

**查询参数：**

- `page`: 页码
- `page_size`: 每页数量

**响应 (200)：**

```json
{
  "data": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "post_id": "660e8400-e29b-41d4-a716-446655440001",
      "user_id": "770e8400-e29b-41d4-a716-446655440002",
      "github_username": null,
      "github_avatar_url": null,
      "content": "这是一条评论",
      "created_at": "2026-01-29T10:00:00Z",
      "updated_at": "2026-01-29T10:00:00Z"
    }
  ],
  "total": 50,
  "page": 1,
  "page_size": 10
}
```

#### 创建评论（注册用户）

使用注册用户身份创建评论。

**端点：**

```
POST /comments
```

**认证：** 需要

**请求体：**

```json
{
  "post_id": "660e8400-e29b-41d4-a716-446655440001",
  "content": "这是一条评论"
}
```

**字段说明：**

- `post_id`: 文章 UUID
- `content`: 评论内容（必填）

**响应 (201)：**

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "post_id": "660e8400-e29b-41d4-a716-446655440001",
  "user_id": "770e8400-e29b-41d4-a716-446655440002",
  "github_username": null,
  "github_avatar_url": null,
  "content": "这是一条评论",
  "created_at": "2026-01-29T10:00:00Z",
  "updated_at": "2026-01-29T10:00:00Z"
}
```

#### 创建评论（GitHub 用户）

使用 GitHub OAuth 认证创建评论。

**端点：**

```
POST /comments/github
```

**请求体：**

```json
{
  "post_id": "660e8400-e29b-41d4-a716-446655440001",
  "content": "这是一条 GitHub 评论",
  "access_token": "github-access-token"
}
```

**响应 (201)：**

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "post_id": "660e8400-e29b-41d4-a716-446655440001",
  "user_id": null,
  "github_username": "githubuser",
  "github_avatar_url": "https://avatars.githubusercontent.com/u/123456",
  "content": "这是一条 GitHub 评论",
  "created_at": "2026-01-29T10:00:00Z",
  "updated_at": "2026-01-29T10:00:00Z"
}
```

#### 获取 GitHub 授权 URL

获取 GitHub OAuth 授权 URL。

**端点：**

```
GET /comments/github/auth
```

**响应 (200)：**

```json
{
  "auth_url": "https://github.com/login/oauth/authorize?client_id=xxx&scope=user:email&state=xxx"
}
```

#### 获取单条评论

获取指定评论的详细信息。

**端点：**

```
GET /comments/:id
```

**路径参数：**

- `id`: 评论 UUID

**响应 (200)：**

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "post_id": "660e8400-e29b-41d4-a716-446655440001",
  "user_id": "770e8400-e29b-41d4-a716-446655440002",
  "github_username": null,
  "github_avatar_url": null,
  "content": "这是一条评论",
  "created_at": "2026-01-29T10:00:00Z",
  "updated_at": "2026-01-29T10:00:00Z"
}
```

#### 更新评论

更新指定评论。

**端点：**

```
PUT /comments/:id
```

**认证：** 需要

**权限：** 评论作者

**路径参数：**

- `id`: 评论 UUID

**请求体：**

```json
{
  "content": "更新后的评论内容"
}
```

**响应 (200)：**

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "post_id": "660e8400-e29b-41d4-a716-446655440001",
  "user_id": "770e8400-e29b-41d4-a716-446655440002",
  "github_username": null,
  "github_avatar_url": null,
  "content": "更新后的评论内容",
  "created_at": "2026-01-29T10:00:00Z",
  "updated_at": "2026-01-29T11:00:00Z"
}
```

#### 删除评论

删除指定评论。

**端点：**

```
DELETE /comments/:id
```

**认证：** 需要

**权限：** 评论作者

**路径参数：**

- `id`: 评论 UUID

**响应 (200)：**

```json
{
  "message": "评论删除成功"
}
```

---

### 统计 (Stats)

#### 获取全局访问统计

获取网站的访问统计信息。

**端点：**

```
GET /stats/visits
```

**响应 (200)：**

```json
{
  "total_visits": 10000,
  "today_visits": 150,
  "last_updated": "2026-01-29T10:00:00Z"
}
```

#### 记录访问

记录一次网站访问。

**端点：**

```
POST /stats/visits
```

**请求体：**

```json
{
  "post_id": "550e8400-e29b-41d4-a716-446655440000"
}
```

**字段说明：**

- `post_id`: 可选，如果提供则同时记录文章阅读量

**响应 (200)：**

```json
{
  "message": "访问已记录"
}
```

#### 获取文章阅读量

获取指定文章的阅读量统计。

**端点：**

```
GET /stats/posts/:id/views
```

**路径参数：**

- `id`: 文章 UUID

**响应 (200)：**

```json
{
  "post_id": "550e8400-e29b-41d4-a716-446655440000",
  "views": 500,
  "last_viewed_at": "2026-01-29T10:00:00Z"
}
```

#### 记录文章阅读

记录指定文章被阅读一次。

**端点：**

```
POST /stats/posts/:id/views
```

**路径参数：**

- `id`: 文章 UUID

**响应 (200)：**

```json
{
  "message": "阅读已记录"
}
```

#### 获取总统计（管理员）

获取完整的统计信息（管理员功能）。

**端点：**

```
GET /stats/total
```

**认证：** 需要

**权限：** 管理员

**响应 (200)：**

```json
{
  "total_posts": 100,
  "total_users": 50,
  "total_comments": 500,
  "total_files": 200,
  "total_visits": 10000,
  "today_visits": 150
}
```

---

## 权限系统

### 权限位标志

Peng Blog 使用位标志实现高效的权限控制：

| 权限           | 值  | 说明     |
| -------------- | --- | -------- |
| `POST_CREATE`  | 1   | 创建文章 |
| `POST_UPDATE`  | 2   | 更新文章 |
| `POST_DELETE`  | 4   | 删除文章 |
| `POST_PUBLISH` | 8   | 发布文章 |
| `USER_MANAGE`  | 16  | 管理用户 |

### 默认权限

**普通用户：**

```
POST_CREATE | POST_UPDATE | POST_PUBLISH = 1 | 2 | 8 = 11
```

**管理员：**

```
POST_CREATE | POST_UPDATE | POST_DELETE | POST_PUBLISH | USER_MANAGE = 31
```

### 权限检查示例

**检查是否是资源所有者：**

```javascript
if (resource.user_id === current_user.id) {
  // 可以操作
}
```

**检查管理员权限：**

```javascript
if (current_user.permissions & USER_MANAGE) {
  // 可以管理
}
```

---

## 速率限制

当前版本未实现速率限制，建议在生产环境配置反向代理（如 Nginx）进行限流。

## CORS 配置

开发环境使用 `permissive` CORS 策略，允许所有来源。

生产环境建议配置允许的来源列表。

## WebSocket 支持

当前版本不支持 WebSocket，未来版本将添加实时评论推送功能。

---

## 附录

### 示例代码

#### 使用 cURL

```bash
# 用户注册
curl -X POST http://localhost:3000/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{"username":"testuser","password":"password123"}'

# 用户登录
curl -X POST http://localhost:3000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"testuser","password":"password123"}'

# 创建文章（使用 JWT）
TOKEN="your-jwt-token"
curl -X POST http://localhost:3000/api/posts \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"title":"新文章","content":"文章内容","published":false}'

# 获取文章列表
curl http://localhost:3000/api/posts
```

#### 使用 JavaScript (fetch)

```javascript
// 用户登录
const login = async () => {
  const response = await fetch("http://localhost:3000/api/auth/login", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      username: "testuser",
      password: "password123",
    }),
  });

  const data = await response.json();
  return data.token; // 保存 token
};

// 创建文章
const createPost = async (token) => {
  const response = await fetch("http://localhost:3000/api/posts", {
    method: "POST",
    headers: {
      Authorization: `Bearer ${token}`,
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      title: "新文章",
      content: "文章内容",
      published: false,
    }),
  });

  return await response.json();
};
```

#### 使用 Python (requests)

```python
import requests

BASE_URL = 'http://localhost:3000/api'

# 用户登录
def login():
    response = requests.post(
        f'{BASE_URL}/auth/login',
        json={
            'username': 'testuser',
            'password': 'password123'
        }
    )
    return response.json()['token']

# 创建文章
def create_post(token):
    response = requests.post(
        f'{BASE_URL}/posts',
        headers={
            'Authorization': f'Bearer {token}'
        },
        json={
            'title': '新文章',
            'content': '文章内容',
            'published': False
        }
    )
    return response.json()

# 使用示例
token = login()
post = create_post(token)
print(post)
```

### 常见问题 (FAQ)

**Q: JWT token 过期了怎么办？**  
A: 重新调用 `/auth/login` 接口获取新 token。

**Q: 如何实现"记住我"功能？**  
A: 登录时设置 `remember_me: true`，会话有效期延长至30天。

**Q: 文件上传失败怎么办？**  
A: 检查文件大小（最大10MB）和文件类型（仅支持特定 MIME 类型）。

**Q: GitHub OAuth 配置？**  
A: 在 GitHub 创建 OAuth App，获取 Client ID 和 Client Secret，设置到环境变量。

**Q: 如何查看文章的完整统计数据？**  
A: 管理员权限访问 `/stats/total` 接口获取完整统计。

---

**文档版本：** 1.0.0  
**最后更新：** 2026-01-29  
**API 版本：** v1
