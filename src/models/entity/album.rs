use uuid::Uuid;
use crate::models::entity::{ImageReferenceEntity, ImageFormatEntity, VisibilityEntity};

#[derive(sqlx::FromRow, Debug, Eq, PartialEq, Clone)]
pub struct AlbumEntity {
    pub id: Uuid,
    pub owner_user_id: Uuid,
    pub title: String,
    pub description: String,
    pub visibility: VisibilityEntity,
    pub cover_image: ImageReferenceEntity,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(sqlx::FromRow, Debug, Eq, PartialEq, Clone)]
pub struct AlbumCoverImageReferenceEntity {
    pub album_id: Uuid,
    pub title: String,
    pub description: String,
    pub visibility: VisibilityEntity,
    pub owner_user_id: Uuid,
    pub image_reference_id: Uuid,
    pub album_created_at: chrono::DateTime<chrono::Utc>,

    pub image_id: Uuid,
    pub url: String,
    pub size: i64,
    pub format: ImageFormatEntity,
    pub image_created_at: chrono::DateTime<chrono::Utc>,
}

impl From<AlbumCoverImageReferenceEntity> for AlbumEntity {
    fn from(album_cover_image_entity: AlbumCoverImageReferenceEntity) -> Self {
        Self {
            id: album_cover_image_entity.album_id,
            owner_user_id: album_cover_image_entity.owner_user_id,
            title: album_cover_image_entity.title,
            description: album_cover_image_entity.description,
            visibility: album_cover_image_entity.visibility,
            cover_image: ImageReferenceEntity {
                id: album_cover_image_entity.image_id,
                url: album_cover_image_entity.url,
                size: album_cover_image_entity.size,
                format: album_cover_image_entity.format,
                created_at: album_cover_image_entity.image_created_at,
            },
            created_at: album_cover_image_entity.album_created_at,
        }
    }
}

#[derive(sqlx::FromRow, Debug, Eq, PartialEq, Clone)]
pub struct AlbumNoCoverImageReferenceEntity {
    pub id: Uuid,
    pub owner_user_id: Uuid,
    pub title: String,
    pub description: String,
    pub visibility: VisibilityEntity,
    pub cover_image_id: Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
}