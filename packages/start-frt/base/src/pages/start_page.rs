//! StartPage - Conditional start page based on authentication status.
//!
//! Shows:
//! - GuestStartPage for non-authenticated users (landing with login form)
//! - AuthenticatedStartPage for authenticated users (onboarding wizard)
//! - Loading indicator while checking authentication

use yew::prelude::*;

use crate::auth::{AuthContext, AuthProvider};
use crate::pages::authenticated_start_page::AuthenticatedStartPage;
use crate::pages::guest_start_page::GuestStartPage;

/// Inner start page component that reads auth context.
///
/// Renders either the guest or authenticated page based on auth state.
#[function_component(StartPageInner)]
fn start_page_inner() -> Html {
    let auth = use_context::<UseReducerHandle<AuthContext>>()
        .expect("AuthContext not found - ensure StartPage is wrapped in AuthProvider");

    if auth.loading {
        return html! {
            <div class="loading-screen">
                <div class="loading-spinner">
                    <div class="spinner" />
                    <p>{ "Loading..." }</p>
                </div>
            </div>
        };
    }

    if auth.is_authenticated {
        html! { <AuthenticatedStartPage /> }
    } else {
        html! { <GuestStartPage /> }
    }
}

/// StartPage is the top-level page component.
///
/// Wraps the inner page in an AuthProvider so authentication state
/// is automatically loaded and shared with child components.
#[function_component(StartPage)]
pub fn start_page() -> Html {
    html! {
        <AuthProvider>
            <StartPageInner />
        </AuthProvider>
    }
}
