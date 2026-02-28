//! Route handlers module

pub mod auth;
pub mod onboarding;

/// Session key used to store the Supabase access token.
pub(super) const SESSION_ACCESS_TOKEN: &str = "access_token";
/// Session key used to store the authenticated user ID.
pub(super) const SESSION_USER_ID: &str = "user_id";
/// Session key used to store the authenticated user email.
pub(super) const SESSION_USER_EMAIL: &str = "user_email";
