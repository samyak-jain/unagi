#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate simple_error;

extern crate dotenv;
extern crate reqwest;
extern crate serde;

use dotenv::dotenv;

mod api;
mod db;
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
        .mount("/", routes![hello, routes::library::add_library])
        .attach(db::Conn::fairing())
        .launch()
        .await
        .unwrap();
}
