use yew::prelude::*;

use crate::components::ArtworkImage;
use crate::models::Artwork;

#[derive(Properties, PartialEq)]
pub(crate) struct ArtworkCardProps {
    pub(crate) artwork: Artwork,
}

#[component]
pub(crate) fn ArtworkCard(props: &ArtworkCardProps) -> Html {
    html! {
        <article class="artwork-card">
            <ArtworkImage key={props.artwork.image_id.clone().unwrap_or_default()}
                artwork_id={props.artwork.id}
                image_id={props.artwork.image_id.clone()}
                title={props.artwork.title.clone()}
                classes={classes!("artwork-image")}
            />
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