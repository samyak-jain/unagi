use crate::db;

#[get("/library/<id>")]
pub async fn get_library(id: i32, conn: db::Conn) {
    conn.run(move |c| {
        let new_id = id.clone();
        let (lib, shows, episodes) = db::library::fetch_library(&c, new_id).unwrap();
        println!("{:#?}", lib);
        println!("{:#?}", shows);
        println!("{:#?}", episodes);
    })
    .await;
}
