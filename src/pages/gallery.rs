use yew::prelude::*;
use yew_router::prelude::Link;
use web_sys::{HtmlDialogElement, HtmlInputElement};
use wasm_bindgen_futures::spawn_local;

use crate::api::fetch_artworks;
use crate::components::ArtworkCard;
use crate::models::Artwork;
use crate::routes::Route;

#[component]
pub(crate) fn Gallery() -> Html {
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

            if let Some(Ok(items)) = &*artworks {
                <p role="status">
                    {
                        format!(
                            "Received {} artwork records for '{}'.",
                            items.len(),
                            &*submitted_search
                        )
                    }
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