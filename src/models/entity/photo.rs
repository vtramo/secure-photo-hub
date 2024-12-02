use uuid::Uuid;

use crate::models::entity::{ImageEntity, ImageFormatEntity, VisibilityEntity};

#[derive(sqlx::FromRow, Debug, Eq, PartialEq, Clone)]
pub struct PhotoEntity {
    pub id: Uuid,
    pub album_id: Option<Uuid>,
    pub owner_user_id: Uuid,
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
    pub category: String,
    pub visibility: VisibilityEntity,
    pub image: ImageEntity,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub is_deleted: bool,
}

impl From<PhotoImageEntity> for PhotoEntity {
    fn from(photo_image_entity: PhotoImageEntity) -> Self {
        PhotoEntity {
            id: photo_image_entity.photo_id,
            album_id: photo_image_entity.album_id.filter(|album_id| !album_id.is_nil()),
            owner_user_id: photo_image_entity.owner_user_id,
            title: photo_image_entity.title,
            description: photo_image_entity.description,
            tags: photo_image_entity.tags,
            category: photo_image_entity.category,
            visibility: photo_image_entity.visibility,
            image: ImageEntity {
                id: photo_image_entity.image_id,
                url: photo_image_entity.url,
                size: photo_image_entity.size,
                format: photo_image_entity.format,
                created_at: photo_image_entity.image_created_at,
            },
            created_at: photo_image_entity.photo_created_at,
            is_deleted: photo_image_entity.is_deleted,
        }
    }
}

#[derive(sqlx::FromRow, Debug, Eq, PartialEq, Clone)]
pub struct PhotoImageEntity {
    pub photo_id: Uuid,
    pub title: String,
    pub description: String,
    pub visibility: VisibilityEntity,
    pub owner_user_id: Uuid,
    pub tags: Vec<String>,
    pub category: String,
    pub album_id: Option<Uuid>,
    pub image_reference_id: Uuid,
    pub is_deleted: bool,
    pub photo_created_at: chrono::DateTime<chrono::Utc>,

    pub image_id: Uuid,
    pub url: String,
    pub size: i64,
    pub format: ImageFormatEntity,
    pub image_created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(sqlx::FromRow, Debug, Eq, PartialEq, Clone)]
pub struct PhotoNoImageEntity {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub visibility: VisibilityEntity,
    pub owner_user_id: Uuid,
    pub tags: Vec<String>,
    pub category: String,
    pub album_id: Option<Uuid>,
    pub image_id: Uuid,
    pub is_deleted: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}