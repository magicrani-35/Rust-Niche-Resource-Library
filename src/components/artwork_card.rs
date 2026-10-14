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
    let image = match &props.artwork.image_id {
        Some(image_id) => {
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
                    />
                </Link<Route>>
            }
        }
        None => html! { <div class="artwork-image-placeholder">{ "Image unavailable" }</div> },
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