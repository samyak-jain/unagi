use rocket::{
    fairing::{self, AdHoc},
    Build, Rocket,
};

use sqlx::postgres::PgPoolOptions;

// pub mod episodes;
pub mod library;
// pub mod shows;

pub(crate) type Database = sqlx::PgPool;

async fn init_db(rocket: Rocket<Build>) -> fairing::Result {
    let pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(dotenv!("DATABASE_URL"))
        .await
    {
        Ok(db) => db,
        Err(e) => {
            error!("Could not create database pool, {}", e);
            return Err(rocket);
        }
    };

    if let Err(e) = sqlx::migrate!("./migrations").run(&pool).await {
        error!("Failed to run SQLx migrations, {}", e);
        return Err(rocket);
    };

    Ok(rocket.manage(pool))
}

pub fn stage_database() -> AdHoc {
    AdHoc::on_ignite("SQLx Stage", |rocket| async {
        rocket.attach(AdHoc::try_on_ignite("SQLx Database", init_db))
    })
}
