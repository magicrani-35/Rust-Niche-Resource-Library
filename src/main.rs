mod api;
mod models;
mod routes;
mod components;
mod pages;

use crate::components::{ArtworkRow};

use crate::api::{
    fetch_origin_artworks,
};

use crate::models::{
    ApiArtwork,
};

use crate::routes::Route;
use crate::pages::{ArtworkViewer, Gallery};

use yew::prelude::*;
use yew_router::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[derive(Properties, PartialEq)]
struct OriginResultsProps {
    origin: String,
}

#[component]
fn OriginResults(props: &OriginResultsProps) -> Html {
    let artworks = use_state(|| None::
    <Result<Vec<ApiArtwork>, String>>);

    {
        let artworks = artworks.clone();

        use_effect_with(props.origin.clone(), move |origin| {
            let origin = origin.clone();

            artworks.set(None);

            spawn_local(async move {
                let result = fetch_origin_artworks(&origin).await;

                artworks.set(Some(result));
            });

            || ()
        });
    }

    let content = match &*artworks {
        None => html! {
            <p role="status">{ "Loading artwork..." }</p>
        },
        Some(Err(error)) => html! {
            <p role="alert">
                { format!("Could not load artwork: {error}") }
            </p>
        },
        Some(Ok(items)) if items.is_empty() => html! {
            <p>{ "No artworks were found for this origin" }</p>
        },
        Some(Ok(items)) => html! {
            <div class="origin-results">
                {
                    for items.iter().map(|artwork| {
                    html! {
                        <ArtworkRow key={artwork.id.to_string()} artwork={artwork.clone()} />
                        }
                    })
                }
            </div>
        },
    };

    html! {
        <section>
            <Link<Route> to={Route::Home}>
                { "<- Back to gallery" }
            </Link<Route>>

            <h1 class="viewer-heading">
                { format!("Art from {}", props.origin) }
            </h1>
            <p class="origin-results-description">
                { "Showing the first 12 matchings artworks." }
            </p>

            { content }
        </section>
    }
}

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
