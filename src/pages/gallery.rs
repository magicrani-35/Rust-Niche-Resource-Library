use yew::prelude::*;
use yew_router::prelude::Link;
use web_sys::{HtmlDialogElement, HtmlInputElement};
use wasm_bindgen_futures::spawn_local;

use crate::api::fetch_artworks;
use crate::components::{ArtworkCard, PaginationControls};
use crate::models::GalleryPage;
use crate::routes::Route;

#[component]
pub(crate) fn Gallery() -> Html {
    let artworks = use_state(|| None::<Result<GalleryPage, String>>);

    let search_text = use_state(String::new);
    let submitted_search = use_state(String::new);

    let page = use_state(|| 1_u32);

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
        let page = page.clone();

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();

            page.set(1);
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

        use_effect_with(
            ((*submitted_search).clone(), *page),
            move |(query, page_number)| {
                let query = query.clone();
                let page_number = *page_number;

                artworks.set(None);

                spawn_local(async move {
                    let result = fetch_artworks(&query, page_number).await;
                    artworks.set(Some(result));
                });

                || ()
            }
        );
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
                Some(Ok(result)) if result.artworks.is_empty() => html! {
                    <p>
                        { "No artworks were found." }
                    </p>
                },
                Some(Ok(result)) => {
                    let current_page = result.pagination.current_page;
                    let page_limit = result.pagination.limit.max(1);

                    let last_page = if submitted_search.is_empty() {
                        result.pagination.total_pages
                    } else {
                        result
                            .pagination
                            .total_pages
                            .min(10_000 / page_limit)
                    };

                    let on_page_change = {
                        let page = page.clone();

                        Callback::from(move |selected_page: u32| {
                            page.set(selected_page);
                        })
                    };

                    html! {
                        <>
                            <p class="search-summary" role="status">
                                {
                                    format!(
                                        "Page {} of {} browsable pages · {} matching artworks",
                                        current_page,
                                        last_page,
                                        result.pagination.total
                                    )
                                }
                            </p>

                            <div class="gallery">
                                {
                                    for result.artworks.iter().map(|artwork| {
                                        html! {
                                            <ArtworkCard
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
              }}
        </section>
    }
}