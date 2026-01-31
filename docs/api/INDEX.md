# Peng Blog API Documentation Index

Welcome to the Peng Blog API documentation. This index provides a comprehensive overview of all available API endpoints and their detailed documentation.

## Table of Contents

- [Quick Start](#quick-start)
- [Authentication](#authentication)
- [API Endpoints Summary](#api-endpoints-summary)
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
curl http://localhost:3000/api/posts

# 4. Publish the post
curl -X POST http://localhost:3000/api/posts/{post_id}/publish \
  -H "Authorization: Bearer $TOKEN"
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

## API Endpoints Summary

### Quick Reference Table

| Resource | Endpoints | Public | Auth Required | Admin Only |
|----------|-----------|--------|---------------|------------|
| **Auth** | 3 | 2 | 1 | 0 |
| **Sessions** | 4 | 3 | 1 | 0 |
| **Users** | 4 | 1 | 3 | 2 |
| **Posts** | 11 | 3 | 8 | 0 |
| **Categories** | 6 | 3 | 0 | 3 |
| **Tags** | 4 | 2 | 0 | 2 |
| **Comments** | 7 | 4 | 3 | 0 |
| **Files** | 5 | 2 | 3 | 0 |
| **Stats** | 5 | 5 | 0 | 0 |
| **Total** | **51** | **25** | **21** | **7** |

---

### Authentication & Sessions

| Endpoint | Method | Auth Required | Description |
|----------|--------|---------------|-------------|
| `POST /auth/register` | Register new user | No | Create account |
| `POST /auth/login` | Login | No | Get JWT token |
| `GET /auth/me` | Get current user | Yes | User info |
| `POST /sessions` | Create session | No | Cookie-based login |
| `DELETE /sessions` | Delete session | No | Cookie logout |
| `GET /sessions/info` | Get session info | Yes | Session details |
| `POST /sessions/github` | GitHub OAuth | No | OAuth callback |

**Documentation:** [Auth API](./AUTH.md) | [Sessions API](./SESSIONS.md)

---

### Posts

| Endpoint | Method | Auth Required | Permission | Description |
|----------|--------|---------------|------------|-------------|
| `GET /posts` | List posts | No* | - | With filters (user, category, tag) |
| `GET /posts/{id}` | Get post | No* | - | Post details |
| `POST /posts` | Create post | Yes | POST_CREATE | New draft post |
| `PUT /posts/{id}` | Update post | Yes | Owner | Edit title/content |
| `DELETE /posts/{id}` | Delete post | Yes | Owner | Remove post |
| `POST /posts/{id}/publish` | Publish | Yes | Owner | Make public |
| `POST /posts/{id}/unpublish` | Unpublish | Yes | Owner | Revert to draft |
| `PUT /posts/{id}/category` | Set category | Yes | Owner | Assign category |
| `GET /posts/{id}/tags` | Get tags | No | - | Post tags |
| `POST /posts/{id}/tags/{tag_id}` | Add tag | Yes | Owner | Associate tag |
| `DELETE /posts/{id}/tags/{tag_id}` | Remove tag | Yes | Owner | Remove tag |

**\*** *Behavior differs for authenticated users (admins see all, owners see their drafts)*

**Documentation:** [Posts API](./POSTS.md)

---

### Users

| Endpoint | Method | Auth Required | Permission | Description |
|----------|--------|---------------|------------|-------------|
| `GET /users` | List users | Yes | USER_MANAGE | All users |
| `GET /users/{id}` | Get user | Yes | Self/Admin | User info |
| `GET /users/{id}/posts` | Get user posts | No | - | Public posts |
| `PATCH /users/{id}/permissions` | Update permissions | Yes | USER_MANAGE | Change perms |

**Documentation:** [Users API](./USERS.md)

---

### Categories

| Endpoint | Method | Auth Required | Permission | Description |
|----------|--------|---------------|------------|-------------|
| `GET /categories` | List categories | No | - | All categories |
| `POST /categories` | Create category | Yes | USER_MANAGE | New category |
| `GET /categories/{id}` | Get category | No | - | Category details |
| `PUT /categories/{id}` | Update category | Yes | USER_MANAGE | Edit category |
| `DELETE /categories/{id}` | Delete category | Yes | USER_MANAGE | Remove category |
| `GET /categories/{id}/children` | Get children | No | - | Subcategories |

**Documentation:** [Categories API](./CATEGORIES.md)

---

### Tags

| Endpoint | Method | Auth Required | Permission | Description |
|----------|--------|---------------|------------|-------------|
| `GET /tags` | List tags | No | - | All tags |
| `POST /tags` | Create tag | Yes | USER_MANAGE | New tag |
| `GET /tags/{id}` | Get tag | No | - | Tag details |
| `DELETE /tags/{id}` | Delete tag | Yes | USER_MANAGE | Remove tag |

**Documentation:** [Tags API](./TAGS.md)

---

### Comments

| Endpoint | Method | Auth Required | Permission | Description |
|----------|--------|---------------|------------|-------------|
| `GET /comments/github/auth` | GitHub OAuth URL | No | - | OAuth start |
| `POST /comments/github` | Create comment (GitHub) | No | - | OAuth comment |
| `GET /comments/posts/{id}` | List comments | No | - | Post comments |
| `POST /comments` | Create comment | Yes | Owner | New comment |
| `GET /comments/{id}` | Get comment | No | - | Comment details |
| `PUT /comments/{id}` | Update comment | Yes | Owner | Edit comment |
| `DELETE /comments/{id}` | Delete comment | Yes | Owner | Remove comment |

**Documentation:** [Comments API](./COMMENTS.md)

---

### Files

| Endpoint | Method | Auth Required | Description |
|----------|--------|---------------|-------------|
| `POST /files` | Upload file | Yes | Upload (multipart) |
| `GET /files` | List files | Yes | User's files |
| `GET /files/{id}` | Get file info | No | File metadata |
| `GET /files/{id}/download` | Download file | No | File content |
| `DELETE /files/{id}` | Delete file | Yes | Owner only |

**File Limits:**
- Max size: 10MB
- Allowed types: JPEG, PNG, GIF, WebP, PDF, TXT, MD

**Documentation:** [Files API](./FILES.md)

---

### Statistics

| Endpoint | Method | Auth Required | Description |
|----------|--------|---------------|-------------|
| `GET /stats/visits` | Get visit stats | No | Global stats |
| `POST /stats/visits` | Record visit | No | Track visit |
| `GET /stats/posts/{id}/views` | Get post views | No | View count |
| `POST /stats/posts/{id}/views` | Record view | No | Increment count |
| `GET /stats/total` | Total stats | No* | All stats |

**\** *Should be admin-only in production*

**Documentation:** [Stats API](./STATS.md)

---

## Common Patterns

### Pagination

Most list endpoints support pagination via query parameters:

```bash
# Get 20 posts (default)
GET /api/posts

# Get 50 posts
GET /api/posts?limit=50

# Get 10 comments
GET /api/comments/posts/{id}?limit=10
```

**Default limits:**
- Posts: 20
- Users: 50
- Comments: 50
- Files: 50

### Filtering

Posts support multiple filter types:

```bash
# By user
GET /api/posts?user_id={uuid}

# By category
GET /api/posts?category_id={uuid}

# By tag
GET /api/posts?tag_id={uuid}

# Combined (AND logic)
GET /api/posts?category_id={uuid}&tag_id={uuid}
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

**Success Response:**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "title": "Post Title",
  ...
}
```

**Error Response:**
```json
{
  "error": "Error message describing what went wrong"
}
```

---

## Error Handling

All API errors follow a consistent format:

### HTTP Status Codes

| Status | Code | Meaning |
|--------|------|---------|
| 200 | OK | Request successful |
| 201 | Created | Resource created |
| 204 | No Content | Deletion successful |
| 400 | Bad Request | Invalid request data |
| 401 | Unauthorized | Authentication required/failed |
| 403 | Forbidden | Insufficient permissions |
| 404 | Not Found | Resource not found |
| 500 | Internal Server Error | Server error |

### Error Response Structure

```json
{
  "error": "Error message here",
  "details": {
    "field": "Additional context"
  }
}
```

### Common Errors

**Authentication Errors:**
```json
// 401 - Missing token
{
  "error": "Missing or invalid authorization token"
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
  "error": "Validation failed: Title cannot be empty"
}
```

**Permission Errors:**
```json
// 403 - Insufficient permissions
{
  "error": "Permission denied: requires permission flag 0x1"
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

## Permission System

Peng Blog uses bit flag-based permissions:

| Permission | Value | Hex | Description |
|------------|-------|-----|-------------|
| `POST_CREATE` | 1 | 0x1 | Create posts |
| `POST_UPDATE` | 2 | 0x2 | Update posts |
| `POST_DELETE` | 4 | 0x4 | Delete posts |
| `POST_PUBLISH` | 8 | 0x8 | Publish posts |
| `USER_MANAGE` | 16 | 0x10 | Manage users, categories, tags |

**Default Permissions:**
- Regular user: `POST_CREATE | POST_UPDATE | POST_PUBLISH` = 11 (0x0B)
- Admin: All permissions = 31 (0x1F)

**Checking Permissions:**
```javascript
// Check if user can create posts
const canCreate = (user.permissions & 0x1) !== 0;

// Check if user is admin
const isAdmin = user.permissions === 0x1F;
```

---

## Rate Limiting

**Status:** Not currently implemented.

**Recommendation:** Configure rate limiting in production via reverse proxy (e.g., Nginx).

**Suggested Limits:**
- Public endpoints: 60 requests/minute
- Auth endpoints: 10 requests/minute
- Authenticated endpoints: 100 requests/minute

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
  async getPosts(limit = 20, userId?: string) {
    const params = new URLSearchParams({ limit: limit.toString() });
    if (userId) params.append('user_id', userId);
    return this.request(`/posts?${params}`);
  }

  async createPost(title: string, content: string) {
    return this.request('/posts', {
      method: 'POST',
      body: JSON.stringify({ title, content }),
    });
  }

  async publishPost(postId: string) {
    return this.request(`/posts/${postId}/publish`, {
      method: 'POST',
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
await api.publishPost(newPost.id);
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
    def get_posts(self, limit: int = 20, user_id: Optional[str] = None) -> Dict[str, Any]:
        params = f"?limit={limit}"
        if user_id:
            params += f"&user_id={user_id}"
        return self._request("GET", f"/posts{params}")

    def create_post(self, title: str, content: str) -> Dict[str, Any]:
        return self._request("POST", "/posts", {
            "title": title,
            "content": content
        })

    def publish_post(self, post_id: str) -> Dict[str, Any]:
        return self._request("POST", f"/posts/{post_id}/publish")

# Usage
api = PengBlogAPI()
api.login("testuser", "SecurePass123!")
posts = api.get_posts()
new_post = api.create_post("Hello", "Content here")
api.publish_post(new_post["id"])
```

---

## Reference

### Related Documentation

- [Main README](../README.md) - Project overview
- [Architecture Guide](../ARCHITECTURE.md) - System architecture
- [Development Guide](../DEVELOPMENT.md) - Development workflow

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

**Last Updated:** 2026-01-30  
**API Version:** v1.0.0  
**Total Endpoints:** 51
