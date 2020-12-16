use std::{borrow::Cow, path::PathBuf};
use std::{error::Error, io::Read};

use serde::{Deserialize, Deserializer};
use serde_json::Value;

const BRIDGE_BASE_URL: &str = "https://relations.yuna.moe/api/ids";

fn converter(id: i64, source: &str, dest: &str) -> Option<i64> {
    let resp =
        reqwest::blocking::get(&format!("{}?source={}&id={}", BRIDGE_BASE_URL, source, id)).ok()?;

    if resp.status().is_success() {
        let body = resp.text().ok()?;
        let mapping: Value = serde_json::from_str(&body).ok()?;
        mapping[dest].as_i64()
    } else {
        None
    }
}

fn anilist_to_anidb(id: i64) -> Option<i64> {
    converter(id, "anilist", "anidb")
}

fn anidb_to_anilist(id: i64) -> Option<i64> {
    converter(id, "anidb", "anilist")
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct AnimeMap {
    anidbid: i64,
    #[serde(deserialize_with = "string_as_i64")]
    tvdbid: i64,
    #[serde(default)]
    #[serde(deserialize_with = "string_as_i64")]
    defaulttvdbseason: i64,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct AnimeList {
    anime: Vec<AnimeMap>,
}

fn string_as_i64<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<Cow<'de, str>> = Deserialize::deserialize(deserializer)?;
    if let Some(st) = s {
        match st.parse::<i64>() {
            Ok(num) => Ok(num),
            Err(_) => Ok(-10),
        }
    } else {
        Ok(-10)
    }
}

pub fn generate_anime_list() -> Result<AnimeList, Box<dyn Error>> {
    let mut cache =
        static_http_cache::Cache::new(PathBuf::from("./cache"), reqwest::blocking::Client::new())?;

    let mut res = cache.get(reqwest::Url::parse(
        "https://github.com/ScudLee/anime-lists/raw/master/anime-list-full.xml",
    )?)?;

    let mut body = String::new();
    res.read_to_string(&mut body)?;
    let mut anime_list: AnimeList = quick_xml::de::from_str(&body)?;
    anime_list.anime = anime_list
        .anime
        .into_iter()
        .filter(|anime| anime.defaulttvdbseason != -10 && anime.tvdbid != -10)
        .collect::<Vec<AnimeMap>>();
    Ok(anime_list)
}

fn anidb_to_tvdb(id: i64, anime_list: &AnimeList) -> Option<i64> {
    Some(
        anime_list
            .anime
            .iter()
            .filter(|anime| anime.anidbid == id)
            .collect::<Vec<&AnimeMap>>()
            .first()?
            .tvdbid,
    )
}

fn season_to_anidb(season: i64, tvdb_id: i64, anime_list: &AnimeList) -> Option<i64> {
    Some(
        anime_list
            .anime
            .iter()
            .filter(|anime| anime.defaulttvdbseason == season && anime.tvdbid == tvdb_id)
            .collect::<Vec<&AnimeMap>>()
            .first()?
            .anidbid,
    )
}

pub fn get_season(id: i64, season_number: i64, anime_list: &AnimeList) -> Option<i64> {
    let anidbid = anilist_to_anidb(id)?;
    let tvdbid = anidb_to_tvdb(anidbid, &anime_list)?;
    let correct_anidbid = season_to_anidb(season_number, tvdbid, &anime_list)?;
    Some(anidb_to_anilist(correct_anidbid)?)
}
