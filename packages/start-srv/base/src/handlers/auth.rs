//! Authentication handlers for login, logout, and session check.
//!
//! These handlers communicate with Supabase Auth API and manage
//! sessions using actix-session encrypted cookies (CookieSessionStore).

use actix_session::Session;
use actix_web::{web, HttpResponse};
use serde_json::json;

use crate::error::AppError;
use crate::models::{AuthUser, LoginRequest};
use crate::supabase::SupabaseClient;

use super::{SESSION_ACCESS_TOKEN, SESSION_USER_EMAIL, SESSION_USER_ID};

/// POST /api/v1/auth/login
///
/// Accepts email/password credentials, authenticates against Supabase,
/// and stores the session token in an encrypted cookie session.
pub async fn login(
    session: Session,
    supabase: web::Data<SupabaseClient>,
    body: web::Json<LoginRequest>,
) -> Result<HttpResponse, AppError> {
    // Server-side input validation.
    // Email is trimmed as leading/trailing whitespace is always unintentional.
    // Password is checked as-is because leading/trailing spaces may be intentional.
    let email = body.email.trim().to_string();
    if email.is_empty() || body.password.is_empty() {
        return Err(AppError::BadRequest(
            "Email and password are required".to_string(),
        ));
    }

    let auth_response = supabase
        .sign_in(&email, &body.password)
        .await
        .map_err(|e| {
            // Map Supabase errors to appropriate HTTP errors
            if e.contains("Invalid login credentials") || e.contains("invalid_grant") {
                AppError::Unauthorized("Invalid email or password".to_string())
            } else {
                AppError::SupabaseError(e)
            }
        })?;

    let user_email = auth_response
        .user
        .email
        .clone()
        .unwrap_or_else(|| email.clone());

    // Store session data in cookie
    session
        .insert(SESSION_ACCESS_TOKEN, &auth_response.access_token)
        .map_err(|e| AppError::Internal(format!("Session insert error: {}", e)))?;
    session
        .insert(SESSION_USER_ID, &auth_response.user.id)
        .map_err(|e| AppError::Internal(format!("Session insert error: {}", e)))?;
    session
        .insert(SESSION_USER_EMAIL, &user_email)
        .map_err(|e| AppError::Internal(format!("Session insert error: {}", e)))?;

    log::info!("User logged in: {}", user_email);

    Ok(HttpResponse::Ok().json(json!({
        "message": "Login successful",
        "user": {
            "id": auth_response.user.id,
            "email": user_email,
        }
    })))
}

/// POST /api/v1/auth/logout
///
/// Invalidates the current session and revokes the Supabase token.
pub async fn logout(
    session: Session,
    supabase: web::Data<SupabaseClient>,
) -> Result<HttpResponse, AppError> {
    // Attempt to revoke the token with Supabase (best-effort)
    if let Ok(Some(access_token)) = session.get::<String>(SESSION_ACCESS_TOKEN) {
        if let Err(e) = supabase.sign_out(&access_token).await {
            log::warn!(
                "Supabase sign-out failed (continuing with local logout): {}",
                e
            );
        }
    }

    // Always purge the local session regardless of Supabase result
    session.purge();

    log::info!("User logged out");

    Ok(HttpResponse::Ok().json(json!({ "message": "Logged out successfully" })))
}

/// GET /api/v1/auth/me
///
/// Returns the currently authenticated user, or 401 if not logged in.
pub async fn me(
    session: Session,
    supabase: web::Data<SupabaseClient>,
) -> Result<HttpResponse, AppError> {
    let access_token = session
        .get::<String>(SESSION_ACCESS_TOKEN)
        .map_err(|e| AppError::Internal(format!("Session read error: {}", e)))?
        .ok_or_else(|| AppError::Unauthorized("Not authenticated".to_string()))?;

    // Validate the token with Supabase on every /me call to ensure it hasn't expired
    let supabase_user = supabase.get_user(&access_token).await.map_err(|_| {
        // Token is invalid or expired - clear the session
        session.purge();
        AppError::Unauthorized("Session expired or invalid".to_string())
    })?;

    let email = if let Some(email) = supabase_user.email {
        email
    } else {
        session
            .get::<String>(SESSION_USER_EMAIL)
            .map_err(|e| AppError::Internal(format!("Session read error: {}", e)))?
            .unwrap_or_default()
    };

    let user = AuthUser {
        id: supabase_user.id,
        email,
    };

    Ok(HttpResponse::Ok().json(user))
}
