//! # Core - Business Logic Layer
//!
//! This crate contains the business logic of the application.
//! It defines the interfaces (traits) for repositories and implements
//! the business rules.
//!
//! ## Responsibilities
//! - Define repository traits (interfaces)
//! - Implement business logic that orchestrates repository calls
//! - Validate business rules
//! - No I/O operations (testable in isolation)
//!
//! ## Architecture
//!
//! The architecture follows a layered approach:
//! - Core (business logic) depends on Domain (types)
//! - Domain (types) is implemented by Infrastructure (data access)

pub mod post;
pub mod user;
pub mod category;
pub mod tag;
pub mod repository;
pub mod session;
pub mod file;
pub mod comment;
pub mod stats;

pub use repository::{PostRepository, UserRepository, CategoryRepository, TagRepository, SessionRepository, FileRepository, CommentRepository, StatsRepository};
pub use post::PostService;
pub use user::UserService;
pub use category::CategoryService;
pub use tag::TagService;
pub use session::SessionService;
pub use file::FileService;
pub use comment::CommentService;
pub use stats::StatsService;

pub use domain::{Error, Result};