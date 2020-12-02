#[macro_use] extern crate rocket;

#[get("/hello/<age>")]
fn hello(age: u8) -> String {
    format!("Hello, {}", age)
}

#[rocket::main]
async fn main() {
    rocket::ignite().mount("/", routes![hello]).launch().await.unwrap();
}
