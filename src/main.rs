mod api;
mod models;
mod routes;
mod components;
mod pages;

use crate::components::{ArtworkRow};

use crate::api::{
    fetch_artwork,
    fetch_origin_artworks,
};

use crate::models::{
    ApiArtwork,
};

use crate::routes::Route;
use crate::pages::Gallery;

use yew::prelude::*;
use yew_router::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[derive(Properties, PartialEq)]
struct ArtworkViewerProps {
    id: u32,
}

#[component]
fn ArtworkViewer(props: &ArtworkViewerProps) -> Html {
    let artwork = use_state(|| None::<Result<ApiArtwork, String>>);
    let zoom = use_state(|| 1.0_f64);

    {
        let artwork = artwork.clone();

        use_effect_with(props.id, move |id| {
            let id = *id;

            artwork.set(None);

            spawn_local(async move {
                let result = fetch_artwork(id).await;
                artwork.set(Some(result));
            });

            || ()
        });
    }

    let zoom_in = {
        let zoom = zoom.clone();

        Callback::from(move |_| {
            zoom.set((*zoom + 0.25).min(3.0));
        })
    };

    let zoom_out = {
        let zoom = zoom.clone();

        Callback::from(move |_| zoom.set((*zoom - 0.25).max(1.0)))
    };

    let reset_zoom = {
        let zoom = zoom.clone();

        Callback::from(move |_| {
            zoom.set(1.0);
        })
    };

    let content = match &*artwork {
        None => html! { <p class="viewer-heading" role="status">{ "Loading artwork..." }</p> },
        Some(Err(error)) => html! {
            <p class="viewer-heading" role="alert">
                { format!("Could not load artwork: {error}") }
            </p>
        },
        Some(Ok(item)) => {
            let image = match &item.image_id {
                None => html! { <p>{ "An image is unavailable for this artwork" }</p> },
                Some(image_id) => {
                    let image_url =
                        format!("https://www.artic.edu/iiif/2/{image_id}/full/843,/0/default.jpg");

                    html! {
                        <>
                            <div class="viewer-controls" role="group" aria-label="Image zoom">
                                <button
                                    type="button"
                                    onclick={zoom_out.clone()}
                                    disabled={*zoom <= 1.0}
                                >
                                    { "- Zoom out" }
                                </button>
                                <output aria-live="polite">
                                    { format!("{:.0}%", *zoom * 100.0) }
                                </output>
                                <button
                                    type="button"
                                    onclick={zoom_in.clone()}
                                    disabled={*zoom >= 3.0}
                                >
                                    { "+ Zoom in" }
                                </button>
                                <button type="button" onclick={reset_zoom.clone()}>
                                    { "Reset" }
                                </button>
                            </div>
                            <div
                                class="viewer-stage"
                                tabindex="0"
                                role="region"
                                aria-label="Scrollable artwork image"
                            >
                                <img
                                    class="viewer-image"
                                    src={image_url}
                                    alt={item.title.clone()}
                                    referrerpolicy="no-referrer"
                                    style={format!("width: {}%", *zoom * 100.0)}
                                />
                            </div>
                        </>
                    }
                }
            };

            html! {
                <>
                    <h1 class="viewer-heading">{ &item.title }</h1>
                    <p>
                        { item.artist_display
                                .as_deref()
                                .unwrap_or("Artist information unavailable") }
                    </p>
                    <p class="artwork-origin">
                        <strong>
                            {
                                format!(
                                "Origin: {}", item.place_of_origin
                                    .as_deref()
                                    .unwrap_or("Origin unavailable")
                                )
                            }
                        </strong>
                    </p>
                    { image }
                </>
            }
        }
    };

    html! {
        <section>
            <Link<Route> to={Route::Home}>{ "← Back to gallery" }</Link<Route>>
            { content }
        </section>
    }
}

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
