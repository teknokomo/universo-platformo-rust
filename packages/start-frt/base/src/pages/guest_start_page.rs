//! GuestStartPage - Landing page for non-authenticated users.
//!
//! Displays:
//! - Navigation bar with logo and auth buttons
//! - Hero section with title and "Sign in" / "Get started" call-to-action
//! - Testimonials section
//! - Footer with contact information
//! - Login modal/form when triggered

use yew::prelude::*;

use crate::components::footer::StartFooter;
use crate::components::hero::Hero;
use crate::components::login_form::LoginForm;
use crate::components::testimonials::Testimonials;

/// GuestStartPage - shown to users who are not authenticated
#[function_component(GuestStartPage)]
pub fn guest_start_page() -> Html {
    let show_login = use_state(|| false);

    let on_open_login = {
        let show_login = show_login.clone();
        Callback::from(move |_| show_login.set(true))
    };

    let on_close_login = {
        let show_login = show_login.clone();
        Callback::from(move |_| show_login.set(false))
    };

    html! {
        <div class="guest-start-page">
            // Navigation bar
            <nav class="navbar">
                <div class="navbar-brand">
                    <span class="navbar-logo">{ "🌌" }</span>
                    <span class="navbar-title">{ "Universo Platformo" }</span>
                </div>
                <div class="navbar-actions">
                    <button
                        class="btn btn-outline"
                        onclick={on_open_login.clone()}
                    >
                        { "Sign In" }
                    </button>
                    <button
                        class="btn btn-primary"
                        onclick={on_open_login.clone()}
                    >
                        { "Get Started" }
                    </button>
                </div>
            </nav>

            // Hero section
            <div class="hero-section">
                <Hero on_start={on_open_login} />
            </div>

            // Testimonials section
            <div class="testimonials-section">
                <Testimonials />
            </div>

            // Footer
            <StartFooter />

            // Login modal
            if *show_login {
                <LoginForm on_close={on_close_login} />
            }
        </div>
    }
}
