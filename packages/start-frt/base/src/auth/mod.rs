//! Authentication context and state management for Yew frontend.
//!
//! Provides an AuthContext that mirrors the React useAuth hook pattern,
//! allowing components to access authentication state and operations.

use serde::{Deserialize, Serialize};
use std::rc::Rc;
use yew::prelude::*;

use crate::api::client::api_get;

/// Authenticated user data
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthUser {
    pub id: String,
    pub email: String,
}

/// Authentication context value shared across the component tree
#[derive(Debug, Clone, PartialEq)]
pub struct AuthContext {
    /// Currently authenticated user, or None if not authenticated
    pub user: Option<AuthUser>,
    /// True while checking authentication status
    pub loading: bool,
    /// Last authentication error message
    pub error: Option<String>,
    /// True if a user is currently authenticated
    pub is_authenticated: bool,
}

impl Default for AuthContext {
    fn default() -> Self {
        Self {
            user: None,
            loading: true,
            error: None,
            is_authenticated: false,
        }
    }
}

/// Actions that can update auth state
pub enum AuthAction {
    SetLoading(bool),
    SetUser(Option<AuthUser>),
    SetError(Option<String>),
}

impl Reducible for AuthContext {
    type Action = AuthAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            AuthAction::SetLoading(loading) => Rc::new(AuthContext {
                loading,
                ..(*self).clone()
            }),
            AuthAction::SetUser(user) => {
                let is_authenticated = user.is_some();
                Rc::new(AuthContext {
                    user,
                    is_authenticated,
                    loading: false,
                    error: None,
                })
            }
            AuthAction::SetError(error) => Rc::new(AuthContext {
                error,
                loading: false,
                user: None,
                is_authenticated: false,
            }),
        }
    }
}

/// Props for the AuthProvider component
#[derive(Properties, PartialEq)]
pub struct AuthProviderProps {
    #[prop_or_default]
    pub children: Children,
}

/// AuthProvider component that wraps the app and provides auth context.
///
/// Checks the current session on mount by calling GET /api/v1/auth/me.
/// Exposes auth state via Yew context, accessible with `use_context::<UseReducerHandle<AuthContext>>()`.
#[function_component(AuthProvider)]
pub fn auth_provider(props: &AuthProviderProps) -> Html {
    let auth_state = use_reducer(AuthContext::default);

    // Check authentication status on mount
    {
        let auth_state = auth_state.clone();
        use_effect_with((), move |_| {
            let auth_state = auth_state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match api_get::<AuthUser>("/api/v1/auth/me").await {
                    Ok(user) => {
                        auth_state.dispatch(AuthAction::SetUser(Some(user)));
                    }
                    Err(_) => {
                        // Not authenticated - this is a normal state, not an error
                        auth_state.dispatch(AuthAction::SetUser(None));
                    }
                }
            });
        });
    }

    html! {
        <ContextProvider<UseReducerHandle<AuthContext>> context={auth_state}>
            { for props.children.iter() }
        </ContextProvider<UseReducerHandle<AuthContext>>>
    }
}
