use rocket_contrib::databases::diesel;

pub mod library;

#[database("denki")]
pub struct Conn(diesel::PgConnection);
