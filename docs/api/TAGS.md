# Tags API Documentation

## Overview

The Tags API provides tag management for organizing and filtering blog posts.

**Base URL:** `http://localhost:3000/api/tags`

---

## Endpoints

### List Tags

Get all tags.

**Endpoint:** `GET /api/tags`

**Authentication:** Not required

**Query Parameters:**

| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `page` | integer | No | 1 | Page number |
| `per_page` | integer | No | 50 | Items per page |

**Response (200 OK):**

```json
{
  "code": 200,
  "message": "success",
  "data": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "name": "rust",
      "slug": "rust",
      "created_at": "2026-01-30T10:00:00Z"
    },
    {
      "id": "660e8400-e29b-41d4-a716-446655440001",
      "name": "web-development",
      "slug": "web-development",
      "created_at": "2026-01-30T10:00:00Z"
    }
  ],
  "pagination": {
    "page": 1,
    "per_page": 50,
    "total": 25,
    "total_pages": 1
  }
}
```

---

### Create Tag

Create a new tag.

**Endpoint:** `POST /api/tags`

**Authentication:** Required

**Permission:** `USER_MANAGE` (admin only)

**Request Body:**

```json
{
  "name": "javascript",
  "slug": "javascript"
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | string | Yes | Tag name |
| `slug` | string | Yes | URL-friendly identifier |

**Response (201 Created):**

```json
{
  "code": 201,
  "message": "created",
  "data": {
    "id": "770e8400-e29b-41d4-a716-446655440002",
    "name": "javascript",
    "slug": "javascript",
    "created_at": "2026-01-30T12:00:00Z"
  }
}
```

---

### Get Tag

Get a single tag by ID.

**Endpoint:** `GET /api/tags/{id}`

**Authentication:** Not required

**Response (200 OK):**

```json
{
  "code": 200,
  "message": "success",
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "name": "rust",
    "slug": "rust",
    "created_at": "2026-01-30T10:00:00Z"
  }
}
```

---

### Get Tag Posts

Get all posts with a specific tag.

**Endpoint:** `GET /api/tags/{id}/posts`

**Authentication:** Not required

**Query Parameters:**

| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `page` | integer | No | 1 | Page number |
| `per_page` | integer | No | 20 | Items per page |

**Response (200 OK):** Paginated list of posts

---

### Delete Tag

Delete a tag.

**Endpoint:** `DELETE /api/tags/{id}`

**Authentication:** Required

**Permission:** `USER_MANAGE` (admin only)

**Response (204 No Content):** Empty body

---

## Managing Post Tags

To add or remove tags from posts, use the Posts API:

```bash
# Add tag to post
POST /api/posts/{post_id}/tags
{
  "tag_id": "550e8400-e29b-41d4-a716-446655440000"
}

# Remove tag from post
DELETE /api/posts/{post_id}/tags/{tag_id}

# Get post tags
GET /api/posts/{post_id}/tags
```

See [Posts API](./POSTS.md) for more details.

---

## Related Documentation

- [Posts API](./POSTS.md) - Managing post tags
