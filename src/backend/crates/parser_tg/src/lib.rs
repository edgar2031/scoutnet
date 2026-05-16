//! Telegram MTProto parser — monitors channels and pushes raw messages to Redis.
//!
//! Each Telegram account runs as an independent [`account::AccountWorker`].
//! Messages are written to the Redis stream `stream:raw_messages` for the
//! Enricher subsystem to consume.

pub mod account;
pub mod channel_sync;
pub mod flood_wait;
pub mod proxy;
pub mod rate_limiter;
pub mod session;
pub mod stream_writer;
