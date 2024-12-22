pub mod photo;
pub mod album;
pub mod pagination;

use std::fmt;

use image::ImageFormat;
use sqlx::types::Uuid;
use actix_multipart::form::tempfile::TempFile;
use std::io::Read;
use crate::models::entity::{ImageFormatEntity, ImageReferenceEntity};

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


#[derive(Debug, Clone)]
pub struct UploadImage {
    bytes: Vec<u8>,
    format: ImageFormat,
    size: usize,
}

impl UploadImage {
    pub fn bytes(&self) -> &Vec<u8> {
        &self.bytes
    }
    pub fn format(&self) -> ImageFormat {
        self.format
    }
    pub fn size(&self) -> usize {
        self.size
    }
    pub fn new(bytes: Vec<u8>, format: ImageFormat, size: usize) -> Self {
        Self { bytes, format, size }
    }
}

impl TryFrom<TempFile> for UploadImage {
    type Error = UploadImageError;

    fn try_from(mut temp_file: TempFile) -> Result<Self, Self::Error> {
        match temp_file.content_type {
            None => Err(UploadImageError::MissingContentType),
            Some(content_type) => match content_type.type_() {
                mime::IMAGE => {
                    let format = match content_type.subtype() {
                        mime::JPEG => ImageFormat::Jpeg,
                        mime::PNG => ImageFormat::Png,
                        mime::GIF => ImageFormat::Gif,
                        _ => return Err(UploadImageError::UnsupportedMimeType),
                    };

                    let size = temp_file.size;
                    let mut bytes = Vec::with_capacity(size);
                    temp_file.file.read_to_end(&mut bytes).map_err(|_| UploadImageError::CorruptedImage)?;
                    Ok(Self {
                        bytes,
                        format,
                        size,
                    })
                },
                _ => Err(UploadImageError::BadContentType)
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum UploadImageError {
    MissingContentType,
    BadContentType,
    UnsupportedMimeType,
    CorruptedImage,
    InvalidAlbum
}
