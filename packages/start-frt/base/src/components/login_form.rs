//! Login form component with email/password fields and Supabase auth.
//!
//! Renders as a modal overlay with a centered card containing the login form.
//! Dispatches to the auth context on successful login.

use yew::prelude::*;

use crate::api::client::api_post;
use crate::auth::{AuthAction, AuthContext, AuthUser};

/// Login response from the backend
#[derive(serde::Deserialize)]
struct LoginResponse {
    user: AuthUser,
}

/// Login request body
#[derive(serde::Serialize)]
struct LoginRequest {
    email: String,
    password: String,
}

/// Props for the LoginForm component
#[derive(Properties, PartialEq)]
pub struct LoginFormProps {
    /// Callback to close the login modal
    pub on_close: Callback<()>,
}

/// Login form modal component
#[function_component(LoginForm)]
pub fn login_form(props: &LoginFormProps) -> Html {
    let auth =
        use_context::<UseReducerHandle<AuthContext>>().expect("AuthContext must be provided");

    let email = use_state(String::new);
    let password = use_state(String::new);
    let is_loading = use_state(|| false);
    let error = use_state(|| None::<String>);

    let on_email_change = {
        let email = email.clone();
        Callback::from(move |e: InputEvent| {
            let target = e.target_dyn_into::<web_sys::HtmlInputElement>();
            if let Some(input) = target {
                email.set(input.value());
            }
        })
    };

    let on_password_change = {
        let password = password.clone();
        Callback::from(move |e: InputEvent| {
            let target = e.target_dyn_into::<web_sys::HtmlInputElement>();
            if let Some(input) = target {
                password.set(input.value());
            }
        })
    };

    let on_submit = {
        let email = email.clone();
        let password = password.clone();
        let is_loading = is_loading.clone();
        let error = error.clone();
        let auth = auth.clone();
        let on_close = props.on_close.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let email_val = (*email).clone();
            let password_val = (*password).clone();
            let is_loading = is_loading.clone();
            let error = error.clone();
            let auth = auth.clone();
            let on_close = on_close.clone();

            if email_val.is_empty() || password_val.is_empty() {
                error.set(Some("Please enter your email and password".to_string()));
                return;
            }

            is_loading.set(true);
            error.set(None);

            wasm_bindgen_futures::spawn_local(async move {
                let request = LoginRequest {
                    email: email_val,
                    password: password_val,
                };

                match api_post::<LoginRequest, LoginResponse>("/api/v1/auth/login", &request).await
                {
                    Ok(response) => {
                        auth.dispatch(AuthAction::SetUser(Some(response.user)));
                        on_close.emit(());
                    }
                    Err(e) => {
                        // Extract a user-friendly error message
                        let msg = if e.contains("401") || e.contains("Invalid") {
                            "Invalid email or password".to_string()
                        } else {
                            format!("Login failed: {}", e)
                        };
                        error.set(Some(msg));
                        is_loading.set(false);
                    }
                }
            });
        })
    };

    let on_backdrop_click = {
        let on_close = props.on_close.clone();
        Callback::from(move |_: MouseEvent| {
            on_close.emit(());
        })
    };

    html! {
        <div class="modal-overlay" onclick={on_backdrop_click}>
            <div
                class="modal-card login-form"
                onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}
            >
                <div class="modal-header">
                    <h2 class="modal-title">{ "Sign in to Universo Platformo" }</h2>
                    <button
                        class="modal-close"
                        onclick={
                            let on_close = props.on_close.clone();
                            Callback::from(move |_: MouseEvent| on_close.emit(()))
                        }
                    >
                        { "✕" }
                    </button>
                </div>

                <form class="form" onsubmit={on_submit}>
                    if let Some(err) = (*error).clone() {
                        <div class="form-error">{ err }</div>
                    }

                    <div class="form-field">
                        <label class="form-label" for="email">{ "Email" }</label>
                        <input
                            id="email"
                            class="form-input"
                            type="email"
                            placeholder="you@example.com"
                            value={(*email).clone()}
                            oninput={on_email_change}
                            disabled={*is_loading}
                            required=true
                        />
                    </div>

                    <div class="form-field">
                        <label class="form-label" for="password">{ "Password" }</label>
                        <input
                            id="password"
                            class="form-input"
                            type="password"
                            placeholder="Your password"
                            value={(*password).clone()}
                            oninput={on_password_change}
                            disabled={*is_loading}
                            required=true
                        />
                    </div>

                    <button
                        class="btn btn-primary btn-full"
                        type="submit"
                        disabled={*is_loading}
                    >
                        if *is_loading {
                            { "Signing in..." }
                        } else {
                            { "Sign In" }
                        }
                    </button>
                </form>

                <p class="form-footer-text">
                    { "Don't have an account? " }
                    <a href="https://universo.pro" target="_blank" rel="noopener noreferrer">
                        { "Contact us" }
                    </a>
                </p>
            </div>
        </div>
    }
}
