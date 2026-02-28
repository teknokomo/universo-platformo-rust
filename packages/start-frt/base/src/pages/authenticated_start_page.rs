//! AuthenticatedStartPage - Onboarding wizard for authenticated users.
//!
//! Displays a multi-step wizard to help users select their interests:
//! - Step 1: Select Projects (Global Goals)
//! - Step 2: Select Campaigns (Personal Interests)
//! - Step 3: Select Clusters (Platform Features)
//! - Step 4: Completion / Welcome screen
//!
//! If onboarding is already completed, shows the completion screen directly.

use yew::prelude::*;

use crate::api::client::{api_get, api_post};
use crate::api::{JoinItemsRequest, OnboardingItems};
use crate::auth::{AuthAction, AuthContext};
use crate::components::footer::StartFooter;
use crate::components::onboarding_wizard::OnboardingWizard;

/// AuthenticatedStartPage - shown to authenticated users
#[function_component(AuthenticatedStartPage)]
pub fn authenticated_start_page() -> Html {
    let auth =
        use_context::<UseReducerHandle<AuthContext>>().expect("AuthContext must be provided");

    let onboarding_items: UseStateHandle<Option<OnboardingItems>> = use_state(|| None);
    let is_loading = use_state(|| true);
    let error = use_state(|| None::<String>);

    // Fetch onboarding items on mount
    {
        let onboarding_items = onboarding_items.clone();
        let is_loading = is_loading.clone();
        let error = error.clone();
        use_effect_with((), move |_| {
            let onboarding_items = onboarding_items.clone();
            let is_loading = is_loading.clone();
            let error = error.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match api_get::<OnboardingItems>("/api/v1/onboarding/items").await {
                    Ok(items) => {
                        onboarding_items.set(Some(items));
                        is_loading.set(false);
                    }
                    Err(e) => {
                        log::error!("Failed to load onboarding items: {}", e);
                        error.set(Some(e));
                        is_loading.set(false);
                    }
                }
            });
        });
    }

    // Handle completion of onboarding wizard
    let on_complete = {
        let onboarding_items = onboarding_items.clone();
        Callback::from(move |request: JoinItemsRequest| {
            let onboarding_items = onboarding_items.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match api_post::<JoinItemsRequest, crate::api::JoinItemsResponse>(
                    "/api/v1/onboarding/join",
                    &request,
                )
                .await
                {
                    Ok(_) => {
                        // Refresh onboarding state to show completion screen
                        if let Ok(items) =
                            api_get::<OnboardingItems>("/api/v1/onboarding/items").await
                        {
                            onboarding_items.set(Some(items));
                        }
                    }
                    Err(e) => {
                        log::error!("Failed to join items: {}", e);
                    }
                }
            });
        })
    };

    // Handle logout
    let on_logout = {
        let auth = auth.clone();
        Callback::from(move |_: MouseEvent| {
            let auth = auth.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Err(e) = api_post::<serde_json::Value, serde_json::Value>(
                    "/api/v1/auth/logout",
                    &serde_json::json!({}),
                )
                .await
                {
                    log::warn!("Logout request failed: {}", e);
                }
                auth.dispatch(AuthAction::SetUser(None));
            });
        })
    };

    // Handle "start over" (reset onboarding)
    let on_start_over = {
        let onboarding_items = onboarding_items.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(mut items) = (*onboarding_items).clone() {
                items.onboarding_completed = false;
                onboarding_items.set(Some(items));
            }
        })
    };

    html! {
        <div class="authenticated-start-page">
            // Navigation bar for authenticated users
            <nav class="navbar navbar-authenticated">
                <div class="navbar-brand">
                    <span class="navbar-logo">{ "🌌" }</span>
                    <span class="navbar-title">{ "Universo Platformo" }</span>
                </div>
                <div class="navbar-user">
                    if let Some(user) = &auth.user {
                        <span class="user-email">{ &user.email }</span>
                    }
                    <button class="btn btn-outline btn-sm" onclick={on_logout}>
                        { "Sign Out" }
                    </button>
                </div>
            </nav>

            // Main content
            <main class="main-content">
                if *is_loading {
                    <div class="loading-screen">
                        <div class="spinner" />
                        <p>{ "Loading your profile..." }</p>
                    </div>
                } else if let Some(err) = (*error).clone() {
                    <div class="error-screen">
                        <p class="error-message">{ format!("Error: {}", err) }</p>
                    </div>
                } else if let Some(items) = (*onboarding_items).clone() {
                    if items.onboarding_completed {
                        // Show completion/welcome screen
                        <div class="completion-screen">
                            <div class="completion-content">
                                <h1 class="completion-title">{ "🎉 Welcome to Universo Platformo!" }</h1>
                                <p class="completion-subtitle">
                                    { "You're all set! Your profile has been configured and you're ready to explore." }
                                </p>
                                if let Some(user) = &auth.user {
                                    <p class="user-greeting">
                                        { format!("Logged in as: {}", user.email) }
                                    </p>
                                }
                                <div class="completion-actions">
                                    <button
                                        class="btn btn-secondary"
                                        onclick={on_start_over}
                                    >
                                        { "Update Preferences" }
                                    </button>
                                </div>
                            </div>
                        </div>
                    } else {
                        // Show onboarding wizard
                        <OnboardingWizard
                            items={items}
                            on_complete={on_complete}
                        />
                    }
                }
            </main>

            // Footer
            <StartFooter variant={"internal"} />
        </div>
    }
}
