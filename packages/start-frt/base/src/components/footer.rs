//! Footer component for start pages.
//!
//! Displays contact information and links for both guest and authenticated views.

use yew::prelude::*;

/// Props for the StartFooter component
#[derive(Properties, PartialEq)]
pub struct StartFooterProps {
    /// Footer variant: "guest" (default) or "internal" for authenticated users
    #[prop_or("guest")]
    pub variant: &'static str,
}

/// Footer component shown at the bottom of start pages
#[function_component(StartFooter)]
pub fn start_footer(props: &StartFooterProps) -> Html {
    html! {
        <footer class={classes!("start-footer", format!("start-footer--{}", props.variant))}>
            <div class="footer-content">
                <div class="footer-brand">
                    <span class="footer-logo">{ "🌌" }</span>
                    <span class="footer-name">{ "Universo Platformo" }</span>
                </div>
                <div class="footer-links">
                    <a href="https://universo.pro" target="_blank" rel="noopener noreferrer">
                        { "Website" }
                    </a>
                    <a href="https://t.me/Vladimir_Levadnij" target="_blank" rel="noopener noreferrer">
                        { "Telegram" }
                    </a>
                    <a href="https://vk.com/vladimirlevadnij" target="_blank" rel="noopener noreferrer">
                        { "VK" }
                    </a>
                    <a href="mailto:universo.pro@yandex.com">
                        { "Email" }
                    </a>
                </div>
                <div class="footer-copy">
                    { "© 2025 Universo Platformo. Omsk Open License." }
                </div>
            </div>
        </footer>
    }
}
