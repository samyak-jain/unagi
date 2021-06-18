// use crate::{
//     api::bridge::{generate_anime_list, get_season},
//     errors::ApiResponse,
//     handlers::montage,
// };
// use rand::prelude::IteratorRandom;
// use rocket::response::status;
use rocket::{serde::json::Json, State};
use serde::Deserialize;
use validator::Validate;

use crate::{
    api::bridge::generate_anime_list,
    db::{library, Database},
};

use super::ApiResult;

#[derive(Deserialize, Validate)]
pub struct Library {
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(length(min = 1))]
    pub location: String,
}

#[post("/library", data = "<library>")]
pub async fn add_library(db: &State<Database>, library: Json<Library>) -> ApiResult<()> {
    let inner_library = library.into_inner();
    let library_id = library::add(db, &inner_library).await?;
    let library_directory =
        crate::handlers::files::Library::read(inner_library.location, library_id);
    let anime_list = generate_anime_list().unwrap();
    Ok(())
    /*
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
                        db::shows::create(&c, show, new_library_id, series_id).unwrap();
                    }
                    Err(error) => error!("{}", error.to_string()),
                }
            }

            let chosen_shows: Vec<String> = library
                .shows
                .into_iter()
                .filter_map(|show| show.cover_image)
                .choose_multiple(&mut rand::thread_rng(), 3);

            montage::combine(chosen_shows, format!("./media/library_{}", new_library_id));

            new_library_id
        })
        .await;

    if new_library_id == -1 {
        Err(status::NotFound("Couldn't find library"))?
    } else {
        Ok(json!({
            "status": "success",
            "library": new_library_id
        }))
    }

    */
}

// #[post("/update/library/<id>/<force>")]
// pub async fn update_library(id: i32, force: bool, conn: db::Conn) -> ApiResponse {
//     conn.run(move |c| {
//         let db_lib = db::library::get(&c, id).unwrap();
//         let mut library = Library::new(db_lib.location, db_lib.id);
//         library.read_library().unwrap();
//         let anime_list = generate_anime_list().unwrap();
//         for show in &mut library.shows {
//             if force && db::shows::exists(&c, &show.path).unwrap() {
//                 continue;
//             }
//             let series_id = show.search_anime().unwrap();
//             let season_series_id =
//                 get_season(series_id, show.season.clone(), &anime_list).unwrap_or(1);
//             match show.fetch_anime(season_series_id) {
//                 Ok(_) => {
//                     db::shows::create(&c, show, db_lib.id, series_id).unwrap();
//                 }
//                 Err(error) => error!("{}", error.to_string()),
//             }
//         }
//
//         let chosen_shows: Vec<String> = library
//             .shows
//             .into_iter()
//             .filter_map(|show| show.cover_image)
//             .choose_multiple(&mut rand::thread_rng(), 3);
//
//         montage::combine(chosen_shows, format!("./media/library_{}", db_lib.id));
//     })
//     .await;
//
//     Ok(json!({
//         "status": "success",
//     }))
// }
//
// #[get("/library/<id>")]
// pub async fn get(id: i32, conn: db::Conn) -> ApiResponse {
//     let library = conn.run(move |c| db::library::get(c, id)).await?;
//
//     Ok(json!(library))
// }
//
// #[get("/library")]
// pub async fn get_all(conn: db::Conn) -> ApiResponse {
//     Ok(json!(conn.run(|c| db::library::get_all(c)).await?))
// }
