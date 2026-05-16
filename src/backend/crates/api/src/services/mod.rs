//! Business-logic services — one per domain, independent of HTTP.

pub mod auth_service;

pub use auth_service::AuthService;
