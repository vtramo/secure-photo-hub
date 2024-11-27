use image::ImageFormat;
use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Debug)]
pub struct PhotoEntity {
    pub id: String,
    pub album_id: Option<String>,
    pub owner_user_id: String,
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
    pub category: String,
    pub visibility: VisibilityEntity,
    pub image: ImageEntity,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub is_deleted: bool,
}

#[derive(sqlx::FromRow, Debug)]
pub struct AlbumEntity {
    pub id: String,
    pub owner_user_id: String,
    pub title: String,
    pub description: String,
    pub visibility: VisibilityEntity,
    pub cover_image: ImageEntity,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(sqlx::FromRow, Debug)]
pub struct ImageEntity {
    pub id: String,
    pub url: String,
    pub size: String,
    pub format: ImageFormat,
}

#[derive(Clone, Debug, Copy, Serialize, Deserialize, PartialEq)]
pub enum VisibilityEntity {
    Public, Private
}