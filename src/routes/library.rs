use crate::{
    api::bridge::{generate_anime_list, get_season},
    errors::ApiResponse,
    handlers::montage,
};
use rand::seq::SliceRandom;
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
            if db::library::exists(&c, &library_path).unwrap() {
                return -1;
            }
            let new_library_id =
                db::library::create(&c, new_library.name, library_path.clone()).unwrap();
            let mut library = Library::new(library_path, new_library_id);
            library.read_library().unwrap();
            let anime_list = generate_anime_list().unwrap();
            for show in &mut library.shows {
                let series_id = show.search_anime().unwrap();
                let season_series_id =
                    get_season(series_id, show.season.clone(), &anime_list).unwrap_or(series_id);
                match show.fetch_anime(season_series_id) {
                    Ok(_) => {
                        db::shows::create(&c, show, new_library_id).unwrap();
                    }
                    Err(error) => error!("{}", error.to_string()),
                }
            }
            let chosen_shows: Vec<String> = library
                .shows
                .choose_multiple(&mut rand::thread_rng(), 3)
                .cloned()
                .map(|show| show.path)
                .collect();

            montage::combine(chosen_shows, format!("./media/{}.jpeg", new_library_id));

            new_library_id
        })
        .await;

    if new_library_id == -1 {
        Ok(json!({
            "status": "failure",
            "message": "library already exists",
        }))
    } else {
        Ok(json!({
            "status": "success",
            "library": new_library_id
        }))
    }
}

#[post("/update/library/<id>/<force>")]
pub async fn update_library(id: i32, force: bool, conn: db::Conn) -> ApiResponse {
    conn.run(move |c| {
        let (db_lib, _, _) = db::library::fetch_library(&c, id).unwrap();
        let mut library = Library::new(db_lib.location, db_lib.id);
        library.read_library().unwrap();
        let anime_list = generate_anime_list().unwrap();
        for show in &mut library.shows {
            if force && db::shows::exists(&c, &show.path).unwrap() {
                continue;
            }
            let series_id = show.search_anime().unwrap();
            let season_series_id =
                get_season(series_id, show.season.clone(), &anime_list).unwrap_or(1);
            match show.fetch_anime(season_series_id) {
                Ok(_) => {
                    db::shows::create(&c, show, db_lib.id).unwrap();
                }
                Err(error) => error!("{}", error.to_string()),
            }
        }

        let chosen_shows: Vec<String> = library
            .shows
            .choose_multiple(&mut rand::thread_rng(), 3)
            .cloned()
            .map(|show| show.path)
            .collect();

        montage::combine(chosen_shows, format!("./media/{}.jpeg", db_lib.id));
    })
    .await;

    Ok(json!({
        "status": "success",
    }))
}
