use uuid::Uuid;
use crate::models::entity::{ImageEntity, VisibilityEntity};

#[derive(sqlx::FromRow, Debug)]
pub struct AlbumEntity {
    pub id: Uuid,
    pub owner_user_id: Uuid,
    pub title: String,
    pub description: String,
    pub visibility: VisibilityEntity,
    pub cover_image: ImageEntity,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
