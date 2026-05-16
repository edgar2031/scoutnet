//! Core domain types, configuration, error definitions, and global traits.
//!
//! This crate is imported by every other crate in the workspace.
//! It contains NO business logic — only types, config, errors, and interfaces.

pub mod config;
pub mod error;
pub mod pagination;
pub mod traits;
pub mod types;

pub use error::AppError;
