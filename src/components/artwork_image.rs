use yew::prelude::*;
use yew_router::prelude::Link;

use crate::routes::Route;

#[derive(Properties, PartialEq)]
pub(crate) struct ArtworkImageProps {
    pub(crate) artwork_id: u32,
    pub(crate) image_id: Option<String>,
    pub(crate) title: String,

    #[prop_or_default]
    pub(crate) classes: Classes,

    #[prop_or_default]
    pub(crate) style: AttrValue,

    #[prop_or(true)]
    pub(crate) link_to_viewer: bool,
}

#[component]
pub(crate) fn ArtworkImage(props: &ArtworkImageProps) -> Html {
    let image_failed = use_state(|| false);

    let on_image_error = {
        let image_failed = image_failed.clone();

        Callback::from(move |_: Event| {
            image_failed.set(true);
        })
    };

    match &props.image_id {
        Some(image_id) if !*image_failed => {
            let image_url = //String::from("/missing-artwork-test.jpg"
                format!(
                "https://www.artic.edu/iiif/2/{image_id}/full/843,/0/default.jpg"
            );

            let image = html! {
                <img
                    class={props.classes.clone()}
                    style={props.style.clone()}
                    src={image_url}
                    alt={props.title.clone()}
                    loading="lazy"
                    referrerpolicy="no-referrer"
                    onerror={on_image_error}
                />
            };

            if props.link_to_viewer {
                html! {
                    <Link<Route>
                        to={Route::Artwork {
                            id: props.artwork_id,
                        }}
                        classes="artwork-image-link"
                    >
                        { image }
                    </Link<Route>>
                }
            } else {
                image
            }
        },
        _ => {
            let museum_url = format!(
                "https://www.artic.edu/artworks/{}",
                props.artwork_id
            );

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
    }
}