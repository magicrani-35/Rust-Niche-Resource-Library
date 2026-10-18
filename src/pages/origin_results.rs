use yew::prelude::*;
use yew_router::prelude::Link;
use wasm_bindgen_futures::spawn_local;

use crate::api::fetch_origin_artworks;
use crate::components::ArtworkRow;
use crate::models::ApiArtwork;
use crate::route::Route;

#[derive(Properties, PartialEq)]
pub(crate) struct OriginResultsProps {
    pub(crate) origin: String,
}

#[component]
pub(crate) fn OriginResults(props: &OriginResultsProps) -> Html {
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