//! GET /health — liveness probe for load balancers and Fly.io.

use axum::{routing::get, Json, Router};
use serde::Serialize;

use crate::state::AppState;

/// Response body for the health endpoint.
#[derive(Serialize)]
struct HealthResponse {
    /// Always `"ok"` when the process is running.
    status: &'static str,
    /// Crate version from `Cargo.toml`.
    version: &'static str,
}

/// Builds the health check sub-router mounted at `/health`.
pub fn router() -> Router<AppState> {
    Router::new().route("/health", get(handler))
}

/// Returns 200 OK with `{"status":"ok","version":"..."}`.
async fn handler() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        version: env!("CARGO_PKG_VERSION"),
    })
}

#[cfg(test)]
mod tests {
    use axum::{body::Body, http::{Request, StatusCode}};
    use tower::ServiceExt;

    use super::*;

    #[tokio::test]
    async fn health_returns_200() {
        // Router without state — health handler doesn't use state
        let app: Router = Router::new().route("/health", get(handler));

        let response = app
            .oneshot(Request::builder().uri("/health").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
