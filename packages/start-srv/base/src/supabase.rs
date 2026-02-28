//! Supabase HTTP client for authentication operations.
//!
//! Communicates with Supabase Auth REST API to perform sign-in,
//! sign-out, and session validation.

use serde::Deserialize;
use std::time::Duration;

/// Minimal Supabase Auth API client
pub struct SupabaseClient {
    url: String,
    anon_key: String,
    http: reqwest::Client,
}

/// Supabase Auth sign-in response
#[derive(Debug, Deserialize)]
pub struct SupabaseAuthResponse {
    pub access_token: String,
    #[allow(dead_code)]
    pub refresh_token: String,
    pub user: SupabaseUser,
}

/// Supabase user object
#[derive(Debug, Deserialize)]
pub struct SupabaseUser {
    pub id: String,
    pub email: Option<String>,
}

/// Supabase error response body
#[derive(Debug, Deserialize)]
struct SupabaseErrorBody {
    /// The `error` code returned by Supabase/GoTrue (e.g. `"invalid_grant"`)
    error: Option<String>,
    message: Option<String>,
    error_description: Option<String>,
    msg: Option<String>,
}

impl SupabaseClient {
    /// Create a new Supabase client.
    pub fn new(url: String, anon_key: String) -> Self {
        // Building a reqwest Client can only fail if TLS initialisation fails,
        // which is a fatal condition — the server cannot serve any requests without it.
        let http = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .expect(
                "Failed to build Supabase HTTP client. \
                Ensure a TLS backend (e.g. rustls or native-tls) is available.",
            );
        Self {
            url,
            anon_key,
            http,
        }
    }

    /// Sign in a user with email and password.
    ///
    /// Calls `POST /auth/v1/token?grant_type=password` on the Supabase project.
    pub async fn sign_in(
        &self,
        email: &str,
        password: &str,
    ) -> Result<SupabaseAuthResponse, String> {
        let endpoint = format!("{}/auth/v1/token?grant_type=password", self.url);
        let body = serde_json::json!({
            "email": email,
            "password": password,
        });

        let response = self
            .http
            .post(&endpoint)
            .header("apikey", &self.anon_key)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if response.status().is_success() {
            response
                .json::<SupabaseAuthResponse>()
                .await
                .map_err(|e| format!("Failed to parse auth response: {}", e))
        } else {
            let status = response.status();
            let error_body =
                response
                    .json::<SupabaseErrorBody>()
                    .await
                    .unwrap_or(SupabaseErrorBody {
                        error: None,
                        message: Some("Unknown error".to_string()),
                        error_description: None,
                        msg: None,
                    });
            let msg = error_body
                .error_description
                .or(error_body.message)
                .or(error_body.error)
                .or(error_body.msg)
                .unwrap_or_else(|| "Authentication failed".to_string());
            Err(format!("Supabase auth error ({}): {}", status, msg))
        }
    }

    /// Validate an access token by calling the Supabase user endpoint.
    ///
    /// Returns the user if the token is valid, or an error if not.
    pub async fn get_user(&self, access_token: &str) -> Result<SupabaseUser, String> {
        let endpoint = format!("{}/auth/v1/user", self.url);

        let response = self
            .http
            .get(&endpoint)
            .header("apikey", &self.anon_key)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if response.status().is_success() {
            response
                .json::<SupabaseUser>()
                .await
                .map_err(|e| format!("Failed to parse user response: {}", e))
        } else {
            Err(format!(
                "Token validation failed: HTTP {}",
                response.status()
            ))
        }
    }

    /// Sign out a user by revoking their token.
    ///
    /// Calls `POST /auth/v1/logout` on the Supabase project.
    pub async fn sign_out(&self, access_token: &str) -> Result<(), String> {
        let endpoint = format!("{}/auth/v1/logout", self.url);

        let response = self
            .http
            .post(&endpoint)
            .header("apikey", &self.anon_key)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if response.status().is_success() || response.status().as_u16() == 204 {
            Ok(())
        } else {
            Err(format!("Logout failed: HTTP {}", response.status()))
        }
    }
}
