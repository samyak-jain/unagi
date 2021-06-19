use rocket::{
    fairing::{self, AdHoc},
    Build, Rocket,
};

use sqlx::postgres::PgPoolOptions;

// pub mod episodes;
pub mod library;
// pub mod shows;

pub(crate) type Database = sqlx::PgPool;

pub async fn get_db_handle() -> anyhow::Result<Database> {
    let pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(dotenv!("DATABASE_URL"))
        .await
    {
        Ok(db) => db,
        Err(e) => {
            error!("Could not create database pool, {}", e);
            return Err(anyhow!("Could not create database pool"));
        }
    };

    if let Err(e) = sqlx::migrate!("./migrations").run(&pool).await {
        error!("Failed to run SQLx migrations, {}", e);
        return Err(anyhow!("Failed to run database migrations"));
    };

    return Ok(pool);
}

async fn init_db(rocket: Rocket<Build>) -> fairing::Result {
    let handle = get_db_handle().await;
    match handle {
        Ok(pool) => Ok(rocket.manage(pool)),
        Err(_) => Err(rocket),
    }
}

pub fn stage_database() -> AdHoc {
    AdHoc::on_ignite("SQLx Stage", |rocket| async {
        rocket.attach(AdHoc::try_on_ignite("SQLx Database", init_db))
    })
}
