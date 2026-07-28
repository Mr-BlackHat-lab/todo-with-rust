use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

pub enum AppError {
    Database(sqlx::Error),
    Unauthorized(String),
    NotFound,
}
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error"),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg.as_str()),
            AppError::NotFound => (StatusCode::NOT_FOUND, "Not found"),
        };
        let body = Json(json!({"error": error_message}));
        (status, body).into_response()
    }
}
impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        AppError::Database(error)
    }
}
