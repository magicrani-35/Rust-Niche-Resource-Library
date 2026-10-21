use yew::prelude::*;
use yew_router::prelude::Link;
use wasm_bindgen_futures::spawn_local;

use crate::api::fetch_artwork;
use crate::models::ApiArtwork;
use crate::routes::Route;
use crate::components::ArtworkImage;
use crate::favorites::{load_favorites, set_favorite};

#[derive(Properties, PartialEq)]
pub(crate) struct ArtworkViewerProps {
    pub(crate) id: u32,
}

#[component]
pub(crate) fn ArtworkViewer(props: &ArtworkViewerProps) -> Html {
    let artwork = use_state(|| None::<Result<ApiArtwork, String>>);
    let zoom = use_state(|| 1.0_f64);

    let is_favorite = use_state(|| false);
    let favorite_error = use_state(|| None::<String>);

    {
        let is_favorite = is_favorite.clone();
        let favorite_error = favorite_error.clone();

        use_effect_with(props.id, move |id| {
            match load_favorites() {
                Ok(ids) => {
                    is_favorite.set(ids.contains(id));
                    favorite_error.set(None);
                },
                Err(error) => {
                    favorite_error.set(Some(error));
                },
            }

            || ()
        });
    }

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

    let toggle_favorite = {
        let is_favorite = is_favorite.clone();
        let favorite_error = favorite_error.clone();
        let artwork_id = props.id;

        Callback::from(move |_| {
            let should_save = !*is_favorite;

            match set_favorite(artwork_id, should_save) {
                Ok(()) => {
                    is_favorite.set(should_save);
                    favorite_error.set(None);
                },
                Err(error) => {
                    favorite_error.set(Some(error));
                },
            }
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
                                <ArtworkImage
                                    key={image_id.clone()}
                                    artwork_id={item.id}
                                    image_id={Some(image_id.clone())}
                                    title={item.title.clone()}
                                    classes={classes!("viewer-image")}
                                    style={format!("Width: {}%;", *zoom * 100.0)}
                                    link_to_viewer={false}
                                />
                            </div>
                        </>
                    }
                }
            };

            html! {
                <>
                    <h1 class="viewer-heading">{ &item.title }</h1>
                    <button
                        type="button"
                        class="favorite-button"
                        onclick={toggle_favorite}
                        aria-pressed={(*is_favorite).to_string()}
                    >
                        {
                            if *is_favorite {
                                "♥ Remove from favorites"
                            } else {
                                "♡ Save to favorites"
                            }
                        }
                    </button>
                    if let Some(error) = &*favorite_error {
                        <p role="alert">{ error }</p>
                    }

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
