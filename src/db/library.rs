use crate::schema::library;
use diesel::RunQueryDsl;
use rocket_contrib::databases::diesel::PgConnection;

use crate::models::library::Library;

#[derive(Insertable)]
#[table_name = "library"]
struct LibraryInsert {
    name: String,
    location: String,
}

pub fn create(
    conn: &PgConnection,
    name_string: String,
    location_string: String,
) -> Result<i32, diesel::result::Error> {
    let new_library = &LibraryInsert {
        name: name_string,
        location: location_string,
    };

    let result_id = diesel::insert_into(library::table)
        .values(new_library)
        .get_result::<Library>(conn)?
        .id;

    Ok(result_id)
}
