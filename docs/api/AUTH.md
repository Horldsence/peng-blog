# Authentication API Documentation

## Overview

The Authentication API provides user registration and login functionality using JWT tokens.

**Base URL:** `http://localhost:3000/api/auth`

---

## Endpoints

### Register

Register a new user account.

**Endpoint:** `POST /api/auth/register`

**Authentication:** Not required

**Request Body:**

```json
{
  "username": "johndoe",
  "password": "SecurePass123!"
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `username` | string | Yes | 3-30 characters, unique |
| `password` | string | Yes | Minimum 8 characters |

**Response (201 Created):**

```json
{
  "code": 201,
  "message": "created",
  "data": {
    "token": "eyJhbGciOiJIUzI1NiIs...",
    "user": {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "username": "johndoe",
      "permissions": 11
    }
  }
}
```

**Error Responses:**

```json
// 400 - Validation error
{
  "code": 400,
  "message": "Username must be at least 3 characters"
}

// 409 - Username already exists
{
  "code": 409,
  "message": "Username already taken"
}
```

---

### Login

Authenticate and get a JWT token.

**Endpoint:** `POST /api/auth/login`

**Authentication:** Not required

**Request Body:**

```json
{
  "username": "johndoe",
  "password": "SecurePass123!"
}
```

**Response (200 OK):**

```json
{
  "code": 200,
  "message": "success",
  "data": {
    "token": "eyJhbGciOiJIUzI1NiIs...",
    "user": {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "username": "johndoe",
      "permissions": 11
    }
  }
}
```

**Error Responses:**

```json
// 401 - Invalid credentials
{
  "code": 401,
  "message": "Invalid username or password"
}
```

---

### Logout

Inform the client to remove the token. Note: JWT tokens are stateless, so actual logout happens client-side.

**Endpoint:** `POST /api/auth/logout`

**Authentication:** Not required (token removal is client-side)

**Response (200 OK):**

```json
{
  "code": 200,
  "message": "success",
  "data": {
    "message": "Logout successful. Please remove the token from client storage."
  }
}
```

---

### Get Current User

Get information about the currently authenticated user.

**Endpoint:** `GET /api/auth/me`

**Authentication:** Required

**Response (200 OK):**

```json
{
  "code": 200,
  "message": "success",
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "username": "johndoe",
    "permissions": 11
  }
}
```

---

## Using the Token

Include the JWT token in the Authorization header for authenticated requests:

```bash
Authorization: Bearer eyJhbGciOiJIUzI1NiIs...
```

**Example:**

```bash
curl http://localhost:3000/api/posts \
  -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIs..."
```

---

## Permission Flags

The `permissions` field in user info is a bit flag value:

| Permission | Value | Description |
|-----------|-------|-------------|
| `POST_CREATE` | 1 | Create posts |
| `POST_UPDATE` | 2 | Update posts |
| `POST_DELETE` | 4 | Delete posts |
| `POST_PUBLISH` | 8 | Publish/unpublish posts |
| `USER_MANAGE` | 16 | Manage users, categories, tags |

**Default user:** 11 (POST_CREATE | POST_UPDATE | POST_PUBLISH)  
**Admin:** 31 (all permissions)

---

## Session-based Authentication

For web applications that prefer cookie-based sessions, use the [Sessions API](./SESSIONS.md) instead.

---

## Related Documentation

- [Sessions API](./SESSIONS.md) - Cookie-based authentication
- [Users API](./USERS.md) - User management
