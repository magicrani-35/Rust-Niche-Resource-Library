use yew::prelude::*;
use yew_router::prelude::Link;

use crate::models::ApiArtwork;
use crate::routes::Route;
use crate::components::ArtworkImage;

#[derive(Properties, PartialEq)]
pub(crate) struct ArtworkRowProps {
    pub(crate) artwork: ApiArtwork,
}

#[component]
pub(crate) fn ArtworkRow(props: &ArtworkRowProps) -> Html {
    let item = &props.artwork;

    html! {
        <article class="artwork-row">
            <div class="artwork-row-media">
                <ArtworkImage
                    key={item.image_id.clone().unwrap_or_default()}
                    artwork_id={item.id}
                    image_id={item.image_id.clone()}
                    title={item.title.clone()}
                    classes={classes!("artwork-row-image")}
                />
            </div>

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