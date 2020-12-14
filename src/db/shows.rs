use diesel::{select, RunQueryDsl};
use rocket_contrib::databases::diesel::PgConnection;

use crate::{models::show::Show, schema::shows};

#[derive(Insertable, Debug, AsChangeset)]
#[table_name = "shows"]
#[changeset_options(treat_none_as_null = "true")]
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
    show: &mut crate::handlers::files::Show,
    current_library_id: i32,
) -> Result<i32, diesel::result::Error> {
    use self::shows::dsl::*;
    use crate::diesel::ExpressionMethods;
    use crate::diesel::QueryDsl;

    let new_show = &ShowNew {
        title: show.name.clone(),
        library_id: current_library_id,
        file_path: show.path.clone(),
        description: show.description.clone(),
        cover_image: show.cover_image.clone(),
        banner_image: show.banner_image.clone(),
    };

    let result_id = if exists(conn, &show.path)? {
        let target = shows.filter(file_path.eq(show.path.clone()));
        diesel::update(target)
            .set(new_show)
            .get_result::<Show>(conn)?
            .id
    } else {
        diesel::insert_into(shows)
            .values(new_show)
            .get_result::<Show>(conn)?
            .id
    };

    show.episodes
        .iter()
        .map(|episode| super::episodes::create(conn, episode, result_id))
        .collect::<Result<Vec<i32>, diesel::result::Error>>()?;

    Ok(result_id)
}

pub fn exists(conn: &PgConnection, path: &str) -> Result<bool, diesel::result::Error> {
    use self::shows::dsl::*;
    use crate::diesel::ExpressionMethods;
    use crate::diesel::QueryDsl;
    use diesel::dsl::exists;

    select(exists(shows.filter(file_path.eq(path)))).get_result(conn)
}
