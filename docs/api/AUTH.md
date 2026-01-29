# Authentication API Documentation

## Overview

The Authentication API provides endpoints for user registration, login, and session management. Peng Blog uses JWT (JSON Web Token) for authentication, with additional support for cookie-based sessions.

### Authentication Mechanisms

1. **JWT Token Authentication** - Stateless, bearer token authentication
2. **Cookie-based Sessions** - Traditional session management with "remember me" support

Both mechanisms can be used simultaneously, giving flexibility to different client types.

---

## Endpoints

### 1. Register User

Register a new user account.

**Endpoint:** `POST /api/auth/register`

**Authentication:** Not required (public endpoint)

**Request Body:**

```json
{
  "username": "johndoe",
  "password": "SecurePass123!"
}
```

**Request Parameters:**

| Field | Type | Required | Description | Constraints |
|-------|------|----------|-------------|-------------|
| `username` | string | Yes | Unique username for the user | 3-20 characters, alphanumeric only |
| `password` | string | Yes | User's password | Minimum 8 characters, must contain at least one letter and one digit |

**Response (201 Created):**

```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c",
  "user": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "username": "johndoe",
    "permissions": 15
  }
}
```

**Response Fields:**

| Field | Type | Description |
|-------|------|-------------|
| `token` | string | JWT bearer token for authentication |
| `user` | object | User information |
| `user.id` | string | Unique user ID (UUID) |
| `user.username` | string | User's username |
| `user.permissions` | number | User's permission bit flags (see Permission System below) |

**Error Responses:**

```json
// 400 Bad Request - Validation error
{
  "error": "Validation failed: Username must be 3-20 characters"
}

// 409 Conflict - Username already exists
{
  "error": "Username 'johndoe' already exists"
}

// 500 Internal Server Error
{
  "error": "Internal server error"
}
```

**Usage Example (cURL):**

```bash
curl -X POST http://localhost:3000/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "johndoe",
    "password": "SecurePass123!"
  }'
```

**Usage Example (JavaScript):**

```javascript
const response = await fetch('http://localhost:3000/api/auth/register', {
  method: 'POST',
  headers: {
    'Content-Type': 'application/json',
  },
  body: JSON.stringify({
    username: 'johndoe',
    password: 'SecurePass123!',
  }),
});

const data = await response.json();
const token = data.token;
const user = data.user;

// Store token for future requests
localStorage.setItem('token', token);
```

---

### 2. Login

Authenticate with username and password to receive a JWT token.

**Endpoint:** `POST /api/auth/login`

**Authentication:** Not required (public endpoint)

**Request Body:**

```json
{
  "username": "johndoe",
  "password": "SecurePass123!"
}
```

**Request Parameters:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `username` | string | Yes | User's username |
| `password` | string | Yes | User's password |

**Response (200 OK):**

```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c",
  "user": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "username": "johndoe",
    "permissions": 15
  }
}
```

**Error Responses:**

```json
// 401 Unauthorized - Invalid credentials
{
  "error": "Invalid username or password"
}

// 404 Not Found - User doesn't exist
{
  "error": "User not found"
}
```

**Usage Example (cURL):**

```bash
curl -X POST http://localhost:3000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "johndoe",
    "password": "SecurePass123!"
  }'
```

---

### 3. Get Current User

Get information about the currently authenticated user.

**Endpoint:** `GET /api/auth/me`

**Authentication:** Required (JWT bearer token)

**Request Headers:**

```
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

**Response (200 OK):**

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "username": "johndoe",
  "permissions": 15
}
```

**Response Fields:**

| Field | Type | Description |
|-------|------|-------------|
| `id` | string | User's unique ID (UUID) |
| `username` | string | User's username |
| `permissions` | number | User's permission bit flags |

**Error Responses:**

```json
// 401 Unauthorized - Missing or invalid token
{
  "error": "Missing or invalid authorization token"
}

// 401 Unauthorized - Token expired
{
  "error": "Token has expired"
}
```

**Usage Example (cURL):**

```bash
curl -X GET http://localhost:3000/api/auth/me \
  -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
```

**Usage Example (JavaScript):**

```javascript
const token = localStorage.getItem('token');

const response = await fetch('http://localhost:3000/api/auth/me', {
  method: 'GET',
  headers: {
    'Authorization': `Bearer ${token}`,
  },
});

const user = await response.json();
console.log('Current user:', user);
```

---

## Permission System

Peng Blog uses a bit flag-based permission system. Each permission is a power of 2, allowing for easy combination and checking.

### Permission Flags

| Flag | Value | Hex | Description |
|------|-------|-----|-------------|
| `POST_CREATE` | 1 | 0x1 | Permission to create posts |
| `POST_UPDATE` | 2 | 0x2 | Permission to update posts |
| `POST_DELETE` | 4 | 0x4 | Permission to delete posts |
| `POST_PUBLISH` | 8 | 0x8 | Permission to publish posts |
| `USER_MANAGE` | 16 | 0x10 | Permission to manage users (admin only) |

### Default Permissions

**Regular User:** `15` (0x0F)
- Can create posts (1)
- Can update posts (2)
- Can publish posts (8)
- **Cannot** delete posts (4)
- **Cannot** manage users (16)

**Administrator:** `31` (0x1F)
- All permissions combined: 1 + 2 + 4 + 8 + 16 = 31

### Checking Permissions

**Example 1:** Check if user can create posts
```javascript
const canCreatePost = (user.permissions & 1) !== 0; // true if bit 0 is set
```

**Example 2:** Check if user is admin
```javascript
const isAdmin = user.permissions === 31; // true if all permissions
```

**Example 3:** Check if user can delete posts
```javascript
const canDeletePost = (user.permissions & 4) !== 0; // true if bit 2 is set
```

---

## Using JWT Tokens

### Token Structure

JWT tokens have three parts separated by dots:
```
header.payload.signature
```

**Example Token:**
```
eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c
```

### Decoded Payload Example

```json
{
  "sub": "550e8400-e29b-41d4-a716-446655440000",
  "username": "johndoe",
  "permissions": 15,
  "exp": 1737907200,
  "iat": 1737820800
}
```

**Payload Fields:**

| Field | Type | Description |
|-------|------|-------------|
| `sub` | string | Subject (user ID) |
| `username` | string | Username |
| `permissions` | number | Permission bit flags |
| `exp` | number | Expiration timestamp (Unix epoch) |
| `iat` | number | Issued at timestamp (Unix epoch) |

### Token Expiration

- **Default:** 24 hours from issuance
- **Storage:** Store securely (localStorage, cookies, or memory)

### Sending Token in Requests

**Authorization Header (Recommended):**

```bash
Authorization: Bearer <your_token_here>
```

**Example:**

```bash
curl -X GET http://localhost:3000/api/posts \
  -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
```

---

## Session Management (Cookie-based)

For cookie-based authentication, see the [Sessions API](./SESSIONS.md).

**Key Differences:**

| Feature | JWT Token | Cookie Session |
|---------|-----------|----------------|
| Storage | Client-side (localStorage, etc.) | Server-side + HttpOnly cookie |
| State | Stateless | Stateful |
| Expiration | Fixed per token | Configurable (24h or 30 days) |
| "Remember Me" | Not applicable | Supported |
| CSRF Protection | Not needed | Required |
| Best For | Mobile apps, SPA | Traditional web apps |

---

## Error Codes

| Status Code | Error Type | Description |
|-------------|------------|-------------|
| 200 | OK | Request successful |
| 201 | Created | Resource created successfully |
| 400 | Bad Request | Validation error in request |
| 401 | Unauthorized | Authentication required or failed |
| 404 | Not Found | Resource not found |
| 409 | Conflict | Resource already exists (e.g., username) |
| 500 | Internal Server Error | Server error |

### Error Response Format

All errors follow this format:

```json
{
  "error": "Error message here"
}
```

---

## Security Best Practices

### For Client Applications

1. **Never expose tokens** - Don't log tokens or send them to untrusted domains
2. **Use HTTPS** - Always transmit tokens over encrypted connections
3. **Store securely** - Use secure storage mechanisms
4. **Handle expiration** - Implement token refresh or re-authentication
5. **Validate responses** - Always check response status codes

### For API Integration

1. **Validate inputs** - Never trust client-side validation
2. **Rate limiting** - Implement rate limiting on auth endpoints
3. **Secure headers** - Use appropriate security headers
4. **Log auth attempts** - Monitor for suspicious activity
5. **Strong secrets** - Use strong, randomly generated JWT secrets

### Password Requirements

- **Minimum length:** 8 characters
- **Complexity:** At least one letter and one digit
- **Storage:** Hashed using Argon2 (industry standard)
- **Recommendation:** Encourage users to use unique, strong passwords

---

## Testing Authentication

### Register and Login Flow

```bash
# 1. Register a new user
curl -X POST http://localhost:3000/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testuser",
    "password": "TestPass123!"
  }'

# Save the token from the response
export TOKEN="eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."

# 2. Use the token to access protected endpoints
curl -X GET http://localhost:3000/api/auth/me \
  -H "Authorization: Bearer $TOKEN"

# 3. Create a post (requires POST_CREATE permission)
curl -X POST http://localhost:3000/api/posts \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "title": "My First Post",
    "content": "This is the content of my post."
  }'
```

---

## Integration Examples

### React Example

```jsx
import { useState } from 'react';

function App() {
  const [token, setToken] = useState(localStorage.getItem('token') || null);

  const login = async (username, password) => {
    const response = await fetch('http://localhost:3000/api/auth/login', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ username, password }),
    });
    const data = await response.json();
    setToken(data.token);
    localStorage.setItem('token', data.token);
  };

  const logout = () => {
    setToken(null);
    localStorage.removeItem('token');
  };

  const fetchUser = async () => {
    const response = await fetch('http://localhost:3000/api/auth/me', {
      headers: { 'Authorization': `Bearer ${token}` },
    });
    return await response.json();
  };

  // ... render UI
}
```

### Axios Example

```javascript
import axios from 'axios';

const api = axios.create({
  baseURL: 'http://localhost:3000/api',
});

// Add token to all requests
api.interceptors.request.use((config) => {
  const token = localStorage.getItem('token');
  if (token) {
    config.headers.Authorization = `Bearer ${token}`;
  }
  return config;
});

// Handle 401 errors
api.interceptors.response.use(
  (response) => response,
  (error) => {
    if (error.response?.status === 401) {
      // Redirect to login or refresh token
      localStorage.removeItem('token');
      window.location.href = '/login';
    }
    return Promise.reject(error);
  }
);

// Usage
async function register(username, password) {
  const { data } = await api.post('/auth/register', {
    username,
    password,
  });
  localStorage.setItem('token', data.token);
  return data.user;
}
```

---

## Related Documentation

- [Sessions API](./SESSIONS.md) - Cookie-based authentication
- [Users API](./USERS.md) - User management
- [Posts API](./POSTS.md) - Post management
- [Permissions](../ARCHITECTURE.md#permission-system) - Detailed permission system

---

## Support

For issues or questions regarding authentication:
- Check the [main README](../README.md)
- Review the [Architecture documentation](../ARCHITECTURE.md)
- Open an issue on GitHub