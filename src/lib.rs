use uuid::Uuid;
use crate::models::service::pagination::Page;
use crate::models::service::photo::Photo;

pub mod models;
mod repository;
pub mod routes;
pub mod security;
pub mod setup;
pub mod service;

#[async_trait::async_trait]
pub trait PhotoService: Clone + Send + Sync + 'static {
    async fn get_all_photos(&self) -> anyhow::Result<Page<Photo>>;
    async fn get_photo_by_id(&self, id: &Uuid) -> anyhow::Result<Option<Photo>>;
    async fn create_photo(&self, ) -> anyhow::Result<Photo>;
}