# Tags API Documentation

## Overview

The Tags API provides tag management for organizing and categorizing blog posts. Tags are hierarchical labels that can be associated with posts for better content discovery.

**Base URL:** `http://localhost:3000/api/tags`

---

## Endpoints

### List All Tags

Retrieve all available tags.

**Endpoint:** `GET /api/tags`

**Authentication:** Not required (public endpoint)

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
  },
  {
    "id": "a00e8400-e29b-41d4-a716-446655440005",
    "name": "tutorial",
    "slug": "tutorial"
  }
]
```

**Example:**

```bash
curl http://localhost:3000/api/tags
```

---

### Create Tag

Create a new tag (admin only).

**Endpoint:** `POST /api/tags`

**Authentication:** Required (JWT)

**Permission:** `USER_MANAGE` (0x10)

**Request Body:**

```json
{
  "name": "rust-programming",
  "slug": "rust-programming"
}
```

**Request Parameters:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | string | Yes | Display name of the tag |
| `slug` | string | Yes | URL-friendly identifier (lowercase, hyphens) |

**Response (201 Created):**

```json
{
  "id": "880e8400-e29b-41d4-a716-446655440003",
  "name": "rust-programming",
  "slug": "rust-programming"
}
```

**Error Responses:**

```json
// 400 Bad Request - Validation error
{
  "error": "Validation failed: Tag name cannot be empty"
}

// 401 Unauthorized
{
  "error": "Missing or invalid authorization token"
}

// 403 Forbidden - Insufficient permissions
{
  "error": "Permission denied: requires permission flag 0x10"
}
```

**Example:**

```bash
curl -X POST http://localhost:3000/api/tags \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "rust-programming",
    "slug": "rust-programming"
  }'
```

---

### Get Tag by ID

Retrieve a specific tag.

**Endpoint:** `GET /api/tags/{id}`

**Authentication:** Not required

**Path Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `id` | UUID | Yes | Tag ID |

**Response (200 OK):**

```json
{
  "id": "880e8400-e29b-41d4-a716-446655440003",
  "name": "rust",
  "slug": "rust"
}
```

**Error Responses:**

```json
// 404 Not Found
{
  "error": "Tag not found"
}
```

**Example:**

```bash
curl http://localhost:3000/api/tags/880e8400-e29b-41d4-a716-446655440003
```

---

### Delete Tag

Delete a tag (admin only).

**Endpoint:** `DELETE /api/tags/{id}`

**Authentication:** Required (JWT)

**Permission:** `USER_MANAGE` (0x10)

**Path Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `id` | UUID | Yes | Tag ID |

**Response (204 No Content):**

Empty response body.

**Error Responses:**

```json
// 401 Unauthorized
{
  "error": "Missing or invalid authorization token"
}

// 403 Forbidden
{
  "error": "Permission denied: requires permission flag 0x10"
}

// 404 Not Found
{
  "error": "Tag not found"
}
```

**Example:**

```bash
curl -X DELETE http://localhost:3000/api/tags/880e8400-e29b-41d4-a716-446655440003 \
  -H "Authorization: Bearer $TOKEN"
```

---

## Tag Management with Posts

Tags are associated with posts through the Posts API. See [Posts API](./POSTS.md) for details on:

- **Get post tags:** `GET /api/posts/{id}/tags`
- **Add tag to post:** `POST /api/posts/{id}/tags/{tag_id}`
- **Remove tag from post:** `DELETE /api/posts/{id}/tags/{tag_id}`

---

## Tag Fields

| Field | Type | Description |
|-------|------|-------------|
| `id` | UUID | Unique tag identifier |
| `name` | string | Display name (e.g., "Rust Programming") |
| `slug` | string | URL-friendly identifier (e.g., "rust-programming") |

---

## Tag vs Category

**Tags** and **Categories** serve different purposes:

| Feature | Tags | Categories |
|---------|------|------------|
| **Purpose** | Flexible content labeling | Hierarchical organization |
| **Structure** | Flat (no hierarchy) | Tree structure (parent/child) |
| **Multiplicity** | Multiple tags per post | One category per post |
| **Management** | Admin only | Admin only |
| **Example** | "rust", "tutorial", "beginner" | "Programming", "Web Development" |

**Use Cases:**
- **Tags:** Cross-cutting topics, technologies, difficulty levels, themes
- **Categories:** Broad subject areas, structural organization

---

## Permission Requirements

Tag management requires admin-level permissions:

| Action | Permission Required | Value | Hex |
|--------|-------------------|-------|-----|
| List tags | None | - | - |
| Get tag | None | - | - |
| Create tag | `USER_MANAGE` | 16 | 0x10 |
| Delete tag | `USER_MANAGE` | 16 | 0x10 |

---

## Related Documentation

- [Posts API](./POSTS.md) - Managing posts and their tags
- [Categories API](./CATEGORIES.md) - Managing hierarchical categories
- [Authentication API](./AUTH.md) - How to authenticate admin requests

---

**Last Updated:** 2026-01-30  
**API Version:** v1
