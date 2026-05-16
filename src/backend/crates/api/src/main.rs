//! LEAD.HUNTER API server entry point.
//!
//! Loads config, connects to Postgres and Redis, builds the Axum router,
//! and starts listening on `0.0.0.0:$PORT` (default 8080).

use std::net::SocketAddr;

use axum::Router;
use sqlx::postgres::PgPoolOptions;
use tracing::info;
use tracing_subscriber::EnvFilter;

use lh_core::config::AppConfig;

mod auth;
mod copilot;
mod error;
mod repositories;
mod routes;
mod services;
mod state;

use state::AppState;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let cfg = AppConfig::from_env().expect("Invalid configuration");

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new(&cfg.log_level))
        .init();

    let db = PgPoolOptions::new()
        .max_connections(20)
        .connect(&cfg.database_url)
        .await
        .expect("Failed to connect to Postgres");

    let redis = redis::Client::open(cfg.redis_url.clone())
        .expect("Invalid Redis URL");

    let state = AppState::new(db, redis, cfg);

    let app = Router::new()
        .merge(routes::health::router())
        .merge(routes::auth::router())
        .merge(routes::ai_providers::router())
        .merge(routes::profile::router())
        .merge(routes::feed::router())
        .merge(routes::matches::router())
        .merge(routes::copilot::router())
        .merge(routes::webhooks::router())
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    info!("Listening on {addr}");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
