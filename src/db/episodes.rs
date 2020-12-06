use crate::{models::episode::Episode, schema::episodes};
use diesel::RunQueryDsl;
use rocket_contrib::databases::diesel::PgConnection;
use uuid::Uuid;

#[derive(Insertable)]
#[table_name = "episodes"]
pub struct EpisodeNew {
    name: Option<String>,
    show_id: i32,
    thumbnail: Option<String>,
    file_path: String,
    locator_id: Uuid,
}

pub fn create(
    conn: &PgConnection,
    file_path: String,
    show_id: i32,
) -> Result<i32, diesel::result::Error> {
    let locator_id = Uuid::new_v4();

    let new_episode = &EpisodeNew {
        show_id,
        file_path,
        locator_id,
        name: None,
        thumbnail: None,
    };

    let result_id = diesel::insert_into(episodes::table)
        .values(new_episode)
        .get_result::<Episode>(conn)?
        .id;

    Ok(result_id)
}

pub fn fetch(conn: &PgConnection, episode_uuid: Uuid) -> Result<String, diesel::result::Error> {
    use self::episodes::dsl::*;
    use crate::diesel::ExpressionMethods;
    use crate::diesel::QueryDsl;

    let episode: Episode = episodes.filter(locator_id.eq(episode_uuid)).first(conn)?;

    Ok(episode.file_path)
}
