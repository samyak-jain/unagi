use rocket::{
    http::Status,
    response::{status::Custom, NamedFile},
};
use rocket_contrib::uuid::Uuid;

use crate::db;

#[rocket::get("/file/<id>")]
pub async fn serve(id: Uuid, conn: db::Conn) -> Result<NamedFile, Custom<String>> {
    let episode_result = conn
        .run(move |c| db::episodes::fetch(c, id.into_inner()))
        .await;

    match episode_result {
        Ok(path) => match NamedFile::open(path).await {
            Ok(file) => Ok(file),
            Err(e) => Err(Custom(Status::InternalServerError, e.to_string())),
        },
        Err(e) => Err(Custom(Status::InternalServerError, e.to_string())),
    }
}
