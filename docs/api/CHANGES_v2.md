# API v2 变更总结

本文档总结了 API 从 v1 到 v2 的主要变更。

---

## 主要改进

### 1. 统一响应格式

**之前 (v1):**
```json
// 成功
{ "id": "...", "title": "..." }

// 或
{ "success": true }

// 错误
{ "error": "Something went wrong" }
```

**之后 (v2):**
```json
// 成功 (单资源)
{
  "code": 200,
  "message": "success",
  "data": { "id": "...", "title": "..." }
}

// 成功 (列表)
{
  "code": 200,
  "message": "success",
  "data": [...],
  "pagination": {
    "page": 1,
    "per_page": 20,
    "total": 100,
    "total_pages": 5
  }
}

// 错误
{
  "code": 400,
  "message": "Validation failed",
  "errors": { "field": ["error message"] }
}
```

### 2. HTTP 方法语义化

| 操作 | 之前 (v1) | 之后 (v2) |
|------|-----------|-----------|
| 发布文章 | `POST /posts/{id}/publish` | `PATCH /posts/{id}` `{ "status": "published" }` |
| 取消发布 | `POST /posts/{id}/unpublish` | `PATCH /posts/{id}` `{ "status": "draft" }` |
| 设置分类 | `PUT /posts/{id}/category` | `PATCH /posts/{id}` `{ "category_id": "..." }` |
| 修改用户权限 | `PATCH /users/{id}/permissions` | `PATCH /users/{id}` `{ "permissions": ... }` |
| 更新分类 | `PUT /categories/{id}` | `PATCH /categories/{id}` |

### 3. 标签关联改进

**之前:**
```bash
POST /posts/{id}/tags/{tag_id}
```

**之后:**
```bash
POST /posts/{id}/tags
{ "tag_id": "..." }
```

这样更符合 RESTful 设计，使用请求体传递数据。

### 4. 查询参数改进

| 功能 | 之前 (v1) | 之后 (v2) |
|------|-----------|-----------|
| 筛选作者 | `?user_id=xxx` | `?author=xxx` |
| 筛选分类 | `?category_id=xxx` | `?category=xxx` |
| 筛选标签 | `?tag_id=xxx` | `?tag=xxx` |
| 筛选状态 | (无) | `?status=draft` / `?status=all` |
| 分页 | `?limit=20` | `?page=1&per_page=20` |

### 5. 新增端点

| 端点 | 描述 |
|------|------|
| `POST /auth/logout` | 登出端点（告知客户端清除 token） |
| `GET /posts/{id}/comments` | 获取文章评论（从评论模块移至文章模块） |
| `POST /posts/{id}/comments` | 添加评论到文章 |
| `GET /categories/{id}/posts` | 获取分类下的文章 |
| `GET /tags/{id}/posts` | 获取标签下的文章 |

### 6. 移除/变更的端点

| 端点 (v1) | 状态 | 替代方案 (v2) |
|-----------|------|---------------|
| `POST /posts/{id}/publish` | ❌ 移除 | `PATCH /posts/{id}` `{ "status": "published" }` |
| `POST /posts/{id}/unpublish` | ❌ 移除 | `PATCH /posts/{id}` `{ "status": "draft" }` |
| `PUT /posts/{id}/category` | ❌ 移除 | `PATCH /posts/{id}` `{ "category_id": "..." }` |
| `POST /posts/{id}/tags/{tag_id}` | ❌ 移除 | `POST /posts/{id}/tags` `{ "tag_id": "..." }` |
| `PATCH /users/{id}/permissions` | ❌ 移除 | `PATCH /users/{id}` `{ "permissions": ... }` |
| `GET /users/{id}/posts` | 🔄 变更 | 现在只返回已发布文章，`?include=drafts` 获取草稿 |

### 7. 端点简化

**文章分类/标签移除:**

之前:
```bash
# 移除分类
PUT /posts/{id}/category
{ "category_id": null }
```

之后:
```bash
# 移除分类
PATCH /posts/{id}
{ "category_id": "" }
```

---

## 迁移指南

### 发布文章

**v1:**
```bash
curl -X POST /api/posts/{id}/publish \
  -H "Authorization: Bearer $TOKEN"
```

**v2:**
```bash
curl -X PATCH /api/posts/{id} \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"status": "published"}'
```

### 添加标签

**v1:**
```bash
curl -X POST /api/posts/{id}/tags/{tag_id} \
  -H "Authorization: Bearer $TOKEN"
```

**v2:**
```bash
curl -X POST /api/posts/{id}/tags \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"tag_id": "..."}'
```

### 处理响应

**v1:**
```javascript
const data = await response.json();
console.log(data.id); // 直接访问
```

**v2:**
```javascript
const result = await response.json();
if (result.code === 200 || result.code === 201) {
  console.log(result.data.id); // 通过 data 访问
}
```

### 筛选文章

**v1:**
```bash
GET /api/posts?user_id=xxx&category_id=yyy
```

**v2:**
```bash
GET /api/posts?author=xxx&category=yyy
```

---

## 设计原则

API v2 遵循以下 RESTful 设计原则：

1. **资源导向**: URI 表示资源，而非操作
2. **HTTP 方法语义化**:
   - `GET` - 读取
   - `POST` - 创建
   - `PUT` - 全量更新
   - `PATCH` - 部分更新
   - `DELETE` - 删除
3. **统一响应**: 所有响应遵循相同格式
4. **分页标准化**: 使用 `page` 和 `per_page` 参数
5. **关系清晰**: 资源关系通过嵌套 URI 表示

---

## 版本控制

API v2 是当前的默认版本。如需在将来引入 v3，可以通过以下方式：
- URL 路径: `/api/v2/posts` → `/api/v3/posts`
- 或 Accept 头: `Accept: application/vnd.api.v2+json`

当前保持 URL 不变，因为：
1. 项目处于开发阶段
2. 简化客户端实现
3. 文档已全面更新
