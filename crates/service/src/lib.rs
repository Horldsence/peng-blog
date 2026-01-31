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

pub mod category;
pub mod comment;
pub mod file;
pub mod post;
pub mod session;
pub mod stats;
pub mod tag;
pub mod user;

pub use category::CategoryService;
pub use comment::CommentService;
pub use file::FileService;
pub use post::PostService;
pub use session::SessionService;
pub use stats::StatsService;
pub use tag::TagService;
pub use user::UserService;

pub use domain::{
    CategoryRepository, CommentRepository, Error, FileRepository, PostRepository, Result,
    SessionRepository, StatsRepository, TagRepository, UserRepository,
};
