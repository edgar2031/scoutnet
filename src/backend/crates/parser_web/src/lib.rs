//! Web-based job board parsers (Upwork RSS, etc.).
//!
//! Outputs [`upwork::RawMessage`] structs to the Redis stream `stream:raw_messages`.

pub mod upwork;
