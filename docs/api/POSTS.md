# Posts API Documentation

## Overview

The Posts API provides comprehensive blog post management including CRUD operations, publishing workflow, category assignment, and tag management.

**Base URL:** `http://localhost:3000/api/posts`

---

## Endpoints

### List Posts

Get posts with optional filtering. By default, returns only published posts.

**Endpoint:** `GET /api/posts`

**Authentication:** Optional

**Query Parameters:**

| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `page` | integer | No | 1 | Page number (1-based) |
| `per_page` | integer | No | 20 | Items per page |
| `author` | UUID | No | - | Filter by author ID |
| `category` | UUID | No | - | Filter by category ID |
| `tag` | UUID | No | - | Filter by tag ID |
| `status` | string | No | `published` | `published`, `draft`, or `all` (admin/owner only) |

**Response (200 OK):**

```json
{
  "code": 200,
  "message": "success",
  "data": [
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
  ],
  "pagination": {
    "page": 1,
    "per_page": 20,
    "total": 100,
    "total_pages": 5
  }
}
```

**Examples:**

```bash
# Get published posts (default)
GET /api/posts

# Get page 2 with 50 items per page
GET /api/posts?page=2&per_page=50

# Get posts by specific author
GET /api/posts?author=660e8400-e29b-41d4-a716-446655440001

# Get posts in a category
GET /api/posts?category=770e8400-e29b-41d4-a716-446655440002

# Get posts with a specific tag
GET /api/posts?tag=880e8400-e29b-41d4-a716-446655440003

# Get draft posts (requires authentication as author or admin)
GET /api/posts?status=draft
```

---

### Search Posts

Search posts by query string.

**Endpoint:** `GET /api/posts/search`

**Query Parameters:**

| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `q` | string | Yes | - | Search query |
| `page` | integer | No | 1 | Page number |
| `per_page` | integer | No | 20 | Items per page |

**Response:** Same format as List Posts

**Example:**

```bash
GET /api/posts/search?q=rust%20tutorial&page=1&per_page=10
```

---

### Create Post

Create a new blog post. Initially created as unpublished (draft).

**Endpoint:** `POST /api/posts`

**Authentication:** Required

**Permission:** `POST_CREATE` (0x1)

**Request Body:**

```json
{
  "title": "My New Post",
  "content": "Post content goes here..."
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `title` | string | Yes | Post title |
| `content` | string | Yes | Post content (Markdown supported) |

**Response (201 Created):**

```json
{
  "code": 201,
  "message": "created",
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "user_id": "660e8400-e29b-41d4-a716-446655440001",
    "title": "My New Post",
    "content": "Post content goes here...",
    "category_id": null,
    "published_at": null,
    "created_at": "2026-01-30T12:00:00Z",
    "views": 0
  }
}
```

---

### Get Post

Retrieve a single post by ID.

**Endpoint:** `GET /api/posts/{id}`

**Authentication:** Optional (required for draft posts)

**Path Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `id` | UUID | Yes | Post ID |

**Response (200 OK):**

```json
{
  "code": 200,
  "message": "success",
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "user_id": "660e8400-e29b-41d4-a716-446655440001",
    "title": "My First Post",
    "content": "Full post content...",
    "category_id": "770e8400-e29b-41d4-a716-446655440002",
    "published_at": "2026-01-30T10:00:00Z",
    "created_at": "2026-01-30T09:00:00Z",
    "views": 150
  }
}
```

**Access Control:**
- Published posts: Visible to everyone
- Draft posts: Only visible to post owner and admins (returns 404 for others)

---

### Update Post (Full)

Full update of an existing post.

**Endpoint:** `PUT /api/posts/{id}`

**Authentication:** Required

**Permission:** Post owner or admin

**Request Body:**

```json
{
  "title": "Updated Title",
  "content": "Updated content..."
}
```

**Response (200 OK):** Updated post object

---

### Update Post (Partial)

Partial update of a post. Use this for:
- Publishing/unpublishing posts
- Changing category
- Updating title or content

**Endpoint:** `PATCH /api/posts/{id}`

**Authentication:** Required

**Permission:** Post owner or admin

**Request Body:**

```json
{
  "title": "Updated Title",
  "content": "Updated content",
  "category_id": "770e8400-e29b-41d4-a716-446655440002",
  "status": "published"
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `title` | string | No | New post title |
| `content` | string | No | New post content |
| `category_id` | string/null | No | Category ID (empty string to remove) |
| `status` | string | No | `"published"` or `"draft"` |

**Examples:**

```bash
# Publish a post
curl -X PATCH /api/posts/{id} \
  -H "Authorization: Bearer $TOKEN" \
  -d '{"status": "published"}'

# Unpublish a post
curl -X PATCH /api/posts/{id} \
  -H "Authorization: Bearer $TOKEN" \
  -d '{"status": "draft"}'

# Change category
curl -X PATCH /api/posts/{id} \
  -H "Authorization: Bearer $TOKEN" \
  -d '{"category_id": "new-category-id"}'

# Remove category
curl -X PATCH /api/posts/{id} \
  -H "Authorization: Bearer $TOKEN" \
  -d '{"category_id": ""}'

# Update title and publish
curl -X PATCH /api/posts/{id} \
  -H "Authorization: Bearer $TOKEN" \
  -d '{"title": "New Title", "status": "published"}'
```

**Response (200 OK):** Updated post object

---

### Delete Post

Permanently delete a post.

**Endpoint:** `DELETE /api/posts/{id}`

**Authentication:** Required

**Permission:** Post owner or admin

**Response (204 No Content):** Empty response body

---

### Get Post Comments

Get all comments for a post.

**Endpoint:** `GET /api/posts/{id}/comments`

**Authentication:** Not required

**Response (200 OK):**

```json
{
  "code": 200,
  "message": "success",
  "data": [
    {
      "id": "...",
      "post_id": "...",
      "content": "Great post!",
      "created_at": "2026-01-30T10:00:00Z"
    }
  ]
}
```

---

### Create Comment

Add a comment to a post.

**Endpoint:** `POST /api/posts/{id}/comments`

**Authentication:** Required

**Request Body:**

```json
{
  "content": "Great post!"
}
```

**Response (201 Created):** Created comment object

---

### Get Post Tags

Get all tags associated with a post.

**Endpoint:** `GET /api/posts/{id}/tags`

**Authentication:** Not required

**Response (200 OK):**

```json
{
  "code": 200,
  "message": "success",
  "data": [
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
}
```

---

### Add Tag to Post

Associate a tag with a post.

**Endpoint:** `POST /api/posts/{id}/tags`

**Authentication:** Required

**Permission:** Post owner or admin

**Request Body:**

```json
{
  "tag_id": "880e8400-e29b-41d4-a716-446655440003"
}
```

**Response (201 Created):** Updated tags list

---

### Remove Tag from Post

Remove a tag association from a post.

**Endpoint:** `DELETE /api/posts/{id}/tags/{tag_id}`

**Authentication:** Required

**Permission:** Post owner or admin

**Response (200 OK):** Updated tags list

---

## Publishing Workflow

Posts follow a simple workflow:

1. **Create** → Post starts as draft (`published_at: null`)
2. **Publish** → Use `PATCH /posts/{id}` with `{"status": "published"}`
3. **Unpublish** → Use `PATCH /posts/{id}` with `{"status": "draft"}`
4. **Update** → Can update at any state using `PUT` or `PATCH`
5. **Delete** → Permanently removes post

**Visibility Rules:**
- Draft posts: Visible only to owner and admins
- Published posts: Visible to everyone

---

## Permission System

| Permission | Value | Description |
|-----------|-------|-------------|
| `POST_CREATE` | 1 | Create new posts |
| `POST_UPDATE` | 2 | Update posts |
| `POST_DELETE` | 4 | Delete posts |
| `POST_PUBLISH` | 8 | Publish/unpublish posts |

**Default User Permissions:** `POST_CREATE | POST_UPDATE | POST_PUBLISH` (11)

**Admin Permissions:** All permissions (31)

---

## Error Responses

### 400 Bad Request

```json
{
  "code": 400,
  "message": "Validation failed",
  "errors": {
    "title": ["Title cannot be empty"]
  }
}
```

### 401 Unauthorized

```json
{
  "code": 401,
  "message": "Authentication required"
}
```

### 403 Forbidden

```json
{
  "code": 403,
  "message": "You don't have permission to update this post"
}
```

### 404 Not Found

```json
{
  "code": 404,
  "message": "Post not found"
}
```

---

## Related Documentation

- [Authentication API](./AUTH.md) - How to authenticate requests
- [Categories API](./CATEGORIES.md) - Managing post categories
- [Tags API](./TAGS.md) - Managing post tags
- [Comments API](./COMMENTS.md) - Managing comments
- [Stats API](./STATS.md) - Post view statistics

---

**Last Updated:** 2026-01-31  
**API Version:** v2
