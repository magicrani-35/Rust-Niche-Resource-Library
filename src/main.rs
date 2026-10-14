mod api;
mod models;
mod routes;
mod components;

use crate::components::ArtworkCard;

use crate::api::{
    fetch_artwork,
    fetch_artworks,
    fetch_origin_artworks,
};

use crate::models::{
    ApiArtwork,
    Artwork
};

use crate::routes::Route;

use yew::prelude::*;
use yew_router::prelude::*;

use web_sys::HtmlInputElement;
use web_sys::HtmlDialogElement;

use wasm_bindgen_futures::spawn_local;

#[component]
fn Gallery() -> Html {
    let artworks = use_state(|| None::<Result<Vec<Artwork>, String>>);

    let search_text = use_state(String::new);
    let submitted_search = use_state(String::new);

    let on_search_input = {
        let search_text = search_text.clone();

        Callback::from(move |event: InputEvent| {
            let input: HtmlInputElement = event.target_unchecked_into();
            search_text.set(input.value());
        })
    };

    let on_search_submit = {
        let search_text = search_text.clone();
        let submitted_search = submitted_search.clone();

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();

            submitted_search.set(search_text.trim().to_owned());
        })
    };

    let origins_dialog = use_node_ref();

    let open_origins = {
        let origins_dialog = origins_dialog.clone();

        Callback::from(move |_| {
            if let Some(dialog) = origins_dialog.cast::
            <HtmlDialogElement>() {
                let _ = dialog.show_modal();
            }
        })
    };

    {
        let artworks = artworks.clone();

        use_effect_with((*submitted_search).clone(), move |query| {
            let query = query.clone();

            artworks.set(None);

            spawn_local(async move {
                let result = fetch_artworks(&query).await;
                artworks.set(Some(result));
            });

            || ()
        });
    }

    html! {
        <section>
            <div class="gallery-intro">
                <p class="eyebrow">{ "A little space for discovery" }</p>
                <h1>{ "Find art that stays with you." }</h1>
                <p class="intro-description">
                    { "Explore the gallery, follow your curiosity, and collect your favorites." }
                </p>
            </div>

            <div class="origin-browser">
                <h2>
                    { "Explore by origin" }
                </h2>

                <div class="origin-links">
                    <Link<Route>
                        to={Route::Origin {
                            origin: String::from("Japan"),
                        }}
                    >
                        { "Japan" }
                    </Link<Route>>

                    <Link<Route>
                        to={Route::Origin {
                            origin: String::from("France"),
                        }}
                    >
                        { "France" }
                    </Link<Route>>

                    <Link<Route>
                        to={Route::Origin {
                            origin: String::from("India"),
                        }}
                    >
                        { "India" }
                    </Link<Route>>

                    <button
                        type="button"
                        class="origins-more"
                        onclick={open_origins}
                    >
                        { "View all origins ->" }
                    </button>
                </div>
            </div>

            <dialog
                ref={origins_dialog.clone()}
                class="origins-dialog"
                aria-labelledby="origins-dialog-title"
            >
                <div class="origins-dialog-header">
                    <h2 id="origins-dialog-title">{ "Explore by origin" }</h2>

                    <form method="dialog">
                        <button type="submit" class="dialog-close">
                            { "Close" }
                        </button>
                    </form>
                </div>

                <p class="origins-dialog-description">
                    { "Choose an origin to explore its artwork." }
                </p>

                <ul class="origins-list">
                    {
                        for ["Japan", "France", "India"].into_iter().map(|origin| {
                        let origins_dialog = origins_dialog.clone();

                        let close_dialog = Callback::from(move |_| {
                            if let Some(dialog) = origins_dialog.cast::
                            <HtmlDialogElement>() {
                                dialog.close();
                            }
                        });

                        html! {
                            <li key={origin} onclick={close_dialog}>
                                <Link<Route>
                                    to={Route::Origin {
                                        origin:origin.to_owned(),
                                    }}
                                >
                                    { origin }
                                </Link<Route>>
                            </li>
                            }
                        })
                    }
                </ul>
            </dialog>

            <form class="search-form" onsubmit={on_search_submit}>
                <label for="artwork-search">
                    { "Search the collection" }
                </label>

                <div class="search-fields">
                    <input
                        id="artwork-search"
                        name="q"
                        type="search"
                        placeholder="Try flowers, cats, or an artist"
                        value={(*search_text).clone()}
                        oninput={on_search_input}
                    />

                    <button type="submit">{ "Search" }</button>
                </div>
            </form>

            if !submitted_search.is_empty() {
                <p class="search-summary" role="status">
                    { format!("You searched for: {}", &*submitted_search) }
                </p>
            }

            { match &*artworks {
                None => html! {
                    <p role="status">{ "Loading artwork..." }</p>
                },
                Some(Err(error)) => html! {
                    <p role="alert">
                        { format!("Could not load artwork: {error}") }
                    </p>
                },
                Some(Ok(items)) if items.is_empty() => html! {
                    <p>
                        { "No artworks were found." }
                    </p>
                },
                Some(Ok(items)) => html! {
                    <div class="gallery">
                        {
                            for items.iter().map(|artwork| {
                            html! {
                                <ArtworkCard key={artwork.id.to_string()} artwork={artwork.clone()} />
                            }
                        })
                        }
                    </div>
                },
            } }
        </section>
    }
}

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

#[derive(Properties, PartialEq)]
struct ArtworkRowProps {
    artwork: ApiArtwork,
}

#[component]
fn ArtworkRow(props: &ArtworkRowProps) -> Html {
    let item = &props.artwork;

    let image = match &item.image_id {
        Some(image_id) => {
            let image_url = format!(
                "https://www.artic.edu/iiif/2/{image_id}/full/843,/0/default.jpg"
            );

            html! {
                <Link<Route>
                    to={Route::Artwork { id: item.id }}
                    classes="artwork-row-image-link"
                >
                    <img
                        class="artwork-row-image"
                        src={image_url}
                        alt={format!("View {}", item.title)}
                        loading="lazy"
                        referrerpolicy="no-referrer"
                    />
                </Link<Route>>
            }
        },
        None => html! {
            <div class="artwork-row-placeholder">
                { "Image unavailable" }
            </div>
        },
    };

    html! {
        <article class="artwork-row">
            { image }

            <div class="artwork-row-info">
                <h2>
                    <Link<Route>
                        to={Route::Artwork {
                            id: item.id }}>
                                { &item.title }
                    </Link<Route>>
                </h2>

                <p>
                    {
                        item.artist_display
                            .as_deref()
                            .unwrap_or("Artist information unavailable")
                    }
                </p>

                <p>
                    {
                        item.date_display
                            .as_deref()
                            .unwrap_or("Date unavailable")
                    }
                </p>

                <p class="artwork-origin">
                    <strong>
                        {
                            format!(
                            "Origin: {}",
                            item.place_of_origin
                                .as_deref()
                                .unwrap_or("Origin unavailable")
                            )
                        }
                    </strong>
                </p>
            </div>
        </article>
    }
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
