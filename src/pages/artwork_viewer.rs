use yew::prelude::*;
use yew_router::prelude::Link;
use wasm_bindgen_futures::spawn_local;

use crate::api::fetch_artwork;
use crate::models::ApiArtwork;
use crate::routes::Route;

#[derive(Properties, PartialEq)]
pub(crate) struct ArtworkViewerProps {
    pub(crate) id: u32,
}

#[component]
pub(crate) fn ArtworkViewer(props: &ArtworkViewerProps) -> Html {
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
