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

#[derive(Deserialize, Clone, PartialEq)]
pub(crate) struct Pagination {
    pub(crate) total: u32,
    pub(crate) current_page: u32,
    pub(crate) total_pages: u32,
    pub(crate) limit: u32,
}

#[derive(Deserialize)]
pub(crate) struct PaginatedArtworkResponse {
    pub(crate) data: Vec<ApiArtwork>,
    pub(crate) pagination: Pagination,
}

#[derive(Clone, PartialEq)]
pub(crate) struct ArtworkPage {
    pub(crate) artworks: Vec<ApiArtwork>,
    pub(crate) pagination: Pagination,
}

#[derive(Clone, PartialEq)]
pub(crate) struct GalleryPage {
    pub(crate) artworks: Vec<Artwork>,
    pub(crate) pagination: Pagination,
}

#[derive(Deserialize, Clone, PartialEq)]
pub(crate) struct OriginBucket {
    pub(crate) key: String,
    pub(crate) doc_count: u32,
}

#[derive(Deserialize)]
pub(crate) struct OriginAggregation {
    pub(crate) buckets: Vec<OriginBucket>,
    pub(crate) sum_other_doc_count: u32,
}

#[derive(Deserialize)]
pub(crate) struct OriginAggregations {
    pub(crate) origins: OriginAggregation,
}

#[derive(Deserialize)]
pub(crate) struct OriginsResponse {
    pub(crate) aggregations: OriginAggregations,
}