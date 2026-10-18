mod api;
mod models;
mod routes;
mod components;
mod pages;

use crate::routes::Route;
use crate::pages::{ArtworkViewer, Gallery, OriginResults};

use yew::prelude::*;
use yew_router::prelude::*;

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Gallery /> },
        Route::About => html! {
            <section>
                <h1>{ "About this museum" }</h1>
                <p>{ "A small art discovery app built with Rust and Yew." }</p>
            </section>
        },
        Route::Origin { origin } => {
            let key = origin.clone();

            html! {
                <OriginResults key={key} origin={origin} />
            }
        },
        Route::NotFound => html! {
            <section>
                <h1>{ "Page not found" }</h1>
                <Link<Route> to={Route::Home}>{ "Return to the gallery" }</Link<Route>>
            </section>
        },
        Route::Artwork { id } => html! { <ArtworkViewer key={id.to_string()} id={id} />
        },

    }
}

#[component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <header class="site-header">
                <p class="site-brand">{ "The Little Museum" }</p>
                <nav class="site-nav" aria-label="Main Navigation">
                    <Link<Route> to={Route::Home}>{ "Gallery" }</Link<Route>>
                    { " | " }
                    <Link<Route> to={Route::About}>{ "About" }</Link<Route>>
                </nav>
            </header>
            <main>
                <Switch<Route> render={switch} />
            </main>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
