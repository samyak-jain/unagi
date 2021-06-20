use crate::services::files::ShowDetails;
use async_trait::async_trait;

#[async_trait]
pub trait AnimeApi {
    async fn search_anime(name: &str) -> anyhow::Result<i32>;
    async fn fetch_anime(anime_id: i32, show: &mut ShowDetails) -> anyhow::Result<()>;
}
