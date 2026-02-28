//! Onboarding handlers for fetching and joining platform items.
//!
//! Provides sample onboarding data (Projects, Campaigns, Clusters).
//! In production, this would query a database for real data.

use actix_session::Session;
use actix_web::{web, HttpResponse};

use crate::error::AppError;
use crate::models::{JoinItemsRequest, JoinItemsResponse, OnboardingItem, OnboardingItems};

use super::SESSION_ACCESS_TOKEN;

/// Session key for tracking onboarding completion status.
const SESSION_ONBOARDING_COMPLETED: &str = "onboarding_completed";

/// Known valid project IDs in the onboarding catalog.
const VALID_PROJECT_IDS: &[&str] = &["project-1", "project-2", "project-3"];
/// Known valid campaign IDs in the onboarding catalog.
const VALID_CAMPAIGN_IDS: &[&str] = &["campaign-1", "campaign-2", "campaign-3"];
/// Known valid cluster IDs in the onboarding catalog.
const VALID_CLUSTER_IDS: &[&str] = &["cluster-1", "cluster-2", "cluster-3"];

/// GET /api/v1/onboarding/items
///
/// Returns onboarding items (Projects, Campaigns, Clusters) and whether
/// the current user has already completed onboarding.
pub async fn get_items(session: Session) -> Result<HttpResponse, AppError> {
    // Check if user is authenticated
    let is_authenticated = session
        .get::<String>(SESSION_ACCESS_TOKEN)
        .map_err(|e| AppError::Internal(format!("Session read error: {}", e)))?
        .is_some();

    if !is_authenticated {
        return Err(AppError::Unauthorized("Not authenticated".to_string()));
    }

    let onboarding_completed = session
        .get::<bool>(SESSION_ONBOARDING_COMPLETED)
        .unwrap_or(None)
        .unwrap_or(false);

    // Sample onboarding data - in production this would come from a database
    let items = OnboardingItems {
        projects: vec![
            OnboardingItem {
                id: "project-1".to_string(),
                title: "Global Goals".to_string(),
                description: "Join global sustainability and development initiatives".to_string(),
            },
            OnboardingItem {
                id: "project-2".to_string(),
                title: "Tech Innovation".to_string(),
                description: "Contribute to cutting-edge technology projects".to_string(),
            },
            OnboardingItem {
                id: "project-3".to_string(),
                title: "Community Building".to_string(),
                description: "Help build and strengthen communities worldwide".to_string(),
            },
        ],
        campaigns: vec![
            OnboardingItem {
                id: "campaign-1".to_string(),
                title: "Education".to_string(),
                description: "Support educational initiatives globally".to_string(),
            },
            OnboardingItem {
                id: "campaign-2".to_string(),
                title: "Healthcare".to_string(),
                description: "Advance healthcare access and quality".to_string(),
            },
            OnboardingItem {
                id: "campaign-3".to_string(),
                title: "Environment".to_string(),
                description: "Protect and restore natural environments".to_string(),
            },
        ],
        clusters: vec![
            OnboardingItem {
                id: "cluster-1".to_string(),
                title: "3D Worlds".to_string(),
                description: "Create immersive 3D virtual worlds".to_string(),
            },
            OnboardingItem {
                id: "cluster-2".to_string(),
                title: "AR Experiences".to_string(),
                description: "Build augmented reality experiences".to_string(),
            },
            OnboardingItem {
                id: "cluster-3".to_string(),
                title: "VR Simulations".to_string(),
                description: "Design virtual reality simulations".to_string(),
            },
        ],
        onboarding_completed,
    };

    Ok(HttpResponse::Ok().json(items))
}

/// POST /api/v1/onboarding/join
///
/// Records the user's selected onboarding items and marks onboarding as complete.
pub async fn join_items(
    session: Session,
    body: web::Json<JoinItemsRequest>,
) -> Result<HttpResponse, AppError> {
    // Check if user is authenticated
    let is_authenticated = session
        .get::<String>(SESSION_ACCESS_TOKEN)
        .map_err(|e| AppError::Internal(format!("Session read error: {}", e)))?
        .is_some();

    if !is_authenticated {
        return Err(AppError::Unauthorized("Not authenticated".to_string()));
    }

    // Validate that at least one item was selected across all categories
    if body.project_ids.is_empty() && body.campaign_ids.is_empty() && body.cluster_ids.is_empty() {
        return Err(AppError::BadRequest(
            "At least one item must be selected".to_string(),
        ));
    }

    // Validate all submitted IDs exist in the known catalog.
    // This prevents phantom IDs from being accepted silently.
    // The catalog is a small static set (3 entries each), so linear search is O(n×3) ≈ O(n).
    for id in &body.project_ids {
        if !VALID_PROJECT_IDS.contains(&id.as_str()) {
            return Err(AppError::BadRequest(format!("Unknown project id: {}", id)));
        }
    }
    for id in &body.campaign_ids {
        if !VALID_CAMPAIGN_IDS.contains(&id.as_str()) {
            return Err(AppError::BadRequest(format!("Unknown campaign id: {}", id)));
        }
    }
    for id in &body.cluster_ids {
        if !VALID_CLUSTER_IDS.contains(&id.as_str()) {
            return Err(AppError::BadRequest(format!("Unknown cluster id: {}", id)));
        }
    }

    log::info!(
        "User joined onboarding items: {} projects, {} campaigns, {} clusters",
        body.project_ids.len(),
        body.campaign_ids.len(),
        body.cluster_ids.len()
    );

    // Mark onboarding as completed in session
    session
        .insert(SESSION_ONBOARDING_COMPLETED, true)
        .map_err(|e| AppError::Internal(format!("Session insert error: {}", e)))?;

    Ok(HttpResponse::Ok().json(JoinItemsResponse {
        success: true,
        message: "Successfully joined selected items. Welcome to Universo Platformo!".to_string(),
    }))
}
