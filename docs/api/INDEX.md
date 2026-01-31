# Peng Blog API Documentation Index

Welcome to the Peng Blog API documentation. This index provides a comprehensive overview of all available API endpoints.

**API Version:** v2.0  
**Last Updated:** 2026-01-31

---

## Quick Start

### Base URL

```
Development: http://localhost:3000/api
Production: https://yourdomain.com/api
```

### Response Format

All API responses follow a unified format:

```json
// Success Response (Single Resource)
{
  "code": 200,
  "message": "success",
  "data": { ... }
}

// Success Response (List)
{
  "code": 200,
  "message": "success",
  "data": [ ... ],
  "pagination": {
    "page": 1,
    "per_page": 20,
    "total": 100,
    "total_pages": 5
  }
}

// Error Response
{
  "code": 400,
  "message": "Validation failed",
  "errors": {
    "field": ["error message"]
  }
}
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

# 3. Get all published posts
curl http://localhost:3000/api/posts

# 4. Publish the post (using PATCH)
curl -X PATCH http://localhost:3000/api/posts/{post_id} \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "status": "published"
  }'
```

---

## API Endpoints Summary

### Authentication

| Method | Endpoint | Auth Required | Description |
|--------|----------|---------------|-------------|
| POST | `/auth/register` | No | Register new user |
| POST | `/auth/login` | No | Login with credentials |
| POST | `/auth/logout` | No | Logout (client-side) |
| GET | `/auth/me` | Yes | Get current user info |

### Users

| Method | Endpoint | Auth Required | Permission | Description |
|--------|----------|---------------|------------|-------------|
| GET | `/users` | Yes | USER_MANAGE | List all users |
| GET | `/users/{id}` | Yes | Self/Admin | Get user info |
| PATCH | `/users/{id}` | Yes | Self/Admin | Update user |
| DELETE | `/users/{id}` | Yes | Self/Admin | Delete user (cannot delete last admin) |
| GET | `/users/{id}/posts` | No | - | Get user's posts |

### Posts

| Method | Endpoint | Auth Required | Description |
|--------|----------|---------------|-------------|
| GET | `/posts` | No | List posts with filters |
| GET | `/posts/search` | No | Search posts |
| POST | `/posts` | Yes | Create new post |
| GET | `/posts/{id}` | No | Get post details |
| PUT | `/posts/{id}` | Yes | Full update post |
| PATCH | `/posts/{id}` | Yes | Partial update (status, category, etc.) |
| DELETE | `/posts/{id}` | Yes | Delete post |
| GET | `/posts/{id}/comments` | No | Get post comments |
| POST | `/posts/{id}/comments` | Yes | Add comment to post |
| GET | `/posts/{id}/tags` | No | Get post tags |
| POST | `/posts/{id}/tags` | Yes | Add tag to post |
| DELETE | `/posts/{id}/tags/{tag_id}` | Yes | Remove tag from post |

### Categories

| Method | Endpoint | Auth Required | Permission | Description |
|--------|----------|---------------|------------|-------------|
| GET | `/categories` | No | - | List all categories |
| POST | `/categories` | Yes | USER_MANAGE | Create category |
| GET | `/categories/{id}` | No | - | Get category details |
| GET | `/categories/{id}/posts` | No | - | Get posts in category |
| PATCH | `/categories/{id}` | Yes | USER_MANAGE | Update category |
| DELETE | `/categories/{id}` | Yes | USER_MANAGE | Delete category |

### Tags

| Method | Endpoint | Auth Required | Permission | Description |
|--------|----------|---------------|------------|-------------|
| GET | `/tags` | No | - | List all tags |
| POST | `/tags` | Yes | USER_MANAGE | Create tag |
| GET | `/tags/{id}` | No | - | Get tag details |
| GET | `/tags/{id}/posts` | No | - | Get posts with tag |
| DELETE | `/tags/{id}` | Yes | USER_MANAGE | Delete tag |

### Comments

| Method | Endpoint | Auth Required | Description |
|--------|----------|---------------|-------------|
| GET | `/comments/{id}` | No | Get comment details |
| PATCH | `/comments/{id}` | Yes | Update comment (owner) |
| DELETE | `/comments/{id}` | Yes | Delete comment (owner) |

### Files

| Method | Endpoint | Auth Required | Description |
|--------|----------|---------------|-------------|
| GET | `/files` | Yes | List user's files |
| POST | `/files` | Yes | Upload file |
| GET | `/files/{id}` | No | Get file info |
| GET | `/files/{id}/download` | No | Download file |
| DELETE | `/files/{id}` | Yes | Delete file |

### Sessions (Cookie Auth)

| Method | Endpoint | Auth Required | Description |
|--------|----------|---------------|-------------|
| POST | `/sessions` | No | Create session (cookie login) |
| DELETE | `/sessions` | Yes | Delete session (logout) |
| GET | `/sessions/info` | Yes | Get session info |
| POST | `/sessions/github` | No | GitHub OAuth callback |

### Statistics

| Method | Endpoint | Auth Required | Description |
|--------|----------|---------------|-------------|
| GET | `/stats` | No | Get overall statistics |
| GET | `/stats/visits` | No | Get visit statistics |
| POST | `/stats/visits` | No | Record a visit |
| GET | `/stats/posts/{id}` | No | Get post statistics |
| POST | `/stats/posts/{id}/views` | No | Record a post view |

---

## Common Patterns

### Pagination

All list endpoints support pagination:

```bash
# Default pagination (page 1, 20 items per page)
GET /api/posts

# Custom page and page size
GET /api/posts?page=2&per_page=50

# Response includes pagination info
{
  "code": 200,
  "data": [...],
  "pagination": {
    "page": 2,
    "per_page": 50,
    "total": 100,
    "total_pages": 2
  }
}
```

### Filtering Posts

```bash
# Filter by author
GET /api/posts?author={user_id}

# Filter by category
GET /api/posts?category={category_id}

# Filter by tag
GET /api/posts?tag={tag_id}

# Filter by status (admin/owner only)
GET /api/posts?status=draft
GET /api/posts?status=all

# Combined filters
GET /api/posts?author={user_id}&category={category_id}
```

### Partial Updates (PATCH)

Use PATCH for partial updates:

```bash
# Publish a post
curl -X PATCH /api/posts/{id} \
  -d '{"status": "published"}'

# Unpublish a post
curl -X PATCH /api/posts/{id} \
  -d '{"status": "draft"}'

# Update category
curl -X PATCH /api/posts/{id} \
  -d '{"category_id": "..."}'

# Remove category
curl -X PATCH /api/posts/{id} \
  -d '{"category_id": ""}'

# Combined updates
curl -X PATCH /api/posts/{id} \
  -d '{"title": "New Title", "status": "published"}'
```

### Authentication

Include JWT token in Authorization header:

```bash
Authorization: Bearer <your_jwt_token>
```

---

## HTTP Status Codes

| Code | Meaning | Usage |
|------|---------|-------|
| 200 | OK | Successful GET, PUT, PATCH |
| 201 | Created | Successful POST |
| 204 | No Content | Successful DELETE |
| 400 | Bad Request | Validation error |
| 401 | Unauthorized | Missing or invalid token |
| 403 | Forbidden | Insufficient permissions |
| 404 | Not Found | Resource not found |
| 500 | Internal Server Error | Server error |

---

## Permission System

Peng Blog uses bit flag-based permissions:

| Permission | Value | Description |
|------------|-------|-------------|
| `POST_CREATE` | 1 | Create posts |
| `POST_UPDATE` | 2 | Update posts |
| `POST_DELETE` | 4 | Delete posts |
| `POST_PUBLISH` | 8 | Publish/unpublish posts |
| `USER_MANAGE` | 16 | Manage users, categories, tags |

**Default Permissions:**
- Regular user: `POST_CREATE | POST_UPDATE | POST_PUBLISH` (11)
- Admin: All permissions (31)

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

    const data = await response.json();

    if (!response.ok) {
      throw new Error(data.message || 'Request failed');
    }

    return data;
  }

  // Auth
  async register(username: string, password: string) {
    return this.request('/auth/register', {
      method: 'POST',
      body: JSON.stringify({ username, password }),
    });
  }

  async login(username: string, password: string) {
    const data = await this.request<{ data: { token: string } }>('/auth/login', {
      method: 'POST',
      body: JSON.stringify({ username, password }),
    });
    this.token = data.data.token;
    return data;
  }

  // Posts
  async getPosts(filters?: { author?: string; category?: string; tag?: string }) {
    const params = new URLSearchParams();
    if (filters?.author) params.append('author', filters.author);
    if (filters?.category) params.append('category', filters.category);
    if (filters?.tag) params.append('tag', filters.tag);
    return this.request(`/posts?${params}`);
  }

  async createPost(title: string, content: string) {
    return this.request('/posts', {
      method: 'POST',
      body: JSON.stringify({ title, content }),
    });
  }

  async updatePost(postId: string, updates: {
    title?: string;
    content?: string;
    status?: 'published' | 'draft';
    category_id?: string | null;
  }) {
    return this.request(`/posts/${postId}`, {
      method: 'PATCH',
      body: JSON.stringify(updates),
    });
  }

  setToken(token: string) {
    this.token = token;
  }
}
```

---

## Migration from API v1

If you're migrating from API v1, here are the key changes:

### 1. Publish/Unpublish Changed

**Old:**
```bash
POST /posts/{id}/publish
POST /posts/{id}/unpublish
```

**New:**
```bash
PATCH /posts/{id}
{"status": "published"}

PATCH /posts/{id}
{"status": "draft"}
```

### 2. Add Tag Changed

**Old:**
```bash
POST /posts/{id}/tags/{tag_id}
```

**New:**
```bash
POST /posts/{id}/tags
{"tag_id": "..."}
```

### 3. Response Format Changed

**Old:**
```json
{ "id": "...", "title": "..." }
// or
{ "success": true }
```

**New:**
```json
{
  "code": 200,
  "message": "success",
  "data": { "id": "...", "title": "..." }
}
```

### 4. Query Parameters Changed

**Old:**
```bash
/posts?user_id=xxx&category_id=yyy
```

**New:**
```bash
/posts?author=xxx&category=yyy
```

---

## Support

- **GitHub Issues**: Report bugs and request features
- **Documentation**: Check individual API docs for detailed examples
