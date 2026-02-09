# CHANGELOG

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

#### Architecture & Setup

- Initialize Rust workspace with 6 crates
  - `blog-bin`: Main entry point
  - `blog-web`: Leptos frontend (WASM)
  - `blog-api`: Axum API endpoints
  - `blog-core`: Business logic layer
  - `blog-domain`: Shared types (PORS)
  - `blog-infrastructure`: SeaORM repositories

#### Technical Stack

- **Backend**: Axum 0.8 (type-safe routing, Tower middleware)
- **Frontend**: Leptos 0.7 (fine-grained reactivity, SSR support)
- **Database**: SeaORM 1.2 + SQLite (async ORM, migrations)
- **Serialization**: serde 1.0 + chrono 0.4
- **Runtime**: Tokio 1.42 (async runtime)

#### Documentation

- `docs/AGENT.md`: Technical decisions, architecture rationale, design principles
- `docs/CHANGELOG.md`: This changelog

### Design Principles

#### Data Structure First

- `blog-domain` crate contains Plain Old Rust Structs (PORS)
- Shared between frontend and backend (zero duplication)
- Eliminates type synchronization issues

#### Eliminate Special Cases

- Unified error handling across all layers
- Trait abstractions eliminate code duplication
- State embedded in data, not in control flow

#### Simplicity Focus

- Single responsibility per crate
- Short, focused functions (<3 levels of indentation)
- Early returns, avoid nesting

#### Pragmatic Choices

- SQLite over PostgreSQL (sufficient for blog scale)
- REST over GraphQL (adequate for blog use case)
- Monolithic over microservices (no need for complexity)
- No distributed caching (premature optimization)

---

## [0.1.0] - 2026-01-27

### Initial Release

#### Features

- Project scaffolding
- Workspace configuration
- Documentation infrastructure

#### Technology Decisions

**Why Axum over Actix-web?**

- More ergonomic, type-safe routing
- 10-20% lower memory footprint
- Actix-web's extreme performance overkill for blogs

**Why Leptos over Yew?**

- Fine-grained reactivity (no Virtual DOM)
- Full SSR support for SEO
- 2.1x better real-world performance
- Modern design, no historical baggage

**Why SeaORM over SQLx?**

- Built-in migrations (SQLx requires manual setup)
- Cleaner CRUD operations for blog use case
- Production-ready for enterprise (2026 consensus)
- Async-first, seamless Axum integration

**Why Rust fullstack over React + TypeScript?**

- Shared types via `blog-domain` crate
- Compile-time type safety
- Zero serialization errors
- Automatic type synchronization

### Architecture Overview

```
peng-blog/
├── Cargo.toml                    # Workspace root
├── crates/
│   ├── blog-bin/                # Entry point
│   ├── blog-web/                # Leptos frontend
│   ├── blog-api/                # Axum API endpoints
│   ├── blog-core/               # Business logic
│   ├── blog-domain/             # Shared types (PORS)
│   └── blog-infrastructure/     # SeaORM repositories
├── migrations/                  # SeaORM migrations
└── docs/                        # Documentation
```

### Dependency Graph (acyclic)

```
blog-bin
  ├─→ blog-api
  │     ├─→ blog-core
  │     │     └─→ blog-domain
  │     └─→ blog-infrastructure
  │           └─→ blog-domain
  └─→ blog-web
        └─→ blog-domain
```

### Data Flow

```
Frontend → API → Core → Infrastructure → SQLite
   ↑                                            ↓
   └──────────── Domain (bidirectional) ──────────┘
```

### License

MIT License (to be added)

---

## Future Roadmap

### [0.2.0] - Planned

- [ ] User authentication (JWT tokens)
- [ ] Password hashing (bcrypt/argon2)
- [ ] Post CRUD operations (Create, Read, Update, Delete)
- [ ] Post publishing workflow
- [ ] Frontend post list view
- [ ] Frontend post detail view

### [0.3.0] - Planned

- [ ] Comment system
- [ ] Tag/Category support
- [ ] Search functionality (SQLite FTS5)
- [ ] Markdown support for post content
- [ ] Syntax highlighting for code blocks

### [0.4.0] - Planned

- [ ] Admin dashboard
- [ ] File uploads (images)
- [ ] Post drafts
- [ ] Scheduled publishing
- [ ] Analytics integration

### [1.0.0] - Planned

- [ ] Production deployment
- [ ] Docker containerization
- [ ] CI/CD pipeline
- [ ] Performance optimization
- [ ] Security audit
- [ ] Comprehensive test coverage
