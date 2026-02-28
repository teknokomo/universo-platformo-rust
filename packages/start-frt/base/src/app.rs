//! Root App component with Yew Router configuration.

use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::start_page::StartPage;

/// Application routes
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Start,
    #[not_found]
    #[at("/404")]
    NotFound,
}

/// Route renderer - maps routes to page components
fn switch(route: Route) -> Html {
    match route {
        Route::Start => html! { <StartPage /> },
        Route::NotFound => html! {
            <div style="text-align:center;padding:4rem;color:#888;">
                <h1>{ "404 - Page Not Found" }</h1>
                <a href="/">{ "← Go Home" }</a>
            </div>
        },
    }
}

/// Root application component
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
