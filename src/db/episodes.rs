use crate::{
    models::{episode::Episode, show::Show},
    schema::{episodes, shows},
};
use diesel::RunQueryDsl;
use rocket_contrib::databases::diesel::PgConnection;
use uuid::Uuid;

#[derive(Insertable, AsChangeset)]
#[table_name = "episodes"]
#[changeset_options(treat_none_as_null = "true")]
pub struct EpisodeNew {
    name: Option<String>,
    show_id: i32,
    episode_number: Option<i32>,
    thumbnail: Option<String>,
    file_path: String,
    locator_id: Uuid,
}

pub fn create(
    conn: &PgConnection,
    episode: &crate::handlers::files::Episode,
    new_show_id: i32,
) -> Result<i32, diesel::result::Error> {
    use self::episodes::dsl::*;
    use crate::diesel::ExpressionMethods;
    use crate::diesel::QueryDsl;

    let new_locator_id = Uuid::new_v4();

    let new_episode = &EpisodeNew {
        show_id: new_show_id,
        file_path: episode.path.clone(),
        episode_number: episode.number,
        locator_id: new_locator_id,
        name: Some(episode.name.clone()),
        thumbnail: None,
    };

    let result_id = if exists(conn, &episode.path)? {
        let target = episodes.filter(file_path.eq(episode.path.clone()));
        diesel::update(target)
            .set(new_episode)
            .get_result::<Episode>(conn)?
            .show_id
    } else {
        diesel::insert_into(episodes)
            .values(new_episode)
            .get_result::<Episode>(conn)?
            .show_id
    };

    Ok(result_id)
}

pub fn fetch(conn: &PgConnection, episode_uuid: Uuid) -> Result<String, diesel::result::Error> {
    use self::episodes::dsl::*;
    use crate::diesel::ExpressionMethods;
    use crate::diesel::QueryDsl;

    let episode: Episode = episodes.filter(locator_id.eq(episode_uuid)).first(conn)?;

    Ok(episode.file_path)
}

pub fn get(conn: &PgConnection, show_id: i32) -> Result<Vec<Episode>, diesel::result::Error> {
    use self::shows::dsl::*;
    use crate::diesel::BelongingToDsl;
    use crate::diesel::QueryDsl;

    let result_show: Show = shows.find(show_id).get_result::<Show>(conn)?;
    Episode::belonging_to(&result_show).load::<Episode>(conn)
}

fn exists(conn: &PgConnection, path: &str) -> Result<bool, diesel::result::Error> {
    use self::episodes::dsl::*;
    use crate::diesel::ExpressionMethods;
    use crate::diesel::QueryDsl;
    use diesel::dsl::exists;
    use diesel::select;

    select(exists(episodes.filter(file_path.eq(path)))).get_result(conn)
}
