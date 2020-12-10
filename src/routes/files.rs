use crate::{
    errors::{ApiResponse, FileResponse},
    SETTINGS,
};
use shared_child::SharedChild;
use std::{
    collections::HashMap,
    fs,
    path::PathBuf,
    sync::{Arc, Mutex},
    thread::spawn,
};

use rocket::response::NamedFile;
use rocket_contrib::uuid::Uuid;

use crate::{db, handlers::transcode::start_transcoding};

lazy_static! {
    static ref PROCESS: Mutex<HashMap<String, Arc<SharedChild>>> = Mutex::new(HashMap::new());
}

#[rocket::post("/transcode/<id>")]
pub async fn transcode(id: Uuid, conn: db::Conn) -> ApiResponse {
    let episode_result = conn
        .run(move |c| db::episodes::fetch(c, id.into_inner()))
        .await?;

    let pid = id.into_inner().to_string();
    let mut transcoding_path = PathBuf::from(SETTINGS.read()?.get_str("TRANSCODING")?);
    transcoding_path.push(pid.clone());

    fs::create_dir(transcoding_path.clone())?;

    let is_hw_enabled = SETTINGS.read()?.get_bool("HW_ACCEL")?;

    let mut command = start_transcoding(
        PathBuf::from(episode_result),
        PathBuf::from(transcoding_path),
        is_hw_enabled,
    )?;
    let result = SharedChild::spawn(&mut command)?;
    let child_arc = Arc::new(result);
    let child_arc_clone = child_arc.clone();

    spawn(move || child_arc_clone.wait().unwrap());

    PROCESS.lock()?.insert(pid, child_arc);

    Ok(json!({
        "status": "success",
    }))
}

#[rocket::post("/transcode/<id>")]
pub async fn stop_transcoding(id: Uuid) -> ApiResponse {
    let pid = id.into_inner().to_string();
    match PROCESS.lock()?.get(&pid) {
        Some(child) => {
            child.kill()?;
            Ok(json!({
                "status": "success"
            }))
        }
        None => Ok(json!({
            "status": "failed",
            "reason": "trancoding process not found",
        })),
    }
}

#[rocket::get("/file/<id>")]
pub async fn serve(id: Uuid) -> FileResponse {
    let pid = id.into_inner().to_string();
    let mut transcoding_path = PathBuf::from(SETTINGS.read()?.get_str("TRANSCODING")?);
    transcoding_path.push(pid.clone());

    Ok(NamedFile::open(transcoding_path).await?)
}
