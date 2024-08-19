use crate::inventory::services::ServiceError;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;

pub mod person;

impl IntoResponse for ServiceError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ServiceError::NotFound(e) => (StatusCode::NOT_FOUND, e),
            ServiceError::InvalidUuid(e) => (StatusCode::BAD_REQUEST, e),
            ServiceError::InputValidationError(e) => (StatusCode::BAD_REQUEST, e),
            ServiceError::UniqueViolation(e) => (StatusCode::CONFLICT, e),
            ServiceError::UnexpectedError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e),
            ServiceError::Other(e) => (StatusCode::INTERNAL_SERVER_ERROR, e),
        };
        let body = Json(json!({
            "status": status.as_u16(),
            "error": error_message,
        }));
        (status, body).into_response()
    }
}
