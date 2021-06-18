use graphql_client::*;
use std::error::Error;

use crate::handlers::files::Show;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/api/anilist/schema.graphql",
    query_path = "src/api/anilist/search/query.graphql",
    response_derives = "Debug"
)]
struct GetAnime;

impl Show {
    pub fn search_anime(&self) -> Result<i64, Box<dyn Error>> {
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
            bail!("GraphQLQuery not successfull");
        }

        let response_data: get_anime::ResponseData =
            response_body.data.ok_or("Invalid GraphQL Response")?;

        let response_anime = response_data.media.ok_or("no anime found")?;
        Ok(response_anime.id)
    }
}
