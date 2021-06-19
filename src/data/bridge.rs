use core::fmt;
use std::{borrow::Cow, path::PathBuf};

use rocket::futures::TryStreamExt;
use serde::{Deserialize, Deserializer};
use serde_json::Value;

use crate::db::{Database, get_db_handle};
use std::convert::TryFrom;

use super::utils::request_and_cache;

const BRIDGE_BASE_URL: &str = "https://relations.yuna.moe/api/ids";

#[derive(Debug)]
enum AnimeProvider {
    Anilist,
    AniDB,
    Kitsu,
}

impl fmt::Display for AnimeProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.to_string().to_lowercase())
    }
}

// Convert from one anime provider to another
// Eg: Anilist to AniDB
async fn convert_anime_provider(
    source: AnimeProvider,
    dest: AnimeProvider,
    show_id: i32,
) -> anyhow::Result<i32> {
    let anime_response = reqwest::get(&format!(
        "{}?source={}&id={}",
        BRIDGE_BASE_URL, source, show_id
    ))
    .await?;

    let not_found_error = anyhow!("Unable to find destionation, {}", dest);

    if anime_response.status().is_success() {
        let body = anime_response.text().await?;
        let mapping: Value = serde_json::from_str(&body)?;
        i32::try_from(mapping[dest.to_string()].as_i64().ok_or(not_found_error)?).map_err(|_| anyhow!("Overflow Error"))
    } else {
        Err(not_found_error)
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct AnimeMap {
    // The ID for the AniDB database
    pub anidbid: i32,

    // The ID for the TVDB database
    #[serde(deserialize_with = "string_as_i32")]
    pub tvdbid: Option<i32>,

    // The season number according to the TVDB database
    #[serde(default)]
    #[serde(deserialize_with = "string_as_i32")]
    pub defaulttvdbseason: Option<i32>,

    // Number to add to the AniDB episode number
    // to correspond to the respective TVDB episode number
    // in the defaulttvdbseason
    pub episodeoffset: Option<i32>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct AnimeList {
    anime: Vec<AnimeMap>,
}

// Certain fields like tvdbid and defaulttvdbseason are represented as string in the XML
// but not always. To deal with that, we have written a custom deserializer
fn string_as_i32<'de, D>(deserializer: D) -> Result<Option<i32>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<Cow<'de, str>> = Deserialize::deserialize(deserializer)?;
    if let Some(st) = s {
        match st.parse::<i32>() {
            Ok(num) => Ok(Some(num)),
            Err(_) => Ok(None),
        }
    } else {
        Ok(None)
    }
}

pub async fn generate_anime_list() -> anyhow::Result<()> {
    let anime_list_string =
        request_and_cache("https://github.com/ScudLee/anime-lists/raw/master/anime-list-full.xml")
            .await?;
    if let Some(anime_list_unwrapped) = anime_list_string {
        let anime_list: AnimeList = quick_xml::de::from_str(&anime_list_unwrapped)?;
    }
    Ok(())
}

// Store the parsed anime details into a database
async fn store_in_database(anime_list: Vec<AnimeMap>, database: &Database) -> anyhow::Result<()> {
    for anime in anime_list {
        sqlx::query!(
            "INSERT INTO anime (anidb, tvdb, season, episode_offset) VALUES ($1, $2, $3, $4)",
            anime.anidbid,
            anime.tvdbid,
            anime.defaulttvdbseason,
            anime.episodeoffset
        )
        .execute(database)
        .await?;
    }
    Ok(())
}

// Convert an AniDB ID to a TVDB ID
async fn anidb_to_tvdb(anidb_id: i32, database: &Database) -> anyhow::Result<i32> {
    let result = sqlx::query!("SELECT tvdb FROM anime WHERE anidb = $1", anidb_id)
        .fetch_one(database)
        .await?;
    result.tvdb.ok_or(anyhow!("TVDB not found"))
}

// Function gets the season number and returns the AniDB ID
async fn season_to_anidb(tvdb: i32, season: i32, database: &Database) -> anyhow::Result<i32> {
    let anime_list: Vec<AnimeMap> = sqlx::query_as!(AnimeMap, "SELECT anidb as anidbid, tvdb as tvdbid, season as defaulttvdbseason, episode_offset as episodeoffset FROM anime WHERE tvdb = $1 AND (season = $2 OR season = NULL) ORDER BY episode_offset", tvdb, season).fetch(database).try_collect::<Vec<_>>().await?;

    let anime_list_index = season as usize;
    if anime_list_index >= anime_list.len() {
        anime_list
            .first()
            .ok_or(anyhow!("Not available"))
            .map(|anime| anime.anidbid)
    } else {
        Ok(anime_list[anime_list_index].anidbid)
    }
}

pub async fn get_season(anilist_id: i32, season_number: i32) -> anyhow::Result<i32> {
    let anidbid = convert_anime_provider(AnimeProvider::Anilist, AnimeProvider::AniDB, anilist_id).await?;
    let database = get_db_handle().await?;
    let tvdbid = anidb_to_tvdb(anidbid, &database).await?;
    let correct_anidbid = season_to_anidb(season_number, tvdbid, &database).await?;
    convert_anime_provider(AnimeProvider::AniDB, AnimeProvider::Anilist, correct_anidbid).await
}
