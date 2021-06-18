use serde::{Deserialize, Serialize};

const BASE_API: &str = "https://kitsu.io/api/edge";

pub mod search;

#[derive(Serialize, Deserialize, Debug)]
pub struct AnimeImage {
    small: String,
    large: String,
    original: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnimeAttributes {
    synopsis: String,
    #[serde(rename(deserialize = "canonicalTitle"))]
    cononical_title: String,
    #[serde(rename(deserialize = "posterImage"))]
    poster_image: AnimeImage,
    #[serde(rename(deserialize = "coverImage"))]
    cover_image: Option<AnimeImage>,
    #[serde(rename(deserialize = "episodeCount"))]
    episode_count: i32,
    #[serde(skip_deserializing)]
    season_number: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Anime {
    id: String,
    attributes: AnimeAttributes,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnimeList {
    data: Vec<Anime>,
}
