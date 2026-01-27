# Peng Blog

A modern, full-stack blog built with Rust, featuring a clean architecture and type-safe development across the entire stack.

## 🚀 Features

- **RESTful API** built with Axum - type-safe routing and middleware
- **JWT Authentication** with secure token-based auth
- **Role-Based Access Control** using bit-mask permissions
- **Post Management** - Create, read, update, delete, and publish posts
- **SQLite Database** with SeaORM for type-safe queries
- **Domain-Driven Design** - Clean architecture with separated concerns
- **CORS Support** for frontend integration
- **Request Tracing** with structured logging

## 🛠 Tech Stack

### Backend
- **Axum 0.8** - Ergonomic and modular web framework
- **SeaORM 1.2** - Async ORM for database operations
- **SQLite** - Embedded database (easy to swap for PostgreSQL)
- **Tokio** - Async runtime
- **jsonwebtoken** - JWT token generation and validation
- **tower-http** - HTTP middleware (CORS, tracing)

### Architecture
- **Domain-Driven Design** with clear layer separation
- **Workspace** structure for better code organization
- **Type safety** across the entire stack

## 📁 Project Structure

```
peng-blog/
├── crates/
│   ├── app/              # Entry point and main application
│   │   └── main.rs
│   ├── api/              # Axum API endpoints and HTTP layer
│   │   ├── src/
│   │   │   ├── auth.rs   # Authentication endpoints
│   │   │   ├── post.rs   # Post endpoints
│   │   │   ├── state.rs  # Application state
│   │   │   ├── error.rs  # API error handling
│   │   │   └── middleware/
│   │   │       └── auth.rs # JWT claims extractor
│   ├── domain/           # Shared types and business rules (PORS)
│   │   └── src/
│   │       ├── lib.rs
│   │       └── post.rs   # Post, User, Permission types
│   ├── infrastructure/    # Database operations (SeaORM)
│   │   └── src/
│   │       ├── post.rs   # Post repository implementation
│   │       ├── user.rs   # User repository implementation
│   │       ├── entity/   # SeaORM entity definitions
│   │       └── lib.rs    # Database connection
│   └── web/              # Leptos frontend (planned)
├── migrations/           # SeaORM database migrations
├── sql/                  # Initial schema SQL
├── docs/                 # Documentation (AGENT.md, CHANGELOG.md)
├── static/               # Static files served by the app
└── Cargo.toml            # Workspace configuration
```

## 🏗 Architecture

The project follows Domain-Driven Design principles with clear separation of concerns:

```
Frontend → API → Core → Infrastructure → SQLite
   ↑                                            ↓
   └──────────── Domain (shared types) ──────────┘
```

### Layer Responsibilities

- **Domain (`blog-domain`)**: Plain Old Rust Structs (PORS) - no dependencies, shared between frontend and backend
- **Infrastructure (`blog-infrastructure`)**: SeaORM repository implementations - handles database I/O
- **API (`blog-api`)**: Axum routes and HTTP layer - handles request/response, authentication, validation
- **App (`blog-app`)**: Application entry point - wires everything together

### Key Design Principles

1. **Data Structure First**: Design types first, let logic flow naturally
2. **Eliminate Special Cases**: Uniform error handling and state management
3. **Simplicity Focus**: Short functions, early returns, minimal nesting
4. **Pragmatic Choices**: SQLite over PostgreSQL, REST over GraphQL (sufficient for this use case)

## 🚦 Getting Started

### Prerequisites

- Rust 1.75 or later
- SQLite 3

### Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd peng-blog
```

2. Copy environment file:
```bash
cp .env.example .env
```

3. Edit `.env` with your configuration:
```env
DATABASE_URL=sqlite://blog.db
JWT_SECRET=your-secret-key-here-change-in-production
```

4. Run database migrations:
```bash
# Using SeaORM CLI (if installed)
sea-orm-cli migrate up

# Or run the SQL manually
sqlite3 blog.db < sql/init.sql
```

### Running the Application

```bash
cargo run --release
```

The server will start on `http://localhost:3000`

## 📡 API Endpoints

### Authentication

#### Register
```http
POST /api/auth/register
Content-Type: application/json

{
  "username": "johndoe",
  "password": "securepassword123"
}
```

#### Login
```http
POST /api/auth/login
Content-Type: application/json

{
  "username": "johndoe",
  "password": "securepassword123"
}
```

Returns:
```json
{
  "token": "eyJhbGciOiJIUzI1NiIs...",
  "user": {
    "id": "uuid",
    "username": "johndoe",
    "permissions": 3
  }
}
```

#### Get Current User
```http
GET /api/auth/me
Authorization: Bearer <token>
```

### Posts

#### List Published Posts
```http
GET /api/posts?limit=20
```

#### Get Single Post
```http
GET /api/posts/:id
```

#### Create Post (Requires Auth)
```http
POST /api/posts
Authorization: Bearer <token>
Content-Type: application/json

{
  "title": "My First Post",
  "content": "This is the content of my post."
}
```

#### Update Post (Requires Auth)
```http
PUT /api/posts/:id
Authorization: Bearer <token>
Content-Type: application/json

{
  "title": "Updated Title",
  "content": "Updated content",
  "published_at": "2024-01-27T12:00:00Z"
}
```

#### Publish Post (Requires Auth)
```http
POST /api/posts/:id/publish
Authorization: Bearer <token>
```

#### Unpublish Post (Requires Auth)
```http
POST /api/posts/:id/unpublish
Authorization: Bearer <token>
```

#### Delete Post (Requires Auth)
```http
DELETE /api/posts/:id
Authorization: Bearer <token>
```

## 🔑 Permissions System

Permissions are implemented as bit-masks for simplicity and efficiency:

| Permission | Value | Description |
|------------|-------|-------------|
| POST_CREATE | 1 | Create new posts |
| POST_UPDATE | 2 | Update existing posts |
| POST_DELETE | 4 | Delete posts |
| POST_PUBLISH | 8 | Publish/unpublish posts |
| USER_MANAGE | 16 | Manage users (admin only) |

### Default Permissions

- **First User**: All permissions (admin)
- **Regular Users**: POST_CREATE | POST_UPDATE | POST_PUBLISH (can create, update, and publish their own posts)

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run tests for a specific crate
cargo test -p blog-api
```

## 📦 Building for Production

```bash
cargo build --release
```

The binary will be available at `target/release/blog-bin`

## 🔧 Configuration

Configuration is done via environment variables:

- `DATABASE_URL`: SQLite database file path
- `JWT_SECRET`: Secret key for JWT token signing (CHANGE IN PRODUCTION!)
- `RUST_LOG`: Logging level (e.g., `debug`, `info`, `warn`)

## 🚧 Future Enhancements

See `docs/CHANGELOG.md` for planned features:

- [ ] Leptos frontend with SSR
- [ ] Comment system
- [ ] Tag/Category support
- [ ] Full-text search (SQLite FTS5)
- [ ] Markdown support with syntax highlighting
- [ ] Admin dashboard
- [ ] File uploads (images)
- [ ] Post drafts and scheduled publishing

## 📝 Development Notes

### Code Style

- Follow the project's commitment to simplicity and clarity
- Prefer explicit code over clever one-liners
- Keep functions short and focused
- Use early returns to avoid nesting
- Eliminate special cases through good data structure design

### Adding New Features

1. Define types in `domain/` crate
2. Implement repository in `infrastructure/` crate
3. Create API endpoints in `api/` crate
4. Wire everything together

### Database Migrations

```bash
# Create a new migration
sea-orm-cli migrate generate <migration_name>

# Run migrations
sea-orm-cli migrate up

# Rollback last migration
sea-orm-cli migrate down
```

## 📄 License

MIT License - See LICENSE file for details

## 🙏 Acknowledgments

- **Linus Torvalds** for inspiring "Good Taste" in code design
- **Axum Team** for an excellent web framework
- **SeaORM Team** for a productive ORM
- **Leptos Team** for bringing Rust to the frontend

## 📚 Additional Resources

- [Axum Documentation](https://docs.rs/axum/latest/axum/)
- [Leptos Documentation](https://book.leptos.dev/)
- [SeaORM Documentation](https://www.sea-ql.org/SeaORM/docs/)
- [docs/AGENT.md](./docs/AGENT.md) - Technical decisions and architecture rationale
- [docs/CHANGELOG.md](./docs/CHANGELOG.md) - Project changelog and roadmap

---

Built with ❤️ using Rust