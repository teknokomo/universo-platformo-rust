//! Hero component for the guest landing page.
//!
//! Displays the main headline, subtitle, and call-to-action buttons
//! for non-authenticated visitors.

use yew::prelude::*;

/// Props for the Hero component
#[derive(Properties, PartialEq)]
pub struct HeroProps {
    /// Callback fired when the user clicks a CTA button
    pub on_start: Callback<MouseEvent>,
}

/// Hero section displayed on the guest landing page
#[function_component(Hero)]
pub fn hero(props: &HeroProps) -> Html {
    html! {
        <div class="hero">
            <div class="hero-content">
                <h1 class="hero-title">
                    { "Build Worlds," }
                    <br />
                    { "Connect Universes" }
                </h1>
                <p class="hero-subtitle">
                    {
                        "Universo Platformo is an open platform for creating \
                        immersive 3D, AR, and VR experiences. Join the global \
                        community building the future of virtual worlds."
                    }
                </p>
                <div class="hero-actions">
                    <button
                        class="btn btn-primary btn-large"
                        onclick={props.on_start.clone()}
                    >
                        { "Start Now →" }
                    </button>
                    <a
                        class="btn btn-ghost btn-large"
                        href="https://universo.pro"
                        target="_blank"
                        rel="noopener noreferrer"
                    >
                        { "Learn More" }
                    </a>
                </div>
            </div>
        </div>
    }
}
