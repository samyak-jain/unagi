use crate::schema::library;
use serde::Serialize;

#[derive(Identifiable, Queryable, Serialize)]
#[table_name = "library"]
pub struct Library {
    pub id: i32,
    pub name: String,
    pub location: String,
}
