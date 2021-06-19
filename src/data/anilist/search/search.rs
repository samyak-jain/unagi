use graphql_client::*;
use std::error::Error;

use crate::services::files::ShowDetails;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/data/anilist/schema.graphql",
    query_path = "src/data/anilist/search/query.graphql",
    response_derives = "Debug"
)]
struct GetAnime;

impl ShowDetails {
    pub fn search_anime(&self) -> anyhow::Result<i64> {
        let request_body = GetAnime::build_query(get_anime::Variables {
            name: self.name.clone(),
        });
        let client = reqwest::blocking::Client::new();
        let res = client
            .post("https://graphql.anilist.co/")
            .json(&request_body)
            .send()?;
        let response_body: Response<get_anime::ResponseData> = res.json()?;

        if response_body.errors.is_some() || response_body.data.is_none() {
            anyhow!("GraphQLQuery not successfull");
        }

        let response_data: get_anime::ResponseData = response_body
            .data
            .ok_or(anyhow!("Invalid GraphQL Response"))?;

        let response_anime = response_data.media.ok_or(anyhow!("no anime found"))?;
        Ok(response_anime.id)
    }
}
