use serde::Deserialize;
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
struct AnimeMap {
    anidbid: i64,
    tvdbid: i64,
    defaulttvdbseason: i64,
}

#[derive(Debug, Deserialize, PartialEq)]
struct AnimeList {
    anime: Vec<AnimeMap>,
}

fn generate_anime_list() -> Option<AnimeList> {
    let res = reqwest::blocking::get(
        "https://github.com/ScudLee/anime-lists/raw/master/anime-list-full.xml",
    )
    .ok()?;

    if res.status().is_success() {
        let body = res.text().ok()?;
        let anime_list: AnimeList = quick_xml::de::from_str(&body).ok()?;
        Some(anime_list)
    } else {
        None
    }
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

pub fn get_season(id: i64, season_number: i64) -> Option<i64> {
    let anidbid = anilist_to_anidb(id)?;
    let anime_list = generate_anime_list()?;
    let tvdbid = anidb_to_tvdb(anidbid, &anime_list)?;
    let correct_anidbid = season_to_anidb(season_number, tvdbid, &anime_list)?;
    Some(anidb_to_anilist(correct_anidbid)?)
}
