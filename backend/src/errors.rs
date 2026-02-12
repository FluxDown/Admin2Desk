use actix_web::{HttpResponse, ResponseError};

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Authentication failed: {0}")]
    Unauthorized(String),

    #[error("Forbidden")]
    Forbidden,

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("SSH error: {0}")]
    Ssh(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let (status, message) = match self {
            AppError::Unauthorized(msg) => (actix_web::http::StatusCode::UNAUTHORIZED, msg.clone()),
            AppError::Forbidden => (
                actix_web::http::StatusCode::FORBIDDEN,
                "Access denied".to_string(),
            ),
            AppError::NotFound(msg) => (actix_web::http::StatusCode::NOT_FOUND, msg.clone()),
            AppError::Validation(msg) => (actix_web::http::StatusCode::BAD_REQUEST, msg.clone()),
            AppError::Database(e) => {
                log::error!("Database error: {}", e);
                (
                    actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
            AppError::Ssh(msg) => (actix_web::http::StatusCode::BAD_GATEWAY, msg.clone()),
            AppError::Internal(msg) => {
                log::error!("Internal error: {}", msg);
                (
                    actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
        };

        HttpResponse::build(status).json(serde_json::json!({ "error": message }))
    }
}
