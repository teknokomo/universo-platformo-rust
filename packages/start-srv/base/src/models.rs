//! Shared data models for request/response serialization.

use serde::{Deserialize, Serialize};

/// Login request body
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// Authenticated user representation
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthUser {
    pub id: String,
    pub email: String,
}

/// Onboarding item types
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OnboardingItem {
    pub id: String,
    pub title: String,
    pub description: String,
}

/// Onboarding items response
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnboardingItems {
    pub projects: Vec<OnboardingItem>,
    pub campaigns: Vec<OnboardingItem>,
    pub clusters: Vec<OnboardingItem>,
    pub onboarding_completed: bool,
}

/// Join items request body
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JoinItemsRequest {
    pub project_ids: Vec<String>,
    pub campaign_ids: Vec<String>,
    pub cluster_ids: Vec<String>,
}

/// Join items response
#[derive(Debug, Serialize)]
pub struct JoinItemsResponse {
    pub success: bool,
    pub message: String,
}
