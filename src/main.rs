use yew::prelude::*;
use yew_router::prelude::*;

use gloo_net::http::Request;
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[derive(Clone, PartialEq)]
struct Artwork {
    id: u32,
    title: String,
    artist: String,
    date: String,
    image_id: Option<String>,
}

#[derive(Deserialize)]
struct ArtworkResponse {
    data: Vec<ApiArtwork>,
}

#[derive(Deserialize)]
struct ApiArtwork {
    id: u32,
    title: String,
    artist_display: Option<String>,
    date_display: Option<String>,
    image_id: Option<String>,
}

#[derive(Properties, PartialEq)]
struct ArtworkCardProps {
    artwork: Artwork,
}

#[component]
fn ArtworkCard(props: &ArtworkCardProps) -> Html {
    let image = match &props.artwork.image_id {
        Some(image_id) => {
            let image_url = format!(
                "https://www.artic.edu/iiif/2/{image_id}/full/843,/0/default.jpg"
            );

            html! {
                <img
                    class="artwork-image"
                    src={image_url}
                    alt={props.artwork.title.clone()}
                    laoding="lazy"
                />
            }
        },
        None => html! {
            <div class="artwork-image-placeholder">
                { "Image unavailable" }
            </div>
        },
    };

    html! {
        <article class="artwork-card">
            { image }

            <div class="artwork-info">
                <h2>{ &props.artwork.title }</h2>
                <p>{ &props.artwork.artist }</p>
                <p>{ &props.artwork.date }</p>
            </div>
        </article>
    }
}

async fn fetch_artworks() -> Result<Vec<Artwork>, String> {
    let response = Request::get(
        "https://api.artic.edu/api/v1/artworks?\
        limit=6&fields=id,title,artist_display,date_display,image_id",
    )
        .send()
        .await
        .map_err(|error| error.to_string())?;

    if !response.ok() {
        return Err(format!(
            "The museum returned HTTP {}.",
            response.status()
        ));
    }

    let body = response
        .json::<ArtworkResponse>()
        .await
        .map_err(|error| error.to_string())?;

    let artworks = body
        .data
        .into_iter()
        .map(|item| Artwork {
            id: item.id,
            title: item.title,
            artist: item
                .artist_display
                .unwrap_or_else(|| String::from("Artist information unavailable")),
            date: item
                .date_display
                .unwrap_or_else(|| String::from("Date unavailable")),
            image_id: item.image_id,
        })
        .collect();

    Ok(artworks)
}

#[component]
fn Gallery() ->Html {
    let artworks = use_state(|| None::<Result<Vec<Artwork>, String>>);

    {
        let artworks = artworks.clone();

        use_effect_with((), move |_| {
            spawn_local(async move {
                let result = fetch_artworks().await;
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

            {
                match &*artworks {
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
            }
            }
        </section>
    }
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Gallery /> },
        Route::About => html! {
            <section>
                <h1>
                    { "About this museum" }
                </h1>
                <p>
                    { "A small art discovery app built with Rust and Yew." }
                </p>
            </section>
        },
        Route::NotFound => html! {
            <section>
                <h1>{ "Page not found" }</h1>
                <Link<Route> to={Route::Home}>
                    { "Return to the gallery" }
                </Link<Route>>
            </section>
        },
    }
}

#[component]
fn App() -> Html {
      html! {
          <BrowserRouter>
            <header class="site-header">
                <p class="site-brand">
                    { "The Little Museum" }
                </p>
                <nav class="site-nav" aria-label="Main Navigation">
                    <Link<Route> to={Route::Home}>
                        { "Gallery" }
                    </Link<Route>>

                    { " | " }

                    <Link<Route> to={Route::About}>
                        { "About" }
                    </Link<Route>>
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