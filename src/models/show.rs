use crate::models::library;
use crate::schema::shows;
use serde::Serialize;

#[derive(Identifiable, Queryable, Serialize, Associations)]
#[belongs_to(library::Library)]
#[table_name = "shows"]
pub struct Show {
    id: i32,
    library_id: i32,
    title: String,
    image: String,
    file_path: String,
}
