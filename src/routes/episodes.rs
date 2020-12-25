use crate::{db, errors::ApiResponse};

#[get("/episodes/<id>")]
pub async fn get(id: i32, conn: db::Conn) -> ApiResponse {
    let episodes = conn.run(move |c| db::episodes::get(c, id)).await?;

    Ok(json!(episodes))
}
