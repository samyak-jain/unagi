use graphql_client::*;
use std::error::Error;

use crate::api::anilist::search::get_anime::GetAnimeMediaStreamingEpisodes;
use crate::handlers::files::Show;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/api/anilist/schema.graphql",
    query_path = "src/api/anilist/query.graphql",
    response_derives = "Debug"
)]
struct GetAnime;

async fn fetch_anime(mut show: Show) -> Result<(), Box<dyn Error>> {
    let request_body = GetAnime::build_query(get_anime::Variables { name: show.name });
    let client = reqwest::Client::new();
    let res = client.post("/").json(&request_body).send().await?;
    let response_body: Response<get_anime::ResponseData> = res.json().await?;

    if response_body.errors.is_some() || response_body.data.is_none() {
        error!("GraphQLQuery not successfull");
        return Ok(());
    }

    // name, cover_image, banner_image, description, episode name, episode thumbnail

    let response_data: get_anime::ResponseData =
        response_body.data.ok_or("Invalid GraphQL Response")?;

    let response_anime = response_data.media.ok_or("no anime found")?;

    show.name = response_anime
        .title
        .ok_or("no title found")?
        .english
        .ok_or("no english title found")?;

    show.description = response_anime.description;
    show.banner_image = response_anime.banner_image;
    show.cover_image = response_anime
        .cover_image
        .ok_or("No cover found")?
        .extra_large;

    let episode_number = response_anime.episodes.unwrap_or(0) as usize;
    let episode_list = response_anime.streaming_episodes.ok_or("No episodes")?;
    if show.episodes.len() != episode_number || episode_number != episode_list.len() {
        warn!(
            "Mismatch of length, Filesystem length: {}, GraphQl len: {}, GraphQl Vector Length {}",
            show.episodes.len(),
            episode_number,
            episode_list.len()
        );
        return Ok(());
    }

    episode_list
        .into_iter()
        .enumerate()
        .map(|(i, mut val)| {
            show.episodes[i].name = val.take().unwrap().title.unwrap();
            show.episodes[i].thumbnail = val.unwrap().thumbnail;
        })
        .for_each(drop);

    Ok(())
}
