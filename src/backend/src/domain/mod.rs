//! Domain layer — pure types, value objects, and trait contracts.
//!
//! No framework dependencies. No side effects. All business concepts live here.

pub mod errors;
pub mod lead;
pub mod message;
pub mod pagination;
pub mod traits;
pub mod user;

pub use errors::AppError;
