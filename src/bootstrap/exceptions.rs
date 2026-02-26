use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppException {
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Not Found")]
    NotFound,
    #[error("Validation Error: {0}")]
    ValidationError(String),
    #[error("Internal Error")]
    Internal(Box<dyn std::error::Error + Send + Sync>),
}

impl IntoResponse for AppException {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppException::Unauthorized => (StatusCode::UNAUTHORIZED, self.to_string()),
            AppException::NotFound => (StatusCode::NOT_FOUND, self.to_string()),
            AppException::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppException::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error".to_string()),
        };
        let body = Json(json!({
            "error": {
                "status": status.as_u16(),
                "message": message
            }
        }));
        (status, body).into_response()
    }
}
