use crate::models::library;
use crate::schema::shows;
use serde::Serialize;

#[derive(Identifiable, Queryable, Serialize, Associations, Debug)]
#[belongs_to(library::Library)]
#[table_name = "shows"]
pub struct Show {
    pub id: i32,
    pub library_id: i32,
    pub title: String,
    pub file_path: String,
    pub description: Option<String>,
    pub cover_image: Option<String>,
    pub banner_image: Option<String>,
}
