#[macro_use]
extern crate rocket;

#[macro_use]
extern crate dotenv_codegen;

#[macro_use]
extern crate anyhow;

use dotenv::dotenv;
use rocket::fs::FileServer;
use std::{fs, path::PathBuf};

mod api;
mod db;
// mod errors;
mod handlers;
// mod models;
mod routes;
// mod schema;

#[get("/hello/<age>")]
fn hello(age: u8) -> String {
    format!("Hello, {}", age)
}

#[rocket::main]
async fn main() {
    dotenv().ok();

    // Check if the transcoding directory is created
    let transcoding_path = match std::env::var("TRANSCODING") {
        Ok(path) => PathBuf::from(path),
        Err(_) => dirs::data_dir().expect(
            "No default data directory in the OS. Please set the TRANSCODING environment variable",
        ),
    };
    fs::create_dir_all(&transcoding_path).unwrap();

    rocket::build()
        .mount(
            "/",
            routes![
                hello,
                // routes::library::add_library,
                // routes::files::serve_thumbnail,
                // routes::files::transcode,
                // routes::library::get,
                // routes::library::get_all,
                // routes::shows::get,
                // routes::episodes::get
            ],
        )
        .mount("/file", FileServer::from(transcoding_path))
        .attach(db::stage_database())
        .launch()
        .await
        .unwrap();
}
