use diesel::RunQueryDsl;
use rocket_contrib::databases::diesel::PgConnection;

use crate::{models::show::Show, schema::shows};

#[derive(Insertable, Debug)]
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
    show: crate::handlers::files::Show,
    library_id: i32,
) -> Result<i32, diesel::result::Error> {
    let new_show = &ShowNew {
        title: show.name,
        library_id,
        image: None,
        file_path: show.path,
        description: None,
        cover_image: None,
        banner_image: None,
    };

    info!("{:#?}", new_show);
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
