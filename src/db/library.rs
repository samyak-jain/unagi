use crate::schema::library;
use diesel::RunQueryDsl;
use rocket_contrib::databases::diesel::PgConnection;

use crate::models::library::Library;

#[derive(Insertable)]
#[table_name = "library"]
struct LibraryNew {
    name: String,
    location: String,
}

pub fn create(
    conn: &PgConnection,
    name_string: String,
    location_string: String,
) -> Result<i32, diesel::result::Error> {
    let new_library = &LibraryNew {
        name: name_string,
        location: location_string,
    };

    let result_id = diesel::insert_into(library::table)
        .values(new_library)
        .get_result::<Library>(conn)?
        .id;

    Ok(result_id)
}

pub fn get(conn: &PgConnection, library_id: i32) -> Result<Library, diesel::result::Error> {
    use self::library::dsl::*;
    use crate::diesel::QueryDsl;

    library.find(library_id).get_result::<Library>(conn)
}

pub fn get_all(conn: &PgConnection) -> Result<Vec<Library>, diesel::result::Error> {
    use self::library::dsl::*;

    library.load::<Library>(conn)
}

pub fn exists(conn: &PgConnection, path: &str) -> Result<bool, diesel::result::Error> {
    use self::library::dsl::*;
    use crate::diesel::ExpressionMethods;
    use crate::diesel::QueryDsl;
    use diesel::dsl::exists;
    use diesel::select;

    select(exists(library.filter(location.eq(path)))).get_result(conn)
}
