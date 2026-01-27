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
//! ```
//! Core (business logic)
//!   ↓ depends on
//! Domain (types)
//!   ↓ implemented by
//! Infrastructure (data access)
//! ```

pub mod post;
pub mod user;
pub mod repository;

pub use repository::{PostRepository, UserRepository};
pub use post::PostService;
pub use user::UserService;

pub use domain::{Error, Result};