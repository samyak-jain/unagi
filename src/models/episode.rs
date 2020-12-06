use crate::models::show;
use crate::schema::episodes;
use serde::Serialize;
use uuid::Uuid;

#[derive(Identifiable, Queryable, Serialize, Associations)]
#[belongs_to(show::Show)]
#[table_name = "episodes"]
pub struct Episode {
    pub id: i32,
    pub show_id: i32,
    pub name: Option<String>,
    pub thumbnail: Option<String>,
    pub file_path: String,
    pub locator_id: Uuid,
}
