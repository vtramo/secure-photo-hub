use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use url::Url;
use uuid::Uuid;
use actix_multipart::form::MultipartForm;
use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::json::Json as MpJson;
use crate::models::api::VisibilityApi;
use crate::models::service::photo::{Photo, UpdatePhoto, UploadPhoto};
use crate::models::service::Visibility;
use crate::models::service::image::{UploadImage, UploadImageError};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PhotoApi {
    pub id: Uuid,
    #[serde(rename = "albumId")]
    pub album_id: Option<Uuid>,
    pub title: String,
    pub description: String,
    pub category: String,
    pub tags: String,
    pub visibility: VisibilityApi,
    #[serde(rename = "createdAt", with = "crate::models::api::serde_date")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "imageId")]
    pub image_id: Uuid,
    #[serde(rename = "imageUrl", with = "crate::models::api::serde_url")]
    pub image_url: Url,
}

impl From<Photo> for PhotoApi {
    fn from(photo: Photo) -> Self {
        Self {
            id: photo.id().clone(),
            album_id: photo.album_id().clone(),
            title: photo.title().to_string(),
            description: photo.description().to_string(),
            category: photo.category().to_string(),
            tags: photo.tags().to_vec().join(", "),
            visibility: VisibilityApi::from(photo.visibility().clone()),
            created_at: photo.created_at(),
            image_id: photo.image().id().clone(),
            image_url: photo.image().url().clone(),
        }
    }
}

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
    pub album_id: Option<String>,
    pub description: String,
    pub category: String,
    pub tags: Vec<String>,
    pub visibility: VisibilityApi,
}

impl TryFrom<UploadPhotoApi> for UploadPhoto {
    type Error = crate::models::service::image::UploadImageError;

    fn try_from(upload_photo_api: UploadPhotoApi) -> Result<Self, Self::Error> {
        let UploadPhotoApi { file: image, metadata } = upload_photo_api;
        let upload_image = UploadImage::try_from(image)?;
        let album_id = metadata.0.album_id
            .map(|uuid_str| Uuid::parse_str(&uuid_str))
            .transpose()
            .map_err(|_| UploadImageError::InvalidAlbum)?;

        Ok(Self::new(
            metadata.0.title,
            album_id,
            metadata.0.description,
            metadata.0.category,
            metadata.0.tags,
            Visibility::from(metadata.0.visibility),
            upload_image,
        ))
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PatchPhotoApi {
    pub title: Option<String>,
    #[serde(rename = "albumId")]
    pub album_id: Option<Uuid>,
    pub visibility: Option<Visibility>,
}

impl UpdatePhoto {
    pub fn from(photo_id: Uuid, patch_photo_api: PatchPhotoApi) -> Self {
        Self::new(
            &photo_id,
            patch_photo_api.title.as_ref(),
            patch_photo_api.album_id.as_ref(),
            patch_photo_api.visibility.as_ref(),
        )
    }
}