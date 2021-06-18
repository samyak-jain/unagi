use crate::{db, errors::ApiResponse};

#[get("/shows/<id>")]
pub async fn get(id: i32, conn: db::Conn) -> ApiResponse {
    let shows = conn.run(move |c| db::shows::get(c, id)).await?;

    Ok(json!(shows))
}
