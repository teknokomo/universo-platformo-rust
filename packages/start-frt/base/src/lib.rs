//! Universo Platformo | Start Frontend
//!
//! Yew/WebAssembly frontend for the start page functionality.
//! Provides conditional rendering based on Supabase authentication state:
//! - GuestStartPage for unauthenticated visitors
//! - AuthenticatedStartPage for authenticated users with onboarding wizard

use wasm_bindgen::prelude::*;

pub mod api;
pub mod auth;
pub mod components;
pub mod pages;

mod app;

/// Main entry point for the WebAssembly application.
///
/// Called by the generated JavaScript glue code to mount the Yew app.
#[wasm_bindgen(start)]
pub fn main() {
    // Initialize logger (only in debug builds for performance)
    #[cfg(debug_assertions)]
    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<app::App>::new().render();
}
