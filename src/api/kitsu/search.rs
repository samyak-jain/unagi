use std::error::Error;

use serde_json::Value;

use super::{Anime, AnimeList, BASE_API};

pub async fn search_show(title: String) -> Result<AnimeList, Box<dyn Error>> {
    let complete_url = format!("{}/anime?filter%5Btext%5D={}", BASE_API, title);
    let resp = reqwest::get(&complete_url).await?;
    if resp.status().is_success() {
        let body = resp.text().await?;
        Ok(serde_json::from_str(&body)?)
    } else {
        bail!("Request not successfull")
    }
}

pub async fn get_season_number(anime: Anime) -> Result<i64, Box<dyn Error>> {
    let episode_url = format!("{}/episodes/{}", BASE_API, anime.id);
    let resp = reqwest::get(&episode_url).await?;
    if resp.status().is_success() {
        let body = resp.text().await?;
        let episodes: Value = serde_json::from_str(&body)?;
        Ok(episodes["data"][0]["attributes"]["seasonNumber"]
            .as_i64()
            .unwrap_or(1))
    } else {
        bail!("Request not successfull")
    }
}

pub async fn get_show_season(anime: Anime, season: i32) {
    let current_season = get_season_number(anime);
}
