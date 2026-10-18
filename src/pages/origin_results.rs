use yew::prelude::*;
use yew_router::prelude::Link;
use wasm_bindgen_futures::spawn_local;

use crate::api::fetch_origin_artworks;
use crate::components::{ArtworkRow, PaginationControls};
use crate::models::ArtworkPage;
use crate::routes::Route;

#[derive(Properties, PartialEq)]
pub(crate) struct OriginResultsProps {
    pub(crate) origin: String,
}

#[component]
pub(crate) fn OriginResults(props: &OriginResultsProps) -> Html {
    let artworks = use_state(|| None::
    <Result<ArtworkPage, String>>);

    let page = use_state(|| 1_u32);

    {
        let artworks = artworks.clone();

        use_effect_with(
            (props.origin.clone(), *page),
                move |(origin, page_number)| {
                    let origin = origin.clone();
                    let page_number = *page_number;

                    artworks.set(None);

                    spawn_local(async move {
                        let result = fetch_origin_artworks(&origin, page_number).await;

                        artworks.set(Some(result));
            });

            || ()
        },
        );
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
        Some(Ok(result)) if result.artworks.is_empty() => html! {
            <p>{ "No artworks were found for this origin." }</p>
        },
        Some(Ok(result)) => {
            let current_page = result.pagination.current_page;

            let page_limit = result.pagination.limit.max(1);

            let last_page = result
                .pagination
                .total_pages
                .min(10_000 / page_limit);

            let on_page_change = {
                let page = page.clone();

                Callback::from(move |selected_page: u32| {
                    page.set(selected_page);
                })
            };

             html! {
                 <>
                    <p class="origin-results-description">
                        {
                            format!(
                                "Page {} of {} browsable pages · {} matching artworks",
                                current_page,
                                last_page,
                                result.pagination.total
                            )
                        }
                    </p>

                    <div class="origin-results">
                        {
                            for result.artworks.iter().map(|artwork| {
                                html! {
                                    <ArtworkRow
                                        key={artwork.id.to_string()}
                                        artwork={artwork.clone()}
                                    />
                                }
                            })
                        }
                    </div>

                    if last_page > 1 {
                        <PaginationControls
                            current_page={current_page}
                            last_page={last_page}
                            on_page_change={on_page_change}
                         />
                    }
                 </>
             }
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

            { content }
        </section>
    }
}