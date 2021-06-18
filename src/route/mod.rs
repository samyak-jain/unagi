type ApiResult<T, E = rocket::response::Debug<sqlx::Error>> = std::result::Result<T, E>;

// pub mod config;
// pub mod episodes;
// pub mod files;
pub mod library;
// pub mod shows;
