use yew::prelude::*;
use yew_router::prelude::Link;

use crate::models::ApiArtwork;
use crate::routes::Route;

#[derive(Properties, PartialEq)]
pub(crate) struct ArtworkRowProps {
    pub(crate) artwork: ApiArtwork,
}

#[component]
pub(crate) fn ArtworkRow(props: &ArtworkRowProps) -> Html {
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