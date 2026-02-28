//! Testimonials component displaying user reviews/quotes.
//!
//! Shows a grid of testimonial cards to build trust for new visitors.

use yew::prelude::*;

struct Testimonial {
    name: &'static str,
    role: &'static str,
    text: &'static str,
    avatar: &'static str,
}

const TESTIMONIALS: &[Testimonial] = &[
    Testimonial {
        name: "Alex Chen",
        role: "3D Artist",
        avatar: "🎨",
        text: "Universo Platformo transformed how I create 3D worlds. The node-based editor is intuitive and powerful.",
    },
    Testimonial {
        name: "Maria Santos",
        role: "VR Developer",
        avatar: "🥽",
        text: "Finally a platform that makes AR/VR development accessible to everyone. The WebAssembly performance is incredible.",
    },
    Testimonial {
        name: "Ivan Petrov",
        role: "Game Designer",
        avatar: "🎮",
        text: "The community here is amazing. Collaborative tools and the open-source approach make this truly special.",
    },
    Testimonial {
        name: "Yuki Tanaka",
        role: "Educator",
        avatar: "🎓",
        text: "Using Universo Platformo for educational simulations has been a game changer for our students.",
    },
];

/// Testimonials section for the guest landing page
#[function_component(Testimonials)]
pub fn testimonials() -> Html {
    html! {
        <section class="testimonials">
            <h2 class="testimonials-title">{ "What our community says" }</h2>
            <div class="testimonials-grid">
                { for TESTIMONIALS.iter().map(|t| html! {
                    <div class="testimonial-card" key={t.name}>
                        <p class="testimonial-text">{ format!("\"{}\"", t.text) }</p>
                        <div class="testimonial-author">
                            <span class="testimonial-avatar">{ t.avatar }</span>
                            <div class="testimonial-info">
                                <span class="testimonial-name">{ t.name }</span>
                                <span class="testimonial-role">{ t.role }</span>
                            </div>
                        </div>
                    </div>
                }) }
            </div>
        </section>
    }
}
