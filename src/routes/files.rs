use crate::{errors::FileResponse, SETTINGS};
use std::{path::PathBuf, thread::spawn};

use rocket::response::NamedFile;
use rocket_contrib::uuid::Uuid;

use crate::{db, handlers::transcode::start_transcoding};

#[rocket::get("/file/<id>")]
pub async fn serve(id: Uuid, conn: db::Conn) -> FileResponse {
    let episode_result = conn
        .run(move |c| db::episodes::fetch(c, id.into_inner()))
        .await?;

    let transcoding_path = SETTINGS.read()?.get_str("TRANSCODING")?;

    spawn(move || {
        start_transcoding(
            PathBuf::from(episode_result),
            PathBuf::from(transcoding_path),
        )
        .unwrap();
    });

    Ok(NamedFile::open(transcoding_path).await?)
}
