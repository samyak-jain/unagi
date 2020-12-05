use std::thread;

use rocket::{http::Status, response::status::Custom};
use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;
use validator::Validate;

use crate::{db, handlers::library::Library};

#[derive(Deserialize, Validate)]
pub struct NewLibrary {
    #[validate(length(min = 1))]
    name: String,
    #[validate(length(min = 1))]
    location: String,
}

#[post("/library", format = "json", data = "<new_library>")]
pub async fn add_library(
    new_library: Json<NewLibrary>,
    conn: db::Conn,
) -> Result<JsonValue, Custom<String>> {
    let new_library = new_library.into_inner();
    let library_path = new_library.location.clone();
    let new_library_id = conn
        .run(|c| db::library::create(&c, new_library.name, new_library.location))
        .await;

    thread::spawn(move || {
        let mut library = Library::new(library_path);
        library.read_library().unwrap();
    });

    match new_library_id {
        Ok(new_library_id) => Ok(json!({ "library": new_library_id })),
        Err(e) => Err(Custom(Status::InternalServerError, e.to_string())),
    }
}
