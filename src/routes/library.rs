use crate::{api::bridge::get_season, errors::ApiResponse};
use rocket_contrib::json::Json;
use serde::Deserialize;
use validator::Validate;

use crate::{db, handlers::files::Library};

#[derive(Deserialize, Validate)]
pub struct NewLibrary {
    #[validate(length(min = 1))]
    name: String,
    #[validate(length(min = 1))]
    location: String,
}

#[post("/library", format = "json", data = "<new_library>")]
pub async fn add_library(new_library: Json<NewLibrary>, conn: db::Conn) -> ApiResponse {
    let new_library = new_library.into_inner();
    let library_path = new_library.location.clone();
    let new_library_id = conn
        .run(|c| {
            let new_library_id =
                db::library::create(&c, new_library.name, new_library.location).unwrap();
            let mut library = Library::new(library_path, new_library_id);
            library.read_library().unwrap();
            for mut show in library.shows {
                if db::shows::exists(&c, &show.path).unwrap() {
                    continue;
                }
                let series_id = show.search_anime().unwrap();
                let season_series_id = get_season(series_id, show.season.clone()).unwrap_or(1);
                match show.fetch_anime(season_series_id) {
                    Ok(_) => {
                        db::shows::create(&c, show, new_library_id).unwrap();
                    }
                    Err(error) => error!("{}", error.to_string()),
                }
            }
            new_library_id
        })
        .await;

    Ok(json!({ "library": new_library_id }))
}
