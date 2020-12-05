use diesel::RunQueryDsl;
use rocket_contrib::databases::diesel::PgConnection;

use crate::{models::show::Show, schema::shows};

#[derive(Insertable)]
#[table_name = "shows"]
pub struct ShowNew {
    title: String,
    library_id: i32,
    image: Option<String>,
    file_path: String,
    description: Option<String>,
    cover_image: Option<String>,
    banner_image: Option<String>,
}

pub fn create(
    conn: &PgConnection,
    name_string: String,
    location_string: String,
    library_id: i32,
) -> Result<i32, diesel::result::Error> {
    let new_show = &ShowNew {
        title: name_string,
        library_id,
        image: None,
        file_path: location_string,
        description: None,
        cover_image: None,
        banner_image: None,
    };

    let result_id = diesel::insert_into(shows::table)
        .values(new_show)
        .get_result::<Show>(conn)?
        .id;

    Ok(result_id)
}
