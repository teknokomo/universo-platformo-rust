//! Application configuration loaded from environment variables.

use std::env;

/// Application configuration
pub struct AppConfig {
    /// Server host address
    pub host: String,
    /// Server port
    pub port: u16,
    /// Session secret key (must be at least 64 bytes)
    pub session_secret: String,
    /// Whether session cookies require HTTPS (should be true in production)
    pub cookie_secure: bool,
    /// Allowed CORS origins (empty list = allow all, development only)
    pub cors_allowed_origins: Vec<String>,
    /// Supabase project URL
    pub supabase_url: String,
    /// Supabase anonymous/public API key
    pub supabase_anon_key: String,
}

impl AppConfig {
    /// Load configuration from environment variables.
    ///
    /// In production, `SESSION_SECRET` and `CORS_ALLOWED_ORIGINS` must be set explicitly.
    /// A warning is logged when the default insecure session secret is used.
    pub fn from_env() -> Self {
        let session_secret = env::var("SESSION_SECRET").unwrap_or_else(|_| {
            // Log a visible warning — this default must never be used in production
            log::warn!(
                "SESSION_SECRET is not set! Using insecure default. \
                Set SESSION_SECRET in production to a random 64+ byte value."
            );
            "CHANGE_THIS_IN_PRODUCTION_TO_A_SECURE_64_BYTE_MINIMUM_SECRET_KEY_HERE!!".to_string()
        });

        let cors_allowed_origins: Vec<String> = env::var("CORS_ALLOWED_ORIGINS")
            .map(|s| s.split(',').map(|o| o.trim().to_string()).collect())
            .unwrap_or_default();

        if cors_allowed_origins.is_empty() {
            log::warn!(
                "CORS_ALLOWED_ORIGINS is not set. Allowing all origins. \
                Set CORS_ALLOWED_ORIGINS in production (e.g. https://yourdomain.com)."
            );
        }

        let cookie_secure = env::var("COOKIE_SECURE")
            .map(|v| v.to_lowercase() == "true" || v == "1")
            .unwrap_or(false);

        Self {
            host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: env::var("PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(8080),
            session_secret,
            cookie_secure,
            cors_allowed_origins,
            supabase_url: env::var("SUPABASE_URL")
                .unwrap_or_else(|_| "https://example.supabase.co".to_string()),
            supabase_anon_key: env::var("SUPABASE_ANON_KEY")
                .unwrap_or_else(|_| "your-anon-key".to_string()),
        }
    }
}
