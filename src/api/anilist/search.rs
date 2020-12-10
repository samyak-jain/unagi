use graphql_client::*;
use regex::Regex;
use std::error::Error;

use crate::handlers::files::Show;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/api/anilist/schema.graphql",
    query_path = "src/api/anilist/query.graphql",
    response_derives = "Debug"
)]
struct GetAnime;

impl Show {
    fn update_episode(
        &mut self,
        number: String,
        name: String,
        thumbnail: Option<String>,
    ) -> Option<()> {
        let ep_index = self
            .episodes
            .iter()
            .position(|ep| ep.number == Some(number.clone()))?;

        self.episodes[ep_index].name = name;
        self.episodes[ep_index].thumbnail = thumbnail;

        Some(())
    }

    pub fn fetch_anime(&mut self) -> Result<(), Box<dyn Error>> {
        let request_body = GetAnime::build_query(get_anime::Variables {
            name: self.name.to_owned(),
        });
        let client = reqwest::blocking::Client::new();
        let res = client
            .post("https://graphql.anilist.co/")
            .json(&request_body)
            .send()?;
        let response_body: Response<get_anime::ResponseData> = res.json()?;

        if response_body.errors.is_some() || response_body.data.is_none() {
            error!("GraphQLQuery not successfull");
            return Ok(());
        }

        // name, cover_image, banner_image, description, episode name, episode thumbnail

        let response_data: get_anime::ResponseData =
            response_body.data.ok_or("Invalid GraphQL Response")?;

        let response_anime = response_data.media.ok_or("no anime found")?;

        if let Some(anime_name) = response_anime.title {
            self.name = anime_name.english.unwrap_or(
                anime_name
                    .romaji
                    .unwrap_or(anime_name.native.unwrap_or(self.name.to_owned())),
            )
        }

        self.description = response_anime.description;
        self.banner_image = response_anime.banner_image;
        self.cover_image = response_anime
            .cover_image
            .ok_or("No cover found")?
            .extra_large;

        let pattern = Regex::new(r".*Episode.(\d+).*")?;
        if let Some(episode_list) = response_anime.streaming_episodes {
            for episode in episode_list {
                match episode {
                    Some(ep) => {
                        if let Some(title) = ep.title {
                            if let Some(ep_num) = pattern.captures(title.as_str()) {
                                self.update_episode(String::from(&ep_num[0]), title, ep.thumbnail);
                            }
                        }
                    }
                    None => continue,
                }
            }
        }
        Ok(())
    }
}
