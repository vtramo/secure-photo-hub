pub mod photo;
pub mod album;
pub mod pagination;

use std::fmt;

use image::ImageFormat;
use sqlx::types::Uuid;
use crate::models::entity::{ImageReferenceEntity, ImageFormatEntity};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Visibility {
    Public,
    Private,
}

impl From<String> for Visibility {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Public" => Self::Public,
            "Private" => Self::Private,
            _ => panic!("impl From<String> for Visibility"),
        }
    }
}

impl fmt::Display for Visibility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let visibility_str = match *self {
            Visibility::Public => "Public",
            Visibility::Private => "Private",
        };
        write!(f, "{}", visibility_str)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Image {
    id: Uuid,
    url: url::Url,
    size: u64,
    format: ImageFormat,
}

impl Image {
    pub fn id(&self) -> &Uuid {
        &self.id
    }
    pub fn url(&self) -> &url::Url {
        &self.url
    }
    pub fn size(&self) -> u64 {
        self.size
    }
    pub fn format(&self) -> ImageFormat {
        self.format
    }
}

impl From<ImageReferenceEntity> for Image {
    fn from(image_entity: ImageReferenceEntity) -> Self {
        Self {
            id: image_entity.id,
            url: url::Url::parse(&image_entity.url).unwrap(),
            size: image_entity.size as u64,
            format: ImageFormat::from(image_entity.format),
        }
    }
}

impl From<ImageFormatEntity> for ImageFormat {
    fn from(image_format_entity: ImageFormatEntity) -> Self {
        match image_format_entity {
            ImageFormatEntity::Png => ImageFormat::Png,
            ImageFormatEntity::Jpeg => ImageFormat::Jpeg,
            ImageFormatEntity::Gif => ImageFormat::Gif,
            ImageFormatEntity::WebP => ImageFormat::WebP,
            ImageFormatEntity::Pnm => ImageFormat::Pnm,
            ImageFormatEntity::Tiff => ImageFormat::Tiff,
            ImageFormatEntity::Tga => ImageFormat::Tga,
            ImageFormatEntity::Dds => ImageFormat::Dds,
            ImageFormatEntity::Bmp => ImageFormat::Bmp,
            ImageFormatEntity::Ico => ImageFormat::Ico,
            ImageFormatEntity::Hdr => ImageFormat::Hdr,
            ImageFormatEntity::OpenExr => ImageFormat::OpenExr,
            ImageFormatEntity::Farbfeld => ImageFormat::Farbfeld,
            ImageFormatEntity::Avif => ImageFormat::Avif,
            ImageFormatEntity::Qoi => ImageFormat::Qoi,
            ImageFormatEntity::Pcx => ImageFormat::Pcx,
        }
    }
}