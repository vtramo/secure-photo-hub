use actix_multipart::form::{json::Json as MpJson, MultipartForm};
use actix_multipart::form::tempfile::TempFile;
use serde::{Deserialize, Serialize};
use crate::models;
use crate::models::service::photo::{UploadImage, UploadPhoto};
use crate::models::service::Visibility;

#[derive(Debug, MultipartForm)]
pub struct UploadPhotoApi {
    #[multipart(limit = "100MB")]
    pub file: TempFile,

    #[multipart]
    pub metadata: MpJson<UploadPhotoMetadataApi>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UploadPhotoMetadataApi {
    pub title: String,
    pub album_id: String,
    pub description: String,
    pub category: String,
    pub tags: Vec<String>,
    pub visibility: VisibilityApi,
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub enum VisibilityApi {
    #[serde(alias="public", alias="PUBLIC")]
    Public,

    #[serde(alias="private", alias="PRIVATE")]
    Private
}

impl TryFrom<UploadPhotoApi> for UploadPhoto {
    type Error = models::service::photo::UploadImageError;

    fn try_from(upload_photo_api: UploadPhotoApi) -> Result<Self, Self::Error> {
        let UploadPhotoApi { file: image, metadata } = upload_photo_api;
        let upload_image = UploadImage::try_from(image)?;
        Ok(Self::new(
            metadata.0.title,
            metadata.0.album_id,
            metadata.0.description,
            metadata.0.category,
            metadata.0.tags,
            Visibility::from(metadata.0.visibility),
            upload_image,
        ))
    }
}

impl From<VisibilityApi> for Visibility {
    fn from(visibility_api: VisibilityApi) -> Self {
        match visibility_api {
            VisibilityApi::Public => Visibility::Public,
            VisibilityApi::Private => Visibility::Private,
        }
    }
}