use actix_multipart::form::MultipartForm;
use actix_multipart::form::tempfile::TempFile;
use serde::{Deserialize, Serialize};
use crate::models::api::VisibilityApi;
use actix_multipart::form::json::Json as MpJson;
use chrono::{DateTime, Utc};
use url::Url;
use uuid::Uuid;
use crate::models::service::album::{Album, CreateAlbumWithCover, UpdateAlbum};
use crate::models::service::Visibility;
use crate::models::service::image::UploadImage;
use crate::models::service::photo::UpdatePhoto;

#[derive(Debug, MultipartForm)]
pub struct CreateAlbumApi {
    #[multipart(limit = "100MB")]
    pub file: TempFile,

    #[multipart]
    pub metadata: MpJson<CreateAlbumMetadataApi>
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateAlbumMetadataApi {
    pub title: String,
    pub description: String,
    pub visibility: VisibilityApi,
}

impl TryFrom<CreateAlbumApi> for CreateAlbumWithCover {
    type Error = crate::models::service::image::UploadImageError;

    fn try_from(create_album_api: CreateAlbumApi) -> Result<Self, Self::Error> {
        let CreateAlbumApi { file: image, metadata } = create_album_api;
        let album_metadata_api = metadata.0;
        let visibility = Visibility::from(album_metadata_api.visibility);
        let upload_image = UploadImage::try_from(image, visibility)?;

        Ok(Self::new(
            album_metadata_api.title,
            album_metadata_api.description,
            visibility,
            upload_image,
        ))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AlbumApi {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub visibility: VisibilityApi,
    #[serde(rename = "createdAt", with = "crate::models::api::serde_date")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "coverImageId")]
    pub cover_image_id: Uuid,
    #[serde(rename = "coverImageUrl", with = "crate::models::api::serde_url")]
    cover_image_url: Url,
}

impl From<Album> for AlbumApi {
    fn from(album: Album) -> Self {
        Self {
            id: album.id(),
            title: album.title().to_string(),
            description: album.description().to_string(),
            visibility: VisibilityApi::from(album.visibility().clone()),
            created_at: album.created_at(),
            cover_image_id: album.cover_image_id(),
            cover_image_url: album.cover_image_url().clone(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PatchAlbumApi {
    pub title: Option<String>,
    #[serde(rename = "albumId")]
    pub album_id: Option<Uuid>,
    pub visibility: Option<Visibility>,
}

impl UpdateAlbum {
    pub fn from(album_id: Uuid, patch_album_api: PatchAlbumApi) -> Self {
        Self::new(
            &album_id,
            patch_album_api.title.as_ref(),
            patch_album_api.visibility.as_ref(),
        )
    }
}