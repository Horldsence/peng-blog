# Categories API Documentation

## Overview

The Categories API provides hierarchical category management for organizing blog posts. Categories support parent-child relationships, allowing for structured content organization.

**Base URL:** `http://localhost:3000/api/categories`

---

## Endpoints

### List All Categories

Retrieve all categories in a flat list.

**Endpoint:** `GET /api/categories`

**Authentication:** Not required (public endpoint)

**Response (200 OK):**

```json
[
  {
    "id": "770e8400-e29b-41d4-a716-446655440002",
    "name": "Programming",
    "slug": "programming",
    "parent_id": null
  },
  {
    "id": "780e8400-e29b-41d4-a716-446655440003",
    "name": "Web Development",
    "slug": "web-development",
    "parent_id": "770e8400-e29b-41d4-a716-446655440002"
  },
  {
    "id": "790e8400-e29b-41d4-a716-446655440004",
    "name": "Rust",
    "slug": "rust",
    "parent_id": "770e8400-e29b-41d4-a716-446655440002"
  }
]
```

**Example:**

```bash
curl http://localhost:3000/api/categories
```

---

### Create Category

Create a new category (admin only).

**Endpoint:** `POST /api/categories`

**Authentication:** Required (JWT)

**Permission:** `USER_MANAGE` (0x10)

**Request Body:**

```json
{
  "name": "Web Development",
  "slug": "web-development",
  "parent_id": "770e8400-e29b-41d4-a716-446655440002"
}
```

**Request Parameters:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | string | Yes | Display name of the category |
| `slug` | string | Yes | URL-friendly identifier |
| `parent_id` | UUID or null | No | Parent category ID (null for top-level) |

**Response (201 Created):**

```json
{
  "id": "780e8400-e29b-41d4-a716-446655440003",
  "name": "Web Development",
  "slug": "web-development",
  "parent_id": "770e8400-e29b-41d4-a716-446655440002"
}
```

**Error Responses:**

```json
// 400 Bad Request - Validation error
{
  "error": "Validation failed: Category name cannot be empty"
}

// 401 Unauthorized
{
  "error": "Missing or invalid authorization token"
}

// 403 Forbidden
{
  "error": "Permission denied: requires permission flag 0x10"
}
```

**Example:**

```bash
# Create top-level category
curl -X POST http://localhost:3000/api/categories \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Programming",
    "slug": "programming"
  }'

# Create sub-category
curl -X POST http://localhost:3000/api/categories \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Web Development",
    "slug": "web-development",
    "parent_id": "770e8400-e29b-41d4-a716-446655440002"
  }'
```

---

### Get Category by ID

Retrieve a specific category.

**Endpoint:** `GET /api/categories/{id}`

**Authentication:** Not required

**Path Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `id` | UUID | Yes | Category ID |

**Response (200 OK):**

```json
{
  "id": "780e8400-e29b-41d4-a716-446655440003",
  "name": "Web Development",
  "slug": "web-development",
  "parent_id": "770e8400-e29b-41d4-a716-446655440002"
}
```

**Error Responses:**

```json
// 404 Not Found
{
  "error": "Category not found"
}
```

**Example:**

```bash
curl http://localhost:3000/api/categories/780e8400-e29b-41d4-a716-446655440003
```

---

### Update Category

Update an existing category (admin only).

**Endpoint:** `PUT /api/categories/{id}`

**Authentication:** Required (JWT)

**Permission:** `USER_MANAGE` (0x10)

**Path Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `id` | UUID | Yes | Category ID |

**Request Body:**

```json
{
  "name": "Web Dev",
  "parent_id": null
}
```

**Request Parameters:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | string | No | New category name |
| `parent_id` | UUID or null | No | New parent category ID |

**Note:** At least one field must be provided.

**Response (200 OK):**

```json
{
  "id": "780e8400-e29b-41d4-a716-446655440003",
  "name": "Web Dev",
  "slug": "web-dev",
  "parent_id": null
}
```

**Error Responses:**

```json
// 400 Bad Request
{
  "error": "Validation failed: At least one field must be provided"
}

// 403 Forbidden
{
  "error": "Permission denied: requires permission flag 0x10"
}

// 404 Not Found
{
  "error": "Category not found"
}
```

**Example:**

```bash
curl -X PUT http://localhost:3000/api/categories/780e8400-e29b-41d4-a716-446655440003 \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Web Dev"
  }'
```

---

### Delete Category

Delete a category (admin only).

**Endpoint:** `DELETE /api/categories/{id}`

**Authentication:** Required (JWT)

**Permission:** `USER_MANAGE` (0x10)

**Path Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `id` | UUID | Yes | Category ID |

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
  "error": "Category not found"
}
```

**Example:**

```bash
curl -X DELETE http://localhost:3000/api/categories/780e8400-e29b-41d4-a716-446655440003 \
  -H "Authorization: Bearer $TOKEN"
```

---

### Get Category Children

Retrieve direct child categories of a specific category.

**Endpoint:** `GET /api/categories/{id}/children`

**Authentication:** Not required

**Path Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `id` | UUID | Yes | Parent category ID |

**Response (200 OK):**

```json
[
  {
    "id": "780e8400-e29b-41d4-a716-446655440003",
    "name": "Web Development",
    "slug": "web-development",
    "parent_id": "770e8400-e29b-41d4-a716-446655440002"
  },
  {
    "id": "790e8400-e29b-41d4-a716-446655440004",
    "name": "Rust",
    "slug": "rust",
    "parent_id": "770e8400-e29b-41d4-a716-446655440002"
  }
]
```

**Example:**

```bash
curl http://localhost:3000/api/categories/770e8400-e29b-41d4-a716-446655440002/children
```

---

## Category Hierarchy

Categories support hierarchical organization through parent-child relationships:

```
Programming (parent_id: null)
├── Web Development (parent_id: programming-id)
│   ├── Frontend (parent_id: web-dev-id)
│   └── Backend (parent_id: web-dev-id)
└── Systems Programming (parent_id: programming-id)
    ├── Rust (parent_id: systems-prog-id)
    └── C++ (parent_id: systems-prog-id)
```

**Hierarchy Rules:**
- A category can have only one parent
- A category can have multiple children
- Top-level categories have `parent_id: null`
- No limit on hierarchy depth (theoretical)

---

## Category Fields

| Field | Type | Description |
|-------|------|-------------|
| `id` | UUID | Unique category identifier |
| `name` | string | Display name (e.g., "Web Development") |
| `slug` | string | URL-friendly identifier (e.g., "web-development") |
| `parent_id` | UUID or null | Parent category ID (null for top-level) |

---

## Category vs Tags

**Categories** and **Tags** serve different purposes:

| Feature | Categories | Tags |
|---------|------------|------|
| **Purpose** | Hierarchical organization | Flexible content labeling |
| **Structure** | Tree structure (parent/child) | Flat (no hierarchy) |
| **Multiplicity** | One category per post | Multiple tags per post |
| **Management** | Admin only | Admin only |
| **Example** | "Programming > Web Development" | "rust", "tutorial", "beginner" |

**When to Use:**
- **Categories:** Broad subject areas, structured navigation, main content divisions
- **Tags:** Cross-cutting topics, technologies, difficulty levels, themes

---

## Category Assignment

Posts are assigned to categories through the Posts API:

**Set post category:** `PUT /api/posts/{id}/category`

See [Posts API](./POSTS.md#set-post-category) for details.

---

## Permission Requirements

Category management requires admin-level permissions:

| Action | Permission Required | Value | Hex |
|--------|-------------------|-------|-----|
| List categories | None | - | - |
| Get category | None | - | - |
| Get children | None | - | - |
| Create category | `USER_MANAGE` | 16 | 0x10 |
| Update category | `USER_MANAGE` | 16 | 0x10 |
| Delete category | `USER_MANAGE` | 16 | 0x10 |

---

## Related Documentation

- [Posts API](./POSTS.md) - Managing posts and their categories
- [Tags API](./TAGS.md) - Managing flat tag structures
- [Authentication API](./AUTH.md) - How to authenticate admin requests

---

**Last Updated:** 2026-01-30  
**API Version:** v1
