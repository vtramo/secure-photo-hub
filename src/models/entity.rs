use std::fmt;

use chrono::Utc;
use image::ImageFormat;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::service::{Visibility};

#[derive(sqlx::FromRow, Debug)]
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
    pub id: Uuid,
    pub url: url::Url,
    pub size: u64,
    pub format: ImageFormatEntity,
    pub created_at: chrono::DateTime<Utc>
}

#[derive(sqlx::Type, Debug, Serialize, Deserialize)]
#[sqlx(type_name = "image_format")]
pub enum ImageFormatEntity {
    Png,
    Jpeg,
    Gif,
    WebP,
    Pnm,
    Tiff,
    Tga,
    Dds,
    Bmp,
    Ico,
    Hdr,
    OpenExr,
    Farbfeld,
    Avif,
    Qoi,
    Pcx,
}

impl From<ImageFormat> for ImageFormatEntity {
    fn from(format: ImageFormat) -> Self {
        match format {
            ImageFormat::Png => ImageFormatEntity::Png,
            ImageFormat::Jpeg => ImageFormatEntity::Jpeg,
            ImageFormat::Gif => ImageFormatEntity::Gif,
            ImageFormat::WebP => ImageFormatEntity::WebP,
            ImageFormat::Pnm => ImageFormatEntity::Pnm,
            ImageFormat::Tiff => ImageFormatEntity::Tiff,
            ImageFormat::Tga => ImageFormatEntity::Tga,
            ImageFormat::Dds => ImageFormatEntity::Dds,
            ImageFormat::Bmp => ImageFormatEntity::Bmp,
            ImageFormat::Ico => ImageFormatEntity::Ico,
            ImageFormat::Hdr => ImageFormatEntity::Hdr,
            ImageFormat::OpenExr => ImageFormatEntity::OpenExr,
            ImageFormat::Farbfeld => ImageFormatEntity::Farbfeld,
            ImageFormat::Avif => ImageFormatEntity::Avif,
            ImageFormat::Qoi => ImageFormatEntity::Qoi,
            ImageFormat::Pcx => ImageFormatEntity::Pcx,
            _ => panic!("impl From<ImageFormat> for ImageFormatEntity"),
        }
    }
}

#[derive(Clone, Debug, Copy, Serialize, Deserialize, PartialEq, sqlx::Type)]
#[sqlx(type_name = "visibility")]
pub enum VisibilityEntity {
    Public,
    Private,
}

impl From<Visibility> for VisibilityEntity {
    fn from(value: Visibility) -> Self {
        match value {
            Visibility::Public => Self::Public,
            Visibility::Private => Self::Private,
        }
    }
}

impl fmt::Display for VisibilityEntity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let visibility_str = match *self {
            VisibilityEntity::Public => "Public",
            VisibilityEntity::Private => "Private",
        };
        write!(f, "{}", visibility_str)
    }
}

impl From<VisibilityEntity> for Visibility {
    fn from(value: VisibilityEntity) -> Self {
        match value {
            VisibilityEntity::Public => Visibility::Public,
            VisibilityEntity::Private => Visibility::Private,
        }
    }
}

#[derive(sqlx::FromRow, Debug)]
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
    pub photo_created_at: chrono::DateTime<Utc>,

    pub image_id: Uuid,
    pub url: String,
    pub size: i64,
    pub format: ImageFormatEntity,
    pub image_created_at: chrono::DateTime<Utc>,
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
                url: url::Url::parse(&photo_image_entity.url).expect("Should be a valid url!"),
                size: photo_image_entity.size as u64,
                format: photo_image_entity.format,
                created_at: photo_image_entity.image_created_at,
            },
            created_at: photo_image_entity.photo_created_at,
            is_deleted: photo_image_entity.is_deleted,
        }
    }
}