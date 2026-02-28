//! API module

pub mod client;

use serde::{Deserialize, Serialize};

/// Onboarding item
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OnboardingItem {
    pub id: String,
    pub title: String,
    pub description: String,
}

/// Onboarding items response from backend
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnboardingItems {
    pub projects: Vec<OnboardingItem>,
    pub campaigns: Vec<OnboardingItem>,
    pub clusters: Vec<OnboardingItem>,
    pub onboarding_completed: bool,
}

/// Join items request body
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JoinItemsRequest {
    pub project_ids: Vec<String>,
    pub campaign_ids: Vec<String>,
    pub cluster_ids: Vec<String>,
}

/// Join items response
#[derive(Debug, Deserialize)]
pub struct JoinItemsResponse {
    pub success: bool,
    pub message: String,
}
