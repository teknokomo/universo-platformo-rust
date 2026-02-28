//! Application error types and HTTP response conversions.

use actix_web::HttpResponse;
use serde_json::json;

/// Application-level errors
#[derive(Debug)]
pub enum AppError {
    /// Authentication failure (wrong credentials, expired token, etc.)
    Unauthorized(String),
    /// Supabase API communication error
    SupabaseError(String),
    /// Request validation error
    BadRequest(String),
    /// Internal server error
    Internal(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            AppError::SupabaseError(msg) => write!(f, "Supabase error: {}", msg),
            AppError::BadRequest(msg) => write!(f, "Bad request: {}", msg),
            AppError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl actix_web::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::Unauthorized(msg) => {
                HttpResponse::Unauthorized().json(json!({ "error": msg }))
            }
            AppError::SupabaseError(msg) => {
                log::error!("Supabase error: {}", msg);
                HttpResponse::BadGateway()
                    .json(json!({ "error": "Authentication service unavailable" }))
            }
            AppError::BadRequest(msg) => HttpResponse::BadRequest().json(json!({ "error": msg })),
            AppError::Internal(msg) => {
                log::error!("Internal error: {}", msg);
                HttpResponse::InternalServerError()
                    .json(json!({ "error": "Internal server error" }))
            }
        }
    }
}
