# Users API Documentation

## Overview

The Users API provides user management functionality. Regular users can manage their own accounts, while admins can manage all users.

**Base URL:** `http://localhost:3000/api/users`

---

## Endpoints

### List Users

Get all users (admin only).

**Endpoint:** `GET /api/users`

**Authentication:** Required

**Permission:** `USER_MANAGE` (admin only)

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
      "username": "johndoe",
      "permissions": 11,
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

### Get User

Get a single user by ID.

**Endpoint:** `GET /api/users/{id}`

**Authentication:** Required

**Permission:** Self (own account) or admin

**Response (200 OK):**

```json
{
  "code": 200,
  "message": "success",
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "username": "johndoe",
    "permissions": 11,
    "created_at": "2026-01-30T10:00:00Z"
  }
}
```

**Error Responses:**

```json
// 403 Forbidden - Trying to access another user's data
{
  "code": 403,
  "message": "You can only access your own user data"
}
```

---

### Update User

Update user information (partial update).

**Endpoint:** `PATCH /api/users/{id}`

**Authentication:** Required

**Permission:** Admin only (for permissions update)

**Request Body:**

```json
{
  "permissions": 31
}
```

| Field         | Type    | Required | Description                       |
| ------------- | ------- | -------- | --------------------------------- |
| `permissions` | integer | No       | New permission flags (admin only) |

**Note:** Currently only permissions can be updated via PATCH. Other user profile updates may be added later.

**Response (200 OK):**

```json
{
  "code": 200,
  "message": "success",
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "username": "johndoe",
    "permissions": 31,
    "created_at": "2026-01-30T10:00:00Z"
  }
}
```

**Error Responses:**

```json
// 403 Forbidden - Non-admin trying to update permissions
{
  "code": 403,
  "message": "Only admins can update user permissions"
}

// 400 Bad Request - Invalid permissions
{
  "code": 400,
  "message": "Invalid permissions: cannot remove USER_MANAGE from yourself"
}
```

---

### Delete User

Delete a user account.

**Endpoint:** `DELETE /api/users/{id}`

**Authentication:** Required

**Permission:** Self (own account) or admin

**Important Notes:**

- Users can delete their own account
- Admins can delete any account
- Cannot delete the last admin user

**Response (204 No Content):** Empty response body

**Error Responses:**

```json
// 403 Forbidden - Trying to delete another user
{
  "code": 403,
  "message": "You can only delete your own account"
}

// 400 Bad Request - Trying to delete last admin
{
  "code": 400,
  "message": "Cannot delete the last admin user"
}

// 404 Not Found
{
  "code": 404,
  "message": "User not found"
}
```

---

### Get User Posts

Get all posts by a specific user.

**Endpoint:** `GET /api/users/{id}/posts`

**Authentication:** Not required

**Query Parameters:**

| Parameter  | Type    | Required | Default | Description                                           |
| ---------- | ------- | -------- | ------- | ----------------------------------------------------- |
| `page`     | integer | No       | 1       | Page number                                           |
| `per_page` | integer | No       | 20      | Items per page                                        |
| `include`  | string  | No       | -       | Use `drafts` to include draft posts (self/admin only) |

**Response (200 OK):** Paginated list of posts

```json
{
  "code": 200,
  "message": "success",
  "data": [
    {
      "id": "660e8400-e29b-41d4-a716-446655440001",
      "title": "My Post",
      "content": "Post content...",
      "published_at": "2026-01-30T10:00:00Z",
      "created_at": "2026-01-30T09:00:00Z"
    }
  ],
  "pagination": {
    "page": 1,
    "per_page": 20,
    "total": 5,
    "total_pages": 1
  }
}
```

**Notes:**

- By default, only published posts are returned
- Use `?include=drafts` to include draft posts (requires authentication as the user or admin)

---

## Permission System

| Permission     | Value | Description                    |
| -------------- | ----- | ------------------------------ |
| `POST_CREATE`  | 1     | Create posts                   |
| `POST_UPDATE`  | 2     | Update posts                   |
| `POST_DELETE`  | 4     | Delete posts                   |
| `POST_PUBLISH` | 8     | Publish/unpublish posts        |
| `USER_MANAGE`  | 16    | Manage users, categories, tags |

**Default User Permissions:** `POST_CREATE | POST_UPDATE | POST_PUBLISH` (11)

**Admin Permissions:** All permissions (31)

---

## Related Documentation

- [Authentication API](./AUTH.md) - User authentication
- [Posts API](./POSTS.md) - User posts management
