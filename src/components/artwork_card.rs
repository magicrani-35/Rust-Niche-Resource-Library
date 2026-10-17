use yew::prelude::*;
use yew_router::prelude::Link;

use crate::models::Artwork;
use crate::routes::Route;

#[derive(Properties, PartialEq)]
pub(crate) struct ArtworkCardProps {
    pub(crate) artwork: Artwork,
}

#[component]
pub(crate) fn ArtworkCard(props: &ArtworkCardProps) -> Html {
    let image_failed = use_state(|| false);

    let on_image_error = {
        let image_failed = image_failed.clone();

        Callback::from(move |_: Event| {
            image_failed.set(true);
        })
    };

    let museum_url = format!(
        "https://www.artic.edu/artworks/{}",
        props.artwork.id
    );

    let image = match &props.artwork.image_id {
        Some(image_id) if !*image_failed => {
            let image_url =
                format!("https://www.artic.edu/iiif/2/{image_id}/full/843,/0/default.jpg");

            html! {
                <Link<Route>
                    to={Route::Artwork {
                    id: props.artwork.id,
                    }}
                    classes="artwork-image-link"
                >
                    <img
                        class="artwork-image"
                        src={image_url}
                        alt={format!("View {}", props.artwork.title)}
                        loading="lazy"
                        referrerpolicy="no-referrer"
                        onerror={on_image_error}
                    />
                </Link<Route>>
            }
        },
        _ => {
            let message = if *image_failed {
                "Image couldn't load"
            } else {
                "Image unavailable"
            };

            html! {
                <div class="artwork-image-placeholder">
                    <div class="artwork-image-fallback">
                        <p>{ message }</p>
                        <a
                            href={museum_url}
                            target="_blank"
                            rel="noopener noreferrer"
                        >
                            { "View artwork at the museum ↗" }
                        </a>
                    </div>
                </div>
            }
        },
    };

    html! {
        <article class="artwork-card">
            { image }
            <div class="artwork-info">
                <h2>{ &props.artwork.title }</h2>
                <p>{ &props.artwork.artist }</p>
                <p>{ &props.artwork.date }</p>
                <p class="artwork-origin">
                    <strong>
                        { format!("Origin: {}", props.artwork.place_of_origin) }
                    </strong>
                </p>
            </div>
        </article>
    }
}