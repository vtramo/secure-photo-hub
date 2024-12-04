use actix_multipart::form::{json::Json as MpJson, MultipartForm};
use actix_multipart::form::tempfile::TempFile;
use serde::{Deserialize, Serialize};

#[derive(Debug, MultipartForm)]
pub struct UploadPhotoApi {
    #[multipart(limit = "100MB")]
    pub file: TempFile,
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