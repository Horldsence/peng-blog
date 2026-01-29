# Architecture Documentation

## Overview

Peng Blog follows a clean, layered architecture based on Domain-Driven Design (DDD) principles. The system is organized into distinct crates, each with a single, well-defined responsibility.

### Core Philosophy

1. **Data Structure First** - Good data structures eliminate special cases
2. **Eliminate Special Cases** - Uniform error handling and state management
3. **Simplicity Focus** - Short functions, early returns, minimal nesting
4. **Pragmatic Choices** - Clear, explicit code over clever one-liners

---

## Crate Structure

### 1. Domain (`crates/domain`)

**Purpose**: Defines the core data structures and business rules. This is the "Plain Old Rust Structs" (PORS) layer.

**Responsibilities**:
- Define all domain types (Post, User, etc.)
- Define domain-specific errors
- Define permission constants and helpers
- No dependencies on other crates
- Shared between frontend and backend (future)

**Key Types**:
```rust
- Post: Blog post entity
- User: User entity with permissions
- CreatePost, UpdatePost: DTOs for post operations
- RegisterRequest, LoginRequest, LoginResponse: Auth DTOs
- Error: Domain errors (NotFound, Validation, Internal)
```

**Dependencies**: None (only serde, chrono, uuid for serialization)

---

### 2. Core (`crates/core`)

**Purpose**: Implements business logic and defines repository interfaces. This is the pure logic layer with no I/O.

**Responsibilities**:
- Define repository traits (interfaces)
- Implement business rules and validation
- Orchestrate repository operations
- Enforce permission and ownership checks
- Testable without database

**Key Services**:
```rust
- PostService<R: PostRepository>: Post business logic
- UserService<R: UserRepository>: User business logic
```

**Key Repository Traits**:
```rust
- PostRepository: Interface for post data access
- UserRepository: Interface for user data access
```

**Dependencies**: `domain`

**Design Pattern**: Dependency Injection - services take generic repository types

---

### 3. Infrastructure (`crates/infrastructure`)

**Purpose**: Implements repository interfaces and handles all database operations using SeaORM.

**Responsibilities**:
- Implement repository traits from `core`
- Handle database connections and migrations
- Perform CRUD operations
- Convert between domain types and SeaORM entities
- Handle password hashing (argon2)

**Key Implementations**:
```rust
- PostRepositoryImpl: SeaORM-based post repository
- UserRepositoryImpl: SeaORM-based user repository
- establish_connection(): Database connection helper
```

**Dependencies**: `domain`, `core`, sea-orm, argon2

---

### 4. API (`crates/api`)

**Purpose**: Implements HTTP endpoints using Axum framework. This is the presentation layer.

**Responsibilities**:
- Define API routes
- Handle HTTP request/response
- Extract and validate request data
- Call business logic services
- Handle authentication via JWT
- Return appropriate HTTP status codes

**Key Modules**:
```rust
- auth.rs: Registration, login, and token validation
- post.rs: Post CRUD endpoints
- middleware/auth.rs: JWT Claims extractor
- error.rs: API error handling
- state.rs: Application state
```

**Dependencies**: `domain`, `core`, `infrastructure`, axum, jsonwebtoken

---

### 5. App (`crates/app`)

**Purpose**: Application entry point. Wires everything together and starts the server.

**Responsibilities**:
- Initialize logging
- Load environment variables
- Establish database connection
- Create repository instances
- Create service instances
- Configure Axum router
- Start HTTP server

**Dependencies**: `api`, `infrastructure`, `domain`

---

## Dependency Graph

```
┌─────────────────────────────────────────────────────────────┐
│                         App                              │
│  (Entry point, wiring, server startup)                     │
└──────────────┬────────────────────────────────────────────┘
               │
               ├───→ API (HTTP layer)
               │         │
               │         ├───→ Core (business logic)
               │         │         │
               │         │         └───→ Domain (types)
               │         │
               │         └───→ Infrastructure (data access)
               │                   │
               │                   └───→ Domain (types)
               │
               └───→ Infrastructure (repositories)
                         │
                         └───→ Domain (types)
```

**Key Points**:
- `Domain` has no dependencies (leaf node)
- `Core` depends only on `Domain`
- `Infrastructure` depends on `Domain` and `Core` (implements traits)
- `API` depends on all layers above
- All dependencies flow downward - no circular dependencies

---

## Data Flow

### 1. Creating a Post

```
Client Request
    ↓
API Layer (POST /api/posts)
    ↓ Extract JWT Claims
    ↓ Validate input
    ↓
Core Layer (PostService::create)
    ↓ Validate business rules
    ↓
Infrastructure Layer (PostRepository::create_post)
    ↓ Insert to SQLite
    ↓
Return Post to API
    ↓ Return HTTP 201 with Post JSON
```

### 2. Authenticating a User

```
Client Request (POST /api/auth/login)
    ↓
API Layer (auth::login)
    ↓ Validate credentials
    ↓
Core Layer (UserService::login)
    ↓ Validate business rules
    ↓
Infrastructure Layer (UserRepository::verify_credentials)
    ↓ Query SQLite
    ↓ Verify password hash
    ↓ Return User or None
    ↓ Generate JWT token
    ↓
Return LoginResponse to API
    ↓ Return HTTP 200 with LoginResponse JSON
```

---

## Design Patterns

### 1. Repository Pattern

**Definition**: Abstracts data access logic behind interfaces.

**Implementation**:
- Repository traits defined in `core` (interfaces)
- Concrete implementations in `infrastructure` (SQLite via SeaORM)
- Services depend on traits, not implementations

**Benefits**:
- Testable (mock repositories for unit tests)
- Swappable (change SQLite → PostgreSQL without changing core)
- Clear separation of concerns

**Example**:
```rust
// In core/src/repository.rs
#[async_trait]
pub trait PostRepository: Send + Sync {
    async fn create_post(&self, user_id: Uuid, title: String, content: String) -> Result<Post>;
    async fn get_post(&self, id: Uuid) -> Result<Post>;
    // ...
}

// In infrastructure/src/post.rs
#[async_trait]
impl PostRepository for PostRepositoryImpl {
    async fn create_post(&self, user_id: Uuid, title: String, content: String) -> Result<Post> {
        // SeaORM implementation
    }
    // ...
}
```

---

### 2. Dependency Injection

**Definition**: Services receive dependencies through constructor, not creating them internally.

**Implementation**:
- Services are generic over repository types
- Repositories passed to service constructors
- Makes testing easy by passing mock repositories

**Benefits**:
- Testable (inject mocks)
- Flexible (change implementations without changing code)
- Explicit dependencies (visible in constructor)

**Example**:
```rust
// Generic service
pub struct PostService<R: PostRepository> {
    repo: Arc<R>,
}

// Inject concrete implementation
let post_repo = PostRepositoryImpl::new(db.clone());
let post_service = PostService::new(post_repo);
```

---

### 3. Service Layer Pattern

**Definition**: Encapsulates business logic in dedicated services.

**Implementation**:
- One service per domain entity (PostService, UserService)
- Services orchestrate repository calls
- Services enforce business rules and validation

**Benefits**:
- Clear separation of business logic from data access
- Reusable logic across multiple endpoints
- Easy to test (unit tests without database)

**Example**:
```rust
impl<R: PostRepository> PostService<R> {
    pub async fn create(&self, user_id: Uuid, title: String, content: String) -> Result<Post> {
        self.validate_title(&title)?;
        self.validate_content(&content)?;
        self.repo.create_post(user_id, title, content).await
    }
}
```

---

### 4. DTO (Data Transfer Object) Pattern

**Definition**: Separate objects for API input/output from domain entities.

**Implementation**:
- Domain types (Post, User) for internal use
- DTOs (CreatePost, UpdatePost, LoginRequest) for API
- Conversion happens in API layer

**Benefits**:
- Separate API concerns from domain logic
- Different validation rules for API vs domain
- Hide sensitive data (password hashes) from API responses

**Example**:
```rust
// Domain type
pub struct Post {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub content: String,
    pub published_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

// DTO for creating post
pub struct CreatePost {
    pub title: String,
    pub content: String,
}
```

---

## Permission System

### Design Rationale

Bit-mask based permissions for simplicity and performance:
- Fast to check (bitwise AND)
- Easy to combine (bitwise OR)
- No complex RBAC infrastructure needed
- Sufficient for blog use case

### Permission Constants

```rust
pub const POST_CREATE: u64   = 1 << 0;  // 1
pub const POST_UPDATE: u64   = 1 << 1;  // 2
pub const POST_DELETE: u64   = 1 << 2;  // 4
pub const POST_PUBLISH: u64  = 1 << 3;  // 8
pub const USER_MANAGE: u64   = 1 << 4;  // 16 (admin only)
```

### Permission Checking

```rust
// Check if user has permission
pub fn has_permission(&self, permission: u64) -> bool {
    (self.permissions & permission) != 0
}

// Use in business logic
if (user.permissions & POST_UPDATE) == 0 {
    return Err(Error::Validation("Insufficient permissions".to_string()));
}
```

### Default Permissions

```rust
// Regular users: can create, update, publish their own posts
pub const DEFAULT_USER_PERMISSIONS: u64 = POST_CREATE | POST_UPDATE | POST_PUBLISH;

// Admins: full permissions including user management
pub const ADMIN_PERMISSIONS: u64 = POST_CREATE | POST_UPDATE | POST_DELETE | POST_PUBLISH | USER_MANAGE;
```

---

## Error Handling

### Error Hierarchy

```rust
// Domain errors (core errors)
pub enum Error {
    NotFound(String),
    Validation(String),
    Internal(String),
}

// API errors wrap domain errors with HTTP context
pub enum ApiError {
    Domain(DomainError),
    Internal(String),
    Unauthorized(String),
    // ...
}
```

### Error Propagation Flow

```
Infrastructure (database error)
    ↓ Convert to Domain::Internal
    ↓
Core (business logic error)
    ↓ Return Domain::Error
    ↓
API (converts to ApiError)
    ↓ Converts to HTTP status code
    ↓ Returns JSON error response
```

### HTTP Status Mapping

| Domain Error | API Error | HTTP Status |
|--------------|-----------|--------------|
| NotFound | Domain(NotFound) | 404 Not Found |
| Validation | Domain(Validation) | 400 Bad Request |
| Internal | Domain/Internal | 500 Internal Server Error |
| Auth failed | Unauthorized | 401 Unauthorized |

---

## Testing Strategy

### 1. Unit Tests (Core Layer)

**Scope**: Business logic without database

**Tools**: `mockall` for mocking repositories

**Example**:
```rust
#[tokio::test]
async fn test_create_post_validates_empty_title() {
    let mut mock_repo = MockPostRepo::new();
    let service = PostService::new(mock_repo);
    
    let result = service.create(user_id, "".to_string(), "content".to_string()).await;
    
    assert!(result.is_err());
}
```

**Benefits**:
- Fast (no database)
- Test business rules in isolation
- Test edge cases easily

---

### 2. Integration Tests (API Layer)

**Scope**: End-to-end with real database

**Tools**: `axum::Router` for testing, SQLite in-memory

**Example**:
```rust
#[tokio::test]
async fn test_create_post_endpoint() {
    let app = create_test_app().await;
    let response = app
        .oneshot(Request::builder()
            .uri("/api/posts")
            .body(json_body)
        )
        .await;
    
    assert_eq!(response.status(), StatusCode::CREATED);
}
```

**Benefits**:
- Test entire request flow
- Validate HTTP layer
- Catch integration issues

---

### 3. Repository Tests (Infrastructure Layer)

**Scope**: Database operations

**Tools**: SQLite test database, fixtures

**Example**:
```rust
#[tokio::test]
async fn test_repository_create_post() {
    let db = setup_test_db().await;
    let repo = PostRepositoryImpl::new(db);
    
    let post = repo.create_post(user_id, "Title".to_string(), "Content".to_string()).await.unwrap();
    
    assert!(post.id != Uuid::nil());
}
```

**Benefits**:
- Test database operations
- Validate SQL queries
- Ensure conversions work

---

## Adding New Features

### Step-by-Step Guide

#### 1. Define Domain Types

```rust
// crates/domain/src/comment.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub id: Uuid,
    pub post_id: Uuid,
    pub user_id: Uuid,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

pub struct CreateComment {
    pub post_id: Uuid,
    pub content: String,
}
```

#### 2. Define Repository Interface

```rust
// crates/core/src/repository.rs
#[async_trait]
pub trait CommentRepository: Send + Sync {
    async fn create_comment(&self, user_id: Uuid, post_id: Uuid, content: String) -> Result<Comment>;
    async fn get_comments_by_post(&self, post_id: Uuid, limit: u64) -> Result<Vec<Comment>>;
    async fn delete_comment(&self, id: Uuid) -> Result<()>;
}
```

#### 3. Implement Business Logic

```rust
// crates/core/src/comment.rs
pub struct CommentService<R: CommentRepository> {
    repo: Arc<R>,
}

impl<R: CommentRepository> CommentService<R> {
    pub async fn create(&self, user_id: Uuid, post_id: Uuid, content: String) -> Result<Comment> {
        self.validate_content(&content)?;
        self.repo.create_comment(user_id, post_id, content).await
    }
    
    fn validate_content(&self, content: &str) -> Result<()> {
        if content.trim().is_empty() {
            return Err(Error::Validation("Content cannot be empty".to_string()));
        }
        if content.len() > 1000 {
            return Err(Error::Validation("Content too long (max 1000 characters)".to_string()));
        }
        Ok(())
    }
}
```

#### 4. Implement Repository

```rust
// crates/infrastructure/src/comment.rs
#[async_trait]
impl core::repository::CommentRepository for CommentRepositoryImpl {
    async fn create_comment(&self, user_id: Uuid, post_id: Uuid, content: String) -> Result<Comment> {
        let comment = Comment::new(user_id, post_id, content);
        let entity = Self::comment_to_entity(&comment);
        let active_model = Self::entity_to_active_model(entity);
        
        active_model.insert(self.db.as_ref()).await?;
        Ok(comment)
    }
    // ...
}
```

#### 5. Create API Endpoints

```rust
// crates/api/src/comment.rs
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/posts/:id/comments", axum::routing::get(list_comments))
        .route("/posts/:id/comments", axum::routing::post(create_comment))
        .route("/comments/:id", axum::routing::delete(delete_comment))
}

async fn create_comment(
    State(state): State<AppState>,
    user: Claims,
    Path(post_id): Path<Uuid>,
    Json(input): Json<CreateComment>,
) -> ApiResult<impl IntoResponse> {
    let comment = state
        .comment_service
        .create(user.user_id, post_id, input.content)
        .await
        .map_err(|e| ApiError::Domain(e))?;
    
    Ok((StatusCode::CREATED, Json(comment)))
}
```

#### 6. Wire in App

```rust
// crates/app/src/main.rs
let comment_repo = Arc::new(CommentRepositoryImpl::new(db.clone()));
let comment_service = Arc::new(CommentService::new(comment_repo));

let state = AppState::new(
    post_repo,
    user_repo,
    comment_service,
    auth_state,
);
```

---

## Security Considerations

### 1. Authentication

- JWT tokens for stateless authentication
- Tokens expire after 24 hours
- Tokens contain user ID, username, and permissions
- Passwords hashed with argon2 (memory-hard KDF)

### 2. Authorization

- Bit-mask permissions checked in business logic layer
- Ownership checks for user-owned resources
- Admin bypasses ownership checks
- Cannot remove own admin privileges

### 3. Input Validation

- Validation at API layer (format, length)
- Validation at business logic layer (business rules)
- Prevents injection attacks
- Clear error messages

### 4. SQL Injection

- SeaORM uses parameterized queries
- No raw SQL in application code
- Automatic type conversion

---

## Performance Considerations

### 1. Database

- SQLite with WAL mode for better concurrency
- Indexes on frequently queried fields
- Connection pooling via Arc<DatabaseConnection>

### 2. Caching

- Currently no caching (sufficient for blog scale)
- Can add Redis for hot posts if needed

### 3. Async

- All I/O operations are async (await)
- Tokio runtime handles concurrency
- Non-blocking database queries

---

## Future Enhancements

### 1. Leptos Frontend

- `crates/web` crate with Leptos components
- Shared domain types via `crates/domain`
- SSR for SEO and performance
- Real-time updates via WebSocket

### 2. Advanced Features

- Comments system
- Tag/Category support
- Full-text search (SQLite FTS5)
- Markdown rendering with syntax highlighting
- File uploads (images)
- Post drafts and scheduled publishing
- Analytics and metrics

### 3. Scalability

- Switch to PostgreSQL for higher concurrency
- Add Redis caching layer
- Implement rate limiting
- Add CDN for static assets

---

## Conclusion

This architecture provides:

✅ **Clear separation of concerns** - Each crate has a single responsibility
✅ **Testability** - Business logic testable without database
✅ **Maintainability** - Changes isolated to specific layers
✅ **Scalability** - Easy to add features or swap implementations
✅ **Type safety** - Compile-time checks across entire stack
✅ **Zero duplication** - Shared domain types across layers

The design follows Linus Torvalds' philosophy: good data structures and elimination of special cases lead to simple, elegant code.