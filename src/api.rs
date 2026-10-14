use gloo_net::http::Request;

use crate::models::{
    ApiArtwork,
    Artwork,
    ArtworkDetailResponse,
    ArtworkResponse,
};

pub(crate) async fn fetch_artworks(query: &str) -> Result<Vec<Artwork>, String> {
    let fields = "id,title,artist_display,date_display,image_id,place_of_origin";

    let url = if query.is_empty() {
        format!(
            "https://api.artic.edu/api/v1/artworks?limit=6&fields={fields}"
        )
    } else {
        let encoded_query = urlencoding::encode(query);

        format!(
            "https://api.artic.edu/api/v1/artworks/search?q={encoded_query}&limit=12&fields={fields}"
        )
    };

    let response = Request::get(&url)
        .send()
        .await
        .map_err(|error| error.to_string())?;

    if !response.ok() {
        return Err(format!("The museum returned HTTP {}.", response.status()));
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
            place_of_origin: item
                .place_of_origin
                .unwrap_or_else(|| String::from("Origin unavailable")),
        })
        .collect();

    Ok(artworks)
}

pub(crate) async fn fetch_artwork(id: u32) -> Result<ApiArtwork, String> {
    let url = format!(
        "https://api.artic.edu/api/v1/artworks/{id}?fields=id,title,artist_display,date_display,image_id,place_of_origin"
    );

    let response = Request::get(&url)
        .send()
        .await
        .map_err(|error| error.to_string())?;

    if !response.ok() {
        return Err(format!("The museum returned HTTP {}.", response.status()));
    }

    let body = response
        .json::<ArtworkDetailResponse>()
        .await
        .map_err(|error| error.to_string())?;

    Ok(body.data)
}

pub(crate) async fn fetch_origin_artworks(
    origin: &str,
) -> Result<Vec<ApiArtwork>, String> {
    let fields = "id,title,artist_display,date_display,image_id,place_of_origin";
    let encoded_origin = urlencoding::encode(origin);
    let url = format!(
        "https://api.artic.edu/api/v1/artworks/search?limit=12&fields={fields}&query%5Bmatch_phrase%5D%5Bplace_of_origin%5D={encoded_origin}"
    );

    let response = Request::get(&url)
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

    Ok(body.data)
}