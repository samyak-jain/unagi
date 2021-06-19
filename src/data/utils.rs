use kv::*;
use rand::{distributions::Alphanumeric, Rng};
use reqwest::header::ETAG;

use crate::db::Database;

use super::bridge::AnimeMap;

// Takes a URL and uses the ETAG header from the response to check if any
// new changes need to be fetched
pub async fn request_and_cache(url: &str) -> anyhow::Result<Option<String>> {
    let response = reqwest::get(url).await?;

    // Get the ETAG header. This header is refreshed for every change to the static resource
    let headers = response.headers();
    let etag = headers.get(ETAG).cloned();

    // Try to get file name for URL
    // If that is not possible, automatically generate a random string
    let name = response
        .url()
        .path_segments()
        .and_then(|segments| segments.last())
        .and_then(|name| {
            if name.is_empty() {
                None
            } else {
                Some(name.to_owned())
            }
        })
        .unwrap_or(
            rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(7)
                .map(char::from)
                .collect::<String>(),
        );

    // Get the directory in which the cached file will be saved
    let mut data_directory = dirs::data_dir().expect("No data directory");
    data_directory.push(&name);

    // Get the directory in which the Key Value will store the configuration
    let config_directory = dirs::config_dir().expect("No config direcotry");

    // Create the KV Store
    let cfg = Config::new(config_directory);
    let store = Store::new(cfg)?;

    // If the ETAG already exists in the KV store, then read the file from the file system
    let cache_bucket = store.bucket::<String, String>(Some("cache"))?;
    if let Ok(Some(value)) = cache_bucket.get(&name) {
        if let Some(ref etag_value) = etag {
            if etag_value.to_str()? == value {
                // No change in ETAG.
                // This means it is not necesarry to fetch this resource again
                return Ok(None);
            }
        }
    };

    // Fetch the data from the server
    let response_text = response.text().await?;

    // Set the new key and value in the KV store
    if let Some(etag_value) = etag {
        cache_bucket.set(name, etag_value.to_str()?);
    };

    Ok(Some(response_text))
}
