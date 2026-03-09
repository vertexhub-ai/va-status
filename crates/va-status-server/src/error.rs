use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    DbError(#[from] sqlx::Error),
    #[error("Migration error: {0}")]
    MigrationError(#[from] sqlx::migrate::MigrateError),
    #[error("Configuration error: {0}")]
    ConfigError(#[from] config::ConfigError),
    #[error("Not found")]
    NotFound,
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("Internal server error")]
    InternalServerError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::NotFound => (
                StatusCode::NOT_FOUND,
                "The requested resource was not found.".to_string(),
            ),
            AppError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::DbError(e) => {
                tracing::error!("Database error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "A database error occurred.".to_string(),
                )
            }
            AppError::MigrationError(e) => {
                tracing::error!("Migration error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Database migration failed.".to_string(),
                )
            }
            AppError::ConfigError(e) => {
                tracing::error!("Configuration error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Server configuration error.".to_string(),
                )
            }
            AppError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "An unexpected error occurred.".to_string(),
            ),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}
