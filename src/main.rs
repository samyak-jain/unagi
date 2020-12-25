#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate simple_error;

extern crate graphql_client;

#[macro_use]
extern crate lazy_static;

extern crate anitomy;
extern crate config;
extern crate dotenv;
extern crate quick_xml;
extern crate reqwest;
extern crate serde;
extern crate static_http_cache;

use config::Config;
use dotenv::dotenv;
use std::sync::RwLock;

lazy_static! {
    static ref SETTINGS: RwLock<Config> = RwLock::new(Config::default());
}

mod api;
mod db;
mod errors;
mod handlers;
mod models;
mod routes;
mod schema;

#[get("/hello/<age>")]
fn hello(age: u8) -> String {
    format!("Hello, {}", age)
}

#[rocket::main]
async fn main() {
    dotenv().ok();

    rocket::ignite()
        .mount(
            "/",
            routes![
                hello,
                routes::library::add_library,
                routes::files::serve,
                routes::files::serve_thumbnail,
                routes::library::get,
                routes::library::get_all,
                routes::shows::get,
                routes::episodes::get
            ],
        )
        .attach(db::Conn::fairing())
        .launch()
        .await
        .unwrap();
}
