use url::Url;
use uuid::Uuid;
use crate::models::entity::{ImageEntity, ImageFormatEntity, VisibilityEntity};

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

#[derive(sqlx::FromRow, Debug)]
pub struct AlbumCoverImageEntity {
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

impl From<AlbumCoverImageEntity> for AlbumEntity {
    fn from(album_cover_image_entity: AlbumCoverImageEntity) -> Self {
        Self {
            id: album_cover_image_entity.album_id,
            owner_user_id: album_cover_image_entity.owner_user_id,
            title: album_cover_image_entity.title,
            description: album_cover_image_entity.description,
            visibility: album_cover_image_entity.visibility,
            cover_image: ImageEntity {
                id: album_cover_image_entity.image_id,
                url: Url::parse(&album_cover_image_entity.url).expect("impl From<AlbumCoverImageEntity> for AlbumEntity"),
                size: album_cover_image_entity.size as u64,
                format: album_cover_image_entity.format,
                created_at: album_cover_image_entity.image_created_at,
            },
            created_at: album_cover_image_entity.album_created_at,
        }
    }
}