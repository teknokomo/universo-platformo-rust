//! HTTP API client for communicating with the backend server.
//!
//! Uses gloo-net for making fetch requests from WebAssembly.
//! All requests include credentials (cookies) to maintain sessions.

use gloo_net::http::Request;
use serde::{de::DeserializeOwned, Serialize};

/// Base API URL - empty string means relative to current origin (works with Trunk proxy)
const API_BASE: &str = "";

/// Send a GET request to the backend API.
///
/// Returns the deserialized response body on success.
pub async fn api_get<T: DeserializeOwned>(path: &str) -> Result<T, String> {
    let url = format!("{}{}", API_BASE, path);
    let response = Request::get(&url)
        .credentials(web_sys::RequestCredentials::Include)
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if response.ok() {
        response
            .json::<T>()
            .await
            .map_err(|e| format!("Parse error: {}", e))
    } else {
        let status = response.status();
        Err(format!("HTTP error: {}", status))
    }
}

/// Send a POST request with a JSON body to the backend API.
///
/// Returns the deserialized response body on success.
pub async fn api_post<B: Serialize, T: DeserializeOwned>(
    path: &str,
    body: &B,
) -> Result<T, String> {
    let url = format!("{}{}", API_BASE, path);
    let response = Request::post(&url)
        .credentials(web_sys::RequestCredentials::Include)
        .header("Content-Type", "application/json")
        .json(body)
        .map_err(|e| format!("Serialize error: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if response.ok() {
        response
            .json::<T>()
            .await
            .map_err(|e| format!("Parse error: {}", e))
    } else {
        let status = response.status();
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        Err(format!("HTTP {}: {}", status, error_text))
    }
}
