use yew_router::prelude::Routable;

#[derive(Clone, Routable, PartialEq)]
pub(crate) enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/artwork/:id")]
    Artwork { id: u32 },
    #[at("/origins/:origin")]
    Origin { origin: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}