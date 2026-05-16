//! Global trait interfaces used across all crates.

pub mod cacheable;
pub mod repository;
pub mod service;

pub use cacheable::Cacheable;
pub use repository::{Crud, Repository};
pub use service::Service;
