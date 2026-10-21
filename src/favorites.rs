use gloo_storage::{errors::StorageError, LocalStorage, Storage};

const FAVORITES_KEY: &str = "little-museum,favorite-ids.v1";

pub(crate) fn load_favorites() -> Result<Vec<u32>, String>
{
    match LocalStorage::get(FAVORITES_KEY) {
        Ok(ids) => Ok(ids),
        Err(StorageError::KeyNotFound(_)) => Ok(Vec::new()),
        Err(error) => Err(format!("Could not read favorites: {error}")),
    }
}

pub(crate) fn set_favorite(
    artwork_id: u32,
    should_save: bool,
) -> Result<(), String> {
    let mut ids = load_favorites()?;

    if should_save {
        if !ids.contains(&artwork_id) {
            ids.push(artwork_id);
        }
    } else {
        ids.retain(|id| *id != artwork_id);
    }

    LocalStorage::set(FAVORITES_KEY, ids)
        .map_err(|error| format!("Could not save to favorites: {error}"))
}