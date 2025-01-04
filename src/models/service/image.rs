use std::io::Read;

use actix_multipart::form::tempfile::TempFile;
use image::ImageFormat;
use uuid::Uuid;

use crate::models::entity::{ImageFormatEntity, ImageReferenceEntity};
use crate::models::service::Visibility;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageReference {
    id: Uuid,
    owner_user_id: Uuid,
    url: url::Url,
    size: u64,
    visibility: Visibility,
    format: ImageFormat,
}

impl ImageReference {
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
    pub fn visibility(&self) -> Visibility {
        self.visibility.clone()
    }
    pub fn new(id: &Uuid, owner_user_id: &Uuid, url: &url::Url, size: u64, format: &ImageFormat, visibility: &Visibility) -> Self {
        Self { id: id.clone(), owner_user_id: owner_user_id.clone(), url: url.clone(), size, visibility: visibility.clone(), format: format.clone() }
    }
    pub fn owner_user_id(&self) -> &Uuid {
        &self.owner_user_id
    }
}

impl From<ImageReferenceEntity> for ImageReference {
    fn from(image_reference_entity: ImageReferenceEntity) -> Self {
        Self {
            id: image_reference_entity.id,
            owner_user_id: image_reference_entity.owner_user_id,
            url: url::Url::parse(&image_reference_entity.url).unwrap(),
            size: image_reference_entity.size as u64,
            visibility: Visibility::from(image_reference_entity.visibility),
            format: ImageFormat::from(image_reference_entity.format),
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
    filename: String,
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
    pub fn new(filename: &str, bytes: Vec<u8>, format: ImageFormat, size: usize) -> Self {
        Self { filename: filename.to_string(), bytes, format, size }
    }
    pub fn filename(&self) -> &str {
        &self.filename
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

                    let file_name = temp_file.file_name.unwrap_or_default();
                    let size = temp_file.size;
                    let mut bytes = Vec::with_capacity(size);
                    temp_file.file.read_to_end(&mut bytes).map_err(|_| UploadImageError::CorruptedImage)?;
                    Ok(Self {
                        filename: file_name,
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

#[derive(Debug, Clone)]
pub struct Image {
    id: Uuid,
    filename: String,
    format: ImageFormat,
    bytes: Vec<u8>,
    size: u32,
}

impl Image {
    pub fn new(id: &Uuid, filename: &str, format: &ImageFormat, bytes: Vec<u8>, size: u32) -> Self {
        Self { id: id.clone(), filename: filename.to_string(), format: format.clone(), bytes, size }
    }
    pub fn id(&self) -> Uuid {
        self.id
    }
    pub fn format(&self) -> ImageFormat {
        self.format
    }
    pub fn bytes(&self) -> &Vec<u8> {
        &self.bytes
    }
    pub fn size(&self) -> u32 {
        self.size
    }
    pub fn take_bytes(self) -> Vec<u8> {
        self.bytes
    }
    pub fn filename(&self) -> &str {
        &self.filename
    }
}

pub enum ImageTransformation {
    HueRotate(i32),
    Thumbnail(u32, u32),
    None,
}

#[derive(Debug, Clone)]
pub struct ImageTransformOptions {
    huerotate: Option<i32>,
    thumbnail: Option<(u32, u32)>
}

impl ImageTransformOptions {
    const AVAILABLE_TRANSFORMATIONS: usize = 2;
    
    pub fn new(huerotate: Option<i32>, thumbnail: Option<(u32, u32)>) -> Self {
        Self { huerotate, thumbnail }
    }
    
    pub fn transformations(&self) -> Vec<ImageTransformation> {
        let mut transformations = Vec::with_capacity(Self::AVAILABLE_TRANSFORMATIONS + 1);

        if let Some(huerotate) = self.huerotate {
            transformations.push(ImageTransformation::HueRotate(huerotate));
        }

        if let Some((nwidth, nheight)) = self.thumbnail {
            transformations.push(ImageTransformation::Thumbnail(nwidth, nheight));
        }

        transformations
    }
}