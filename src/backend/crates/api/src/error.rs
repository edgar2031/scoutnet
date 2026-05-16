//! API-layer error wrapper that converts AppError into HTTP responses.

use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

use lh_core::AppError;

/// Newtype wrapper so we can implement [`IntoResponse`] for [`AppError`].
///
/// Every route handler returns `Result<T, ApiError>` which Axum automatically
/// converts into the appropriate HTTP status + JSON body via this impl.
pub struct ApiError(pub AppError);

impl From<AppError> for ApiError {
    fn from(e: AppError) -> Self {
        ApiError(e)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let status = StatusCode::from_u16(self.0.status_code())
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        let body = Json(json!({ "error": self.0.to_string() }));
        (status, body).into_response()
    }
}
