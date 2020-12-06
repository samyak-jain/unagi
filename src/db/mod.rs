use rocket_contrib::databases::diesel;

pub mod episodes;
pub mod library;
pub mod shows;

#[database("denki")]
pub struct Conn(diesel::PgConnection);
