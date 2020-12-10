use crate::{
    models::{episode::Episode, show::Show},
    schema::library,
};
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

pub fn fetch_library(
    conn: &PgConnection,
    library_id: i32,
) -> Result<(Library, Show, Episode), diesel::result::Error> {
    use self::library::dsl::*;
    use crate::diesel::BelongingToDsl;
    use crate::diesel::QueryDsl;

    let result_library: Library = library.find(library_id).get_result::<Library>(conn)?;
    let result_shows: Show = Show::belonging_to(&result_library).first(conn)?;
    let result_episodes: Episode = Episode::belonging_to(&result_shows).first(conn)?;

    Ok((result_library, result_shows, result_episodes))
}
