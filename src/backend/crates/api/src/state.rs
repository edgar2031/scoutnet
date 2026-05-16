//! Shared application state injected into every Axum handler.

use std::sync::Arc;

use sqlx::PgPool;

use lh_core::config::AppConfig;

use crate::{
    repositories::{SessionRepository, UserRepository},
    services::AuthService,
};

/// Application-wide state cloned into every request handler via Axum's `State` extractor.
///
/// Services are wrapped in [`Arc`] so they are cheap to clone across requests.
#[derive(Clone)]
#[allow(dead_code)]
pub struct AppState {
    /// PostgreSQL connection pool — for raw queries outside services.
    pub db: PgPool,
    /// Redis client — connections checked out per request.
    pub redis: redis::Client,
    /// Immutable application configuration.
    pub cfg: Arc<AppConfig>,
    /// Authentication service: register, login, refresh, logout.
    pub auth: Arc<AuthService>,
}

impl AppState {
    /// Construct [`AppState`] from infrastructure dependencies.
    ///
    /// All services are created here and wrapped in [`Arc`].
    pub fn new(db: PgPool, redis: redis::Client, cfg: AppConfig) -> Self {
        let auth = Arc::new(AuthService::new(
            UserRepository::new(db.clone()),
            SessionRepository::new(db.clone()),
        ));

        AppState { db, redis, cfg: Arc::new(cfg), auth }
    }
}
