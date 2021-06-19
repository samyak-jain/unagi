use rocket::State;

use crate::route::library::Library;

use super::Database;

struct LibraryInsertResult {
    id: i32,
}

pub async fn add(db: &State<Database>, library: &Library) -> Result<i32, sqlx::Error> {
    // Check if library exists
    if let Some(result) = sqlx::query_as!(
        LibraryInsertResult,
        "SELECT id FROM library WHERE location = $1",
        library.location,
    )
    .fetch_optional(&**db)
    .await?
    {
        return Ok(result.id);
    }

    // Insert library if it doesn't exist
    let result = sqlx::query_as!(
        LibraryInsertResult,
        "INSERT INTO library(name, location) VALUES($1, $2) RETURNING id",
        library.name,
        library.location
    )
    .fetch_one(&**db)
    .await?;

    Ok(result.id)
}

// pub fn get(conn: &PgConnection, library_id: i32) -> Result<Library, diesel::result::Error> {
//     use self::library::dsl::*;
//     use crate::diesel::QueryDsl;
//
//     library.find(library_id).get_result::<Library>(conn)
// }
//
// pub fn get_all(conn: &PgConnection) -> Result<Vec<Library>, diesel::result::Error> {
//     use self::library::dsl::*;
//
//     library.load::<Library>(conn)
// }
//
// pub fn exists(conn: &PgConnection, path: &str) -> Result<bool, diesel::result::Error> {
//     use self::library::dsl::*;
//     use crate::diesel::ExpressionMethods;
//     use crate::diesel::QueryDsl;
//     use diesel::dsl::exists;
//     use diesel::select;
//
//     select(exists(library.filter(location.eq(path)))).get_result(conn)
// }
