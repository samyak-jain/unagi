use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct Library {
    pub id: i32,
    pub name: String,
    pub location: String,
}
