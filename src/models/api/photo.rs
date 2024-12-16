use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use url::Url;
use uuid::Uuid;
use actix_multipart::form::MultipartForm;
use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::json::Json as MpJson;
use crate::models;
use crate::models::api::VisibilityApi;
use crate::models::service::photo::{Photo, UploadImage, UploadImageError, UploadPhoto};
use crate::models::service::Visibility;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PhotoApi {
    #[serde(rename = "id")]
    pub id: Uuid, 
    #[serde(rename = "albumId")]
    pub album_id: Option<Uuid>,
    pub title: String,
    pub description: String,
    pub category: String,
    pub tags: String,
    pub visibility: VisibilityApi,
    #[serde(rename = "createdAt", with = "serde_date")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "imageId")]
    pub image_id: Uuid,
    #[serde(rename = "imageUrl", with = "serde_url")]
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

mod serde_date {
    use chrono::{DateTime, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(dt: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_str(&dt.to_rfc3339())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        DateTime::parse_from_rfc3339(&s)
            .map(|dt| dt.with_timezone(&Utc))
            .map_err(serde::de::Error::custom)
    }
}

mod serde_url {
    use serde::{self, Deserialize, Deserializer, Serializer};
    use url::Url;

    pub fn serialize<S>(url: &Url, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_str(url.as_str())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Url, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        Url::parse(&s).map_err(serde::de::Error::custom)
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
    type Error = models::service::photo::UploadImageError;

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
