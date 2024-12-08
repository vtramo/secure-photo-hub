use std::fmt;

use chrono::Utc;
use image::ImageFormat;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::service::Visibility;

pub mod photo;
pub mod album;

#[derive(sqlx::FromRow, Debug, Eq, PartialEq, Clone)]
pub struct ImageReferenceEntity {
    pub id: Uuid,
    pub url: String,
    pub size: i64,
    pub format: ImageFormatEntity,
    pub created_at: chrono::DateTime<Utc>
}

#[derive(sqlx::Type, Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
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

#[derive(Clone, Debug, Copy, Serialize, Deserialize, Eq, PartialEq, sqlx::Type)]
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
