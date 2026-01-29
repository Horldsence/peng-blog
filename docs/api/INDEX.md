# Peng Blog API Documentation Index

Welcome to the Peng Blog API documentation. This index provides a comprehensive overview of all available API endpoints and their detailed documentation.

## Table of Contents

- [Quick Start](#quick-start)
- [Authentication](#authentication)
- [API Endpoints](#api-endpoints)
- [Common Patterns](#common-patterns)
- [Error Handling](#error-handling)
- [Rate Limiting](#rate-limiting)
- [SDK Examples](#sdk-examples)

---

## Quick Start

### Base URL

```
Development: http://localhost:3000/api
Production: https://yourdomain.com/api
```

### Make Your First Request

```bash
# 1. Register a new user
curl -X POST http://localhost:3000/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testuser",
    "password": "SecurePass123!"
  }'

# Response includes a JWT token - save it!
export TOKEN="your_jwt_token_here"

# 2. Create a post
curl -X POST http://localhost:3000/api/posts \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Hello World",
    "content": "My first blog post!"
  }'

# 3. Get all posts
curl -X GET http://localhost:3000/api/posts
```

---

## Authentication

Peng Blog supports two authentication methods:

### JWT Token (Recommended for API Clients)

```bash
# Include token in Authorization header
Authorization: Bearer <your_jwt_token>
```

### Cookie Sessions (For Web Applications)

```bash
# Cookies are automatically handled by the browser
# Sessions support "Remember Me" (30 days vs 24 hours)
```

**Detailed Documentation:**
- [Authentication API](./AUTH.md) - JWT-based authentication
- [Sessions API](./SESSIONS.md) - Cookie-based sessions

---

## API Endpoints

### Authentication & Sessions

| Endpoint | Method | Auth Required | Description |
|----------|--------|---------------|-------------|
| `/auth/register` | POST | No | Register new user |
| `/auth/login` | POST | No | Login with username/password |
| `/auth/me` | GET | Yes | Get current user info |
| `/sessions` | POST | No | Create session (cookie) |
| `/sessions` | DELETE | No | Delete session (logout) |
| `/sessions/me` | GET | Yes | Get session info |

**Documentation:** [Auth API](./AUTH.md) | [Sessions API](./SESSIONS.md)

---

### Posts

| Endpoint | Method | Auth Required | Description |
|----------|--------|---------------|-------------|
| `/posts` | GET | No | List published posts |
| `/posts` | POST | Yes | Create a new post |
| `/posts/:id` | GET | No | Get post details |
| `/posts/:id` | PUT | Yes | Update a post |
| `/posts/:id` | DELETE | Yes | Delete a post |
| `/posts/:id/publish` | POST | Yes | Publish a post |
| `/posts/:id/unpublish` | POST | Yes | Unpublish a post |
| `/posts?user_id=:id` | GET | No | Get posts by user |

**Permissions:**
- Create: `POST_CREATE` (0x1)
- Update: `POST_UPDATE` (0x2)
- Delete: `POST_DELETE` (0x4) - Admin or owner
- Publish: `POST_PUBLISH` (0x8)

**Documentation:** [Posts API](./POSTS.md)

---

### Users

| Endpoint | Method | Auth Required | Description |
|----------|--------|---------------|-------------|
| `/users` | GET | Admin | List all users |
| `/users/:id` | GET | Yes | Get user info |
| `/users/:id/posts` | GET | No | Get user's posts |
| `/users/:id/permissions` | PATCH | Admin | Update user permissions |

**Permissions:**
- List users: `USER_MANAGE` (0x10)
- Update permissions: `USER_MANAGE` (0x10)

**Documentation:** [Users API](./USERS.md)

---

### Comments

| Endpoint | Method | Auth Required | Description |
|----------|--------|---------------|-------------|
| `/comments` | POST | Yes (optional for GitHub) | Create comment |
| `/comments/github` | POST | No | Create comment (GitHub OAuth) |
| `/comments/github/auth` | GET | No | Get GitHub OAuth URL |
| `/comments/:id` | GET | No | Get comment |
| `/comments/:id` | PUT | Yes | Update comment (owner only) |
| `/comments/:id` | DELETE | Yes | Delete comment (owner only) |
| `/comments/posts/:id` | GET | No | Get comments for a post |

**Documentation:** [Comments API](./COMMENTS.md)

---

### Files

| Endpoint | Method | Auth Required | Description |
|----------|--------|---------------|-------------|
| `/files` | GET | Yes | List user's files |
| `/files` | POST | Yes | Upload file (multipart) |
| `/files/:id` | GET | Yes | Get file metadata |
| `/files/:id/download` | GET | Yes | Download file |
| `/files/:id` | DELETE | Yes | Delete file (owner only) |

**File Limits:**
- Max size: 10MB
- Allowed types: JPEG, PNG, GIF, WebP, PDF, TXT, MD

**Documentation:** [Files API](./FILES.md)

---

### Statistics

| Endpoint | Method | Auth Required | Description |
|----------|--------|---------------|-------------|
| `/stats/visits` | GET | No | Get visitor stats |
| `/stats/visits` | POST | No | Record a visit |
| `/stats/posts/:id/views` | GET | No | Get post view count |
| `/stats/posts/:id/views` | POST | No | Record post view |
| `/stats/total` | GET | Admin | Get total statistics |

**Documentation:** [Stats API](./STATS.md)

---

## Common Patterns

### Pagination

Most list endpoints support pagination via the `limit` query parameter:

```bash
# Get 20 posts (default)
GET /api/posts

# Get 50 posts
GET /api/posts?limit=50

# Get 10 comments
GET /api/comments/posts/:id?limit=10
```

### Authentication Headers

```bash
# For JWT authentication
Authorization: Bearer <jwt_token>

# Cookie authentication is automatic (handled by browser)
```

### Request Body Format

```json
{
  "field1": "value1",
  "field2": "value2"
}
```

### Response Format

```json
{
  "data": { ... },
  "meta": {
    "page": 1,
    "limit": 20,
    "total": 100
  }
}
```

---

## Error Handling

All API errors follow a consistent format:

### Error Response Structure

```json
{
  "error": "Error message describing what went wrong",
  "code": "ERROR_CODE",
  "details": {  // Optional, additional context
    "field": "value"
  }
}
```

### HTTP Status Codes

| Status | Code | Meaning |
|--------|------|---------|
| 200 | OK | Request successful |
| 201 | Created | Resource created |
| 400 | Bad Request | Invalid request data |
| 401 | Unauthorized | Authentication required or failed |
| 403 | Forbidden | Insufficient permissions |
| 404 | Not Found | Resource not found |
| 409 | Conflict | Resource already exists |
| 422 | Unprocessable Entity | Validation error |
| 429 | Too Many Requests | Rate limit exceeded |
| 500 | Internal Server Error | Server error |

### Common Errors

**Authentication Errors:**
```json
// 401 - Missing token
{
  "error": "Missing or invalid authorization token"
}

// 401 - Token expired
{
  "error": "Token has expired"
}

// 401 - Invalid credentials
{
  "error": "Invalid username or password"
}
```

**Validation Errors:**
```json
// 400 - Invalid input
{
  "error": "Validation failed",
  "details": {
    "field": "title",
    "message": "Title cannot be empty"
  }
}
```

**Permission Errors:**
```json
// 403 - Insufficient permissions
{
  "error": "Permission denied: requires permission flag 0x2"
}
```

**Not Found Errors:**
```json
// 404 - Resource not found
{
  "error": "Post not found"
}
```

---

## Rate Limiting

To ensure fair usage and prevent abuse, the API implements rate limiting.

### Default Limits

| Endpoint Type | Limit | Window |
|---------------|-------|--------|
| Public endpoints | 60 requests | 1 minute |
| Auth endpoints | 10 requests | 1 minute |
| Authenticated endpoints | 100 requests | 1 minute |

### Rate Limit Headers

Rate limit information is included in response headers:

```
X-RateLimit-Limit: 100
X-RateLimit-Remaining: 95
X-RateLimit-Reset: 1635789120
```

### Rate Limit Exceeded

When you exceed the limit, you'll receive a 429 status:

```json
{
  "error": "Rate limit exceeded",
  "retry_after": 30
}
```

---

## SDK Examples

### JavaScript/TypeScript

```typescript
class PengBlogAPI {
  private baseURL = 'http://localhost:3000/api';
  private token?: string;

  constructor(token?: string) {
    this.token = token;
  }

  private async request<T>(
    endpoint: string,
    options: RequestInit = {}
  ): Promise<T> {
    const headers: HeadersInit = {
      'Content-Type': 'application/json',
      ...(options.headers || {}),
    };

    if (this.token) {
      headers['Authorization'] = `Bearer ${this.token}`;
    }

    const response = await fetch(`${this.baseURL}${endpoint}`, {
      ...options,
      headers,
    });

    if (!response.ok) {
      const error = await response.json();
      throw new Error(error.error || 'Request failed');
    }

    return response.json();
  }

  // Auth methods
  async register(username: string, password: string) {
    return this.request('/auth/register', {
      method: 'POST',
      body: JSON.stringify({ username, password }),
    });
  }

  async login(username: string, password: string) {
    return this.request('/auth/login', {
      method: 'POST',
      body: JSON.stringify({ username, password }),
    });
  }

  // Post methods
  async getPosts(limit = 20) {
    return this.request(`/posts?limit=${limit}`);
  }

  async createPost(title: string, content: string) {
    return this.request('/posts', {
      method: 'POST',
      body: JSON.stringify({ title, content }),
    });
  }

  // Set token after login
  setToken(token: string) {
    this.token = token;
  }
}

// Usage
const api = new PengBlogAPI();
const loginData = await api.login('testuser', 'SecurePass123!');
api.setToken(loginData.token);

const posts = await api.getPosts();
const newPost = await api.createPost('Hello', 'Content here');
```

### Python

```python
import requests
from typing import Optional, Dict, Any

class PengBlogAPI:
    def __init__(self, base_url: str = "http://localhost:3000/api"):
        self.base_url = base_url
        self.session = requests.Session()
        self.token: Optional[str] = None

    def _request(
        self,
        method: str,
        endpoint: str,
        data: Optional[Dict[str, Any]] = None
    ) -> Dict[str, Any]:
        headers = {"Content-Type": "application/json"}
        if self.token:
            headers["Authorization"] = f"Bearer {self.token}"

        response = self.session.request(
            method,
            f"{self.base_url}{endpoint}",
            json=data,
            headers=headers
        )
        response.raise_for_status()
        return response.json()

    # Auth methods
    def register(self, username: str, password: str) -> Dict[str, Any]:
        return self._request("POST", "/auth/register", {
            "username": username,
            "password": password
        })

    def login(self, username: str, password: str) -> Dict[str, Any]:
        data = self._request("POST", "/auth/login", {
            "username": username,
            "password": password
        })
        self.token = data["token"]
        return data

    # Post methods
    def get_posts(self, limit: int = 20) -> Dict[str, Any]:
        return self._request("GET", f"/posts?limit={limit}")

    def create_post(self, title: str, content: str) -> Dict[str, Any]:
        return self._request("POST", "/posts", {
            "title": title,
            "content": content
        })

# Usage
api = PengBlogAPI()
api.login("testuser", "SecurePass123!")
posts = api.get_posts()
new_post = api.create_post("Hello", "Content here")
```

---

## Reference

### Related Documentation

- [Main README](../README.md) - Project overview
- [Architecture Guide](../ARCHITECTURE.md) - System architecture
- [New Features](../NEW_FEATURES.md) - Feature documentation
- [Refactoring History](../REFACTORING.md) - Code improvements

### API Versioning

Current API version: **v1**

The API follows semantic versioning. Backward-compatible changes will not increment the major version.

### Changelog

See [CHANGELOG.md](../CHANGELOG.md) for API changes and updates.

---

## Support

- **GitHub Issues**: Report bugs and request features
- **Documentation**: Check individual API docs for detailed examples
- **Architecture**: Review [Architecture Guide](../ARCHITECTURE.md) for system design

---

## License

This API is part of the Peng Blog project, licensed under the MIT License.

---

**Last Updated:** 2026-01-27
**API Version:** v1.0.0