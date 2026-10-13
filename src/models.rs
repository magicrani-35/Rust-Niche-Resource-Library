use serde::Deserialize;

#[derive(Clone, PartialEq)]
pub(crate) struct Artwork {
    pub(crate) id: u32,
    pub(crate) title: String,
    pub(crate) artist: String,
    pub(crate) date: String,
    pub(crate) image_id: Option<String>,
    pub(crate) place_of_origin: String,
}

#[derive(Deserialize, Clone, PartialEq)]
pub(crate) struct ApiArtwork {
    pub(crate) id: u32,
    pub(crate) title: String,
    pub(crate) artist_display: Option<String>,
    pub(crate) date_display: Option<String>,
    pub(crate) image_id: Option<String>,
    pub(crate) place_of_origin: Option<String>,
}

#[derive(Deserialize)]
pub(crate) struct ArtworkResponse {
    pub(crate) data: Vec<ApiArtwork>,
}

#[derive(Deserialize)]
pub(crate) struct ArtworkDetailResponse {
    pub(crate) data: ApiArtwork,
}