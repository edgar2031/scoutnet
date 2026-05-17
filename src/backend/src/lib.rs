//! ScoutNet backend — monolith library root.
//!
//! Re-exports the six top-level modules that compose the application.
//! External binaries (`main`, `seed`) import from here.

pub mod api;
pub mod application;
pub mod config;
pub mod domain;
pub mod infrastructure;
pub mod jobs;
