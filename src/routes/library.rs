use crate::errors::ApiResponse;
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
                match show.fetch_anime() {
                    Ok(_) => {
                        db::shows::create(c, show, new_library_id).unwrap();
                    }
                    Err(error) => error!("{}", error.to_string()),
                }
            }
            new_library_id
        })
        .await;

    Ok(json!({ "library": new_library_id }))
}
