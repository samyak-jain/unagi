use diesel::{select, RunQueryDsl};
use rocket_contrib::databases::diesel::PgConnection;

use crate::{models::show::Show, schema::shows};

#[derive(Insertable, Debug)]
#[table_name = "shows"]
pub struct ShowNew {
    title: String,
    library_id: i32,
    file_path: String,
    description: Option<String>,
    cover_image: Option<String>,
    banner_image: Option<String>,
}

pub fn create(
    conn: &PgConnection,
    show: crate::handlers::files::Show,
    library_id: i32,
) -> Result<i32, diesel::result::Error> {
    let new_show = &ShowNew {
        title: show.name,
        library_id,
        file_path: show.path,
        description: show.description,
        cover_image: show.cover_image,
        banner_image: show.banner_image,
    };

    let result_id = diesel::insert_into(shows::table)
        .values(new_show)
        .get_result::<Show>(conn)?
        .id;

    show.episodes
        .into_iter()
        .map(|episode| super::episodes::create(conn, episode, result_id))
        .collect::<Result<Vec<String>, diesel::result::Error>>()?;

    Ok(result_id)
}

pub fn exists(conn: &PgConnection, path: &str) -> Result<bool, diesel::result::Error> {
    use self::shows::dsl::*;
    use crate::diesel::ExpressionMethods;
    use crate::diesel::QueryDsl;
    use diesel::dsl::exists;

    select(exists(shows.filter(file_path.eq(path)))).get_result::<bool>(conn)
}
