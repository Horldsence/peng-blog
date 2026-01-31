# Posts API Documentation

## Overview

The Posts API provides comprehensive blog post management including CRUD operations, publishing workflow, category assignment, and tag management. Posts support draft/published states with fine-grained access control.

**Base URL:** `http://localhost:3000/api/posts`

---

## Endpoints

### List Posts

Get all published posts with optional filtering. Admin users see all posts including unpublished drafts.

**Endpoint:** `GET /api/posts`

**Authentication:** Optional (JWT via `Authorization` header)

**Query Parameters:**

| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `limit` | integer | No | 20 | Maximum number of posts to return |
| `user_id` | UUID | No | - | Filter by specific user |
| `category_id` | UUID | No | - | Filter by category |
| `tag_id` | UUID | No | - | Filter by tag |

**Response (200 OK):**

```json
[
  {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "user_id": "660e8400-e29b-41d4-a716-446655440001",
    "title": "My First Post",
    "content": "Post content here...",
    "category_id": "770e8400-e29b-41d4-a716-446655440002",
    "published_at": "2026-01-30T10:00:00Z",
    "created_at": "2026-01-30T09:00:00Z",
    "views": 150
  }
]
```

**Access Control:**
- **Public:** See only published posts
- **Authenticated (non-admin):** See only published posts
- **Admin:** See all posts (published + unpublished)

**Examples:**

```bash
# Get latest 20 published posts
GET /api/posts

# Get latest 50 posts
GET /api/posts?limit=50

# Get posts by specific user
GET /api/posts?user_id=660e8400-e29b-41d4-a716-446655440001

# Get posts in a category
GET /api/posts?category_id=770e8400-e29b-41d4-a716-446655440002

# Get posts with a specific tag
GET /api/posts?tag_id=880e8400-e29b-41d4-a716-446655440003

# Get posts with both category AND tag (AND logic)
GET /api/posts?category_id=770e8400-e29b-41d4-a716-446655440002&tag_id=880e8400-e29b-41d4-a716-446655440003
```

---

### Get Post by ID

Retrieve a single post by ID.

**Endpoint:** `GET /api/posts/{id}`

**Authentication:** Optional

**Path Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `id` | UUID | Yes | Post ID |

**Response (200 OK):**

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "user_id": "660e8400-e29b-41d4-a716-446655440001",
  "title": "My First Post",
  "content": "Full post content...",
  "category_id": "770e8400-e29b-41d4-a716-446655440002",
  "published_at": "2026-01-30T10:00:00Z",
  "created_at": "2026-01-30T09:00:00Z",
  "views": 150
}
```

**Access Control:**
- **Published posts:** Visible to everyone
- **Unpublished posts:** Only visible to post owner and admins

**Error Responses:**

```json
// 404 Not Found
{
  "error": "Post not found"
}
```

---

### Create Post

Create a new blog post. Initially created as unpublished (draft).

**Endpoint:** `POST /api/posts`

**Authentication:** Required (JWT)

**Permission:** `POST_CREATE` (0x1)

**Request Body:**

```json
{
  "title": "My New Post",
  "content": "Post content goes here..."
}
```

**Request Parameters:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `title` | string | Yes | Post title |
| `content` | string | Yes | Post content (Markdown supported) |

**Response (201 Created):**

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "user_id": "660e8400-e29b-41d4-a716-446655440001",
  "title": "My New Post",
  "content": "Post content goes here...",
  "category_id": null,
  "published_at": null,
  "created_at": "2026-01-30T12:00:00Z",
  "views": 0
}
```

**Error Responses:**

```json
// 400 Bad Request - Validation error
{
  "error": "Validation failed: Title cannot be empty"
}

// 401 Unauthorized
{
  "error": "Missing or invalid authorization token"
}

// 403 Forbidden - Insufficient permissions
{
  "error": "Permission denied: requires permission flag 0x1"
}
```

**Example:**

```bash
curl -X POST http://localhost:3000/api/posts \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "title": "My New Post",
    "content": "This is my first blog post!"
  }'
```

---

### Update Post

Update an existing post's title and/or content.

**Endpoint:** `PUT /api/posts/{id}`

**Authentication:** Required (JWT)

**Permission:** Post owner OR admin

**Path Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `id` | UUID | Yes | Post ID |

**Request Body:**

```json
{
  "title": "Updated Title",
  "content": "Updated content..."
}
```

**Request Parameters:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `title` | string | No | New post title |
| `content` | string | No | New post content |

**Note:** At least one field must be provided.

**Response (200 OK):**

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "user_id": "660e8400-e29b-41d4-a716-446655440001",
  "title": "Updated Title",
  "content": "Updated content...",
  "category_id": "770e8400-e29b-41d4-a716-446655440002",
  "published_at": "2026-01-30T10:00:00Z",
  "created_at": "2026-01-30T09:00:00Z",
  "views": 150
}
```

**Error Responses:**

```json
// 403 Forbidden - Not owner
{
  "error": "Permission denied: you must be the resource owner or have admin privileges"
}

// 404 Not Found
{
  "error": "Post not found"
}
```

**Example:**

```bash
curl -X PUT http://localhost:3000/api/posts/550e8400-e29b-41d4-a716-446655440000 \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Updated Title"
  }'
```

---

### Delete Post

Permanently delete a post.

**Endpoint:** `DELETE /api/posts/{id}`

**Authentication:** Required (JWT)

**Permission:** Post owner OR admin

**Path Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `id` | UUID | Yes | Post ID |

**Response (204 No Content):**

Empty response body.

**Error Responses:**

```json
// 403 Forbidden
{
  "error": "Permission denied: you must be the resource owner or have admin privileges"
}

// 404 Not Found
{
  "error": "Post not found"
}
```

**Example:**

```bash
curl -X DELETE http://localhost:3000/api/posts/550e8400-e29b-41d4-a716-446655440000 \
  -H "Authorization: Bearer $TOKEN"
```

---

### Publish Post

Publish a draft post, making it visible to the public.

**Endpoint:** `POST /api/posts/{id}/publish`

**Authentication:** Required (JWT)

**Permission:** Post owner OR admin

**Path Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `id` | UUID | Yes | Post ID |

**Response (200 OK):**

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "user_id": "660e8400-e29b-41d4-a716-446655440001",
  "title": "My Post",
  "content": "Post content...",
  "category_id": null,
  "published_at": "2026-01-30T13:00:00Z",
  "created_at": "2026-01-30T09:00:00Z",
  "views": 0
}
```

**Error Responses:**

```json
// 403 Forbidden
{
  "error": "Permission denied: you must be the resource owner or have admin privileges"
}

// 404 Not Found
{
  "error": "Post not found"
}
```

**Example:**

```bash
curl -X POST http://localhost:3000/api/posts/550e8400-e29b-41d4-a716-446655440000/publish \
  -H "Authorization: Bearer $TOKEN"
```

---

### Unpublish Post

Unpublish a post, reverting it to draft status.

**Endpoint:** `POST /api/posts/{id}/unpublish`

**Authentication:** Required (JWT)

**Permission:** Post owner OR admin

**Path Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `id` | UUID | Yes | Post ID |

**Response (200 OK):**

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "user_id": "660e8400-e29b-41d4-a716-446655440001",
  "title": "My Post",
  "content": "Post content...",
  "category_id": null,
  "published_at": null,
  "created_at": "2026-01-30T09:00:00Z",
  "views": 150
}
```

**Note:** The `published_at` field becomes `null` when unpublished.

**Example:**

```bash
curl -X POST http://localhost:3000/api/posts/550e8400-e29b-41d4-a716-446655440000/unpublish \
  -H "Authorization: Bearer $TOKEN"
```

---

### Set Post Category

Assign or change a post's category.

**Endpoint:** `PUT /api/posts/{id}/category`

**Authentication:** Required (JWT)

**Permission:** Post owner OR admin

**Path Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `id` | UUID | Yes | Post ID |

**Request Body:**

```json
{
  "category_id": "770e8400-e29b-41d4-a716-446655440002"
}
```

**Request Parameters:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `category_id` | UUID or null | No | Category ID (null to remove category) |

**Response (200 OK):**

```json
{
  "success": true
}
```

**Example:**

```bash
# Assign category
curl -X PUT http://localhost:3000/api/posts/550e8400-e29b-41d4-a716-446655440000/category \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "category_id": "770e8400-e29b-41d4-a716-446655440002"
  }'

# Remove category
curl -X PUT http://localhost:3000/api/posts/550e8400-e29b-41d4-a716-446655440000/category \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "category_id": null
  }'
```

---

### Get Post Tags

Retrieve all tags associated with a post.

**Endpoint:** `GET /api/posts/{id}/tags`

**Authentication:** Not required

**Path Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `id` | UUID | Yes | Post ID |

**Response (200 OK):**

```json
[
  {
    "id": "880e8400-e29b-41d4-a716-446655440003",
    "name": "rust",
    "slug": "rust"
  },
  {
    "id": "990e8400-e29b-41d4-a716-446655440004",
    "name": "web-development",
    "slug": "web-development"
  }
]
```

**Example:**

```bash
curl http://localhost:3000/api/posts/550e8400-e29b-41d4-a716-446655440000/tags
```

---

### Add Tag to Post

Associate a tag with a post.

**Endpoint:** `POST /api/posts/{id}/tags/{tag_id}`

**Authentication:** Required (JWT)

**Permission:** Post owner OR admin

**Path Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `id` | UUID | Yes | Post ID |
| `tag_id` | UUID | Yes | Tag ID |

**Response (201 Created):**

```json
{
  "success": true
}
```

**Error Responses:**

```json
// 403 Forbidden
{
  "error": "Permission denied: you must be the resource owner or have admin privileges"
}

// 404 Not Found
{
  "error": "Post not found"
}
```

**Example:**

```bash
curl -X POST http://localhost:3000/api/posts/550e8400-e29b-41d4-a716-446655440000/tags/880e8400-e29b-41d4-a716-446655440003 \
  -H "Authorization: Bearer $TOKEN"
```

---

### Remove Tag from Post

Remove a tag association from a post.

**Endpoint:** `DELETE /api/posts/{id}/tags/{tag_id}`

**Authentication:** Required (JWT)

**Permission:** Post owner OR admin

**Path Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `id` | UUID | Yes | Post ID |
| `tag_id` | UUID | Yes | Tag ID |

**Response (200 OK):**

```json
{
  "success": true
}
```

**Example:**

```bash
curl -X DELETE http://localhost:3000/api/posts/550e8400-e29b-41d4-a716-446655440000/tags/880e8400-e29b-41d4-a716-446655440003 \
  -H "Authorization: Bearer $TOKEN"
```

---

## Permission System

Posts use the following permission flags:

| Permission | Value | Hex | Description |
|-----------|-------|-----|-------------|
| `POST_CREATE` | 1 | 0x1 | Create new posts |
| `POST_UPDATE` | 2 | 0x2 | Update posts |
| `POST_DELETE` | 4 | 0x4 | Delete posts |
| `POST_PUBLISH` | 8 | 0x8 | Publish/unpublish posts |

**Default User Permissions:** `POST_CREATE | POST_UPDATE | POST_PUBLISH` (11)

**Admin Permissions:** All permissions (31)

---

## Publishing Workflow

Posts follow a simple publish/unpublish workflow:

1. **Create** → Post starts as draft (`published_at: null`)
2. **Publish** → Post becomes public (`published_at` set to current time)
3. **Unpublish** → Post reverts to draft (`published_at: null`)
4. **Update** → Can update regardless of published state
5. **Delete** → Permanently removes post

**Visibility Rules:**
- Draft posts: Visible only to owner and admins
- Published posts: Visible to everyone

---

## Related Documentation

- [Authentication API](./AUTH.md) - How to authenticate requests
- [Categories API](./CATEGORIES.md) - Managing post categories
- [Tags API](./TAGS.md) - Managing post tags
- [Stats API](./STATS.md) - Post view statistics

---

**Last Updated:** 2026-01-30  
**API Version:** v1
