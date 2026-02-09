# Categories API Documentation

## Overview

The Categories API provides hierarchical category management for organizing blog posts.

**Base URL:** `http://localhost:3000/api/categories`

---

## Endpoints

### List Categories

Get all categories.

**Endpoint:** `GET /api/categories`

**Authentication:** Not required

**Query Parameters:**

| Parameter  | Type    | Required | Default | Description    |
| ---------- | ------- | -------- | ------- | -------------- |
| `page`     | integer | No       | 1       | Page number    |
| `per_page` | integer | No       | 50      | Items per page |

**Response (200 OK):**

```json
{
  "code": 200,
  "message": "success",
  "data": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "name": "Technology",
      "slug": "technology",
      "description": "Tech-related posts",
      "parent_id": null,
      "created_at": "2026-01-30T10:00:00Z"
    },
    {
      "id": "660e8400-e29b-41d4-a716-446655440001",
      "name": "Programming",
      "slug": "programming",
      "description": "Programming tutorials",
      "parent_id": "550e8400-e29b-41d4-a716-446655440000",
      "created_at": "2026-01-30T10:00:00Z"
    }
  ],
  "pagination": {
    "page": 1,
    "per_page": 50,
    "total": 10,
    "total_pages": 1
  }
}
```

---

### Create Category

Create a new category.

**Endpoint:** `POST /api/categories`

**Authentication:** Required

**Permission:** `USER_MANAGE` (admin only)

**Request Body:**

```json
{
  "name": "Web Development",
  "slug": "web-development",
  "description": "Web development tutorials and articles",
  "parent_id": "550e8400-e29b-41d4-a716-446655440000"
}
```

| Field         | Type      | Required | Description             |
| ------------- | --------- | -------- | ----------------------- |
| `name`        | string    | Yes      | Category name           |
| `slug`        | string    | Yes      | URL-friendly identifier |
| `description` | string    | No       | Category description    |
| `parent_id`   | UUID/null | No       | Parent category ID      |

**Response (201 Created):**

```json
{
  "code": 201,
  "message": "created",
  "data": {
    "id": "770e8400-e29b-41d4-a716-446655440002",
    "name": "Web Development",
    "slug": "web-development",
    "description": "Web development tutorials and articles",
    "parent_id": "550e8400-e29b-41d4-a716-446655440000",
    "created_at": "2026-01-30T12:00:00Z"
  }
}
```

---

### Get Category

Get a single category by ID.

**Endpoint:** `GET /api/categories/{id}`

**Authentication:** Not required

**Response (200 OK):**

```json
{
  "code": 200,
  "message": "success",
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "name": "Technology",
    "slug": "technology",
    "description": "Tech-related posts",
    "parent_id": null,
    "created_at": "2026-01-30T10:00:00Z"
  }
}
```

---

### Get Category Posts

Get all posts in a category.

**Endpoint:** `GET /api/categories/{id}/posts`

**Authentication:** Not required

**Query Parameters:**

| Parameter  | Type    | Required | Default | Description    |
| ---------- | ------- | -------- | ------- | -------------- |
| `page`     | integer | No       | 1       | Page number    |
| `per_page` | integer | No       | 20      | Items per page |

**Response (200 OK):** Paginated list of posts

---

### Update Category

Update a category (partial update).

**Endpoint:** `PATCH /api/categories/{id}`

**Authentication:** Required

**Permission:** `USER_MANAGE` (admin only)

**Request Body:**

```json
{
  "name": "New Name",
  "description": "Updated description"
}
```

**Response (200 OK):** Updated category object

---

### Delete Category

Delete a category.

**Endpoint:** `DELETE /api/categories/{id}`

**Authentication:** Required

**Permission:** `USER_MANAGE` (admin only)

**Response (204 No Content):** Empty body

---

## Related Documentation

- [Posts API](./POSTS.md) - Managing posts in categories
