use chrono::Utc;
use image::ImageFormat;
use url::Url;
use uuid::Uuid;

use crate::models::entity::album::AlbumEntity;
use crate::models::service::Visibility;
use crate::models::service::image::UploadImage;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Album {
    id: Uuid,
    title: String,
    description: String,
    visibility: Visibility,
    owner_user_id: Uuid,
    cover_image_id: Uuid,
    cover_image_url: Url,
    created_at: chrono::DateTime<Utc>,
}

impl Album {
    pub fn new(
        id: Uuid,
        title: String,
        description: String,
        visibility: Visibility,
        owner_user_id: Uuid,
        cover_image_id: Uuid,
        cover_image_url: Url,
        created_at: chrono::DateTime<Utc>,
    ) -> Self {
        Album {
            id,
            title,
            description,
            visibility,
            owner_user_id,
            cover_image_id,
            cover_image_url,
            created_at,
        }
    }
    pub fn id(&self) -> Uuid {
        self.id
    }
    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn visibility(&self) -> &Visibility {
        &self.visibility
    }
    pub fn owner_user_id(&self) -> Uuid {
        self.owner_user_id
    }
    pub fn cover_image_id(&self) -> Uuid {
        self.cover_image_id
    }
    pub fn created_at(&self) -> chrono::DateTime<Utc> {
        self.created_at
    }
    pub fn cover_image_url(&self) -> &Url {
        &self.cover_image_url
    }
}

impl From<AlbumEntity> for Album {
    fn from(album_entity: AlbumEntity) -> Self {
        Self {
            id: album_entity.id,
            title: album_entity.title,
            description: album_entity.description,
            visibility: Visibility::from(album_entity.visibility),
            owner_user_id: album_entity.owner_user_id,
            cover_image_id: album_entity.cover_image.id,
            cover_image_url: Url::parse(album_entity.cover_image.url.as_str()).unwrap(), // TODO:
            created_at: album_entity.created_at,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CreateAlbumWithCover {
    title: String,
    description: String,
    visibility: Visibility,
    upload_image: UploadImage,
}

impl CreateAlbumWithCover {
    pub fn new(title: String, description: String, visibility: Visibility, upload_image: UploadImage) -> Self {
        Self { title, description, visibility, upload_image }
    }
    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn visibility(&self) -> &Visibility {
        &self.visibility
    }
    pub fn upload_image(&self) -> &UploadImage {
        &self.upload_image
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateAlbum {
    title: String,
    description: String,
    visibility: Visibility,
    owner_user_id: Uuid,
    cover_image_id: Uuid,
    cover_image_url: url::Url,
    cover_image_reference_url: url::Url,
    cover_image_size: u64,
    cover_image_format: ImageFormat,
}

impl CreateAlbum {
    pub fn new(
        title: String, 
        description: String, 
        visibility: Visibility, 
        owner_user_id: Uuid, 
        cover_image_id: Uuid, 
        cover_image_url: url::Url,
        cover_image_reference_url: url::Url,
        cover_image_size: u64, 
        cover_image_format: ImageFormat
    ) -> Self {
        Self { title, description, visibility, owner_user_id, cover_image_id, cover_image_url, cover_image_reference_url, cover_image_size, cover_image_format }
    }
    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn visibility(&self) -> &Visibility {
        &self.visibility
    }
    pub fn owner_user_id(&self) -> &Uuid {
        &self.owner_user_id
    }
    pub fn cover_image_id(&self) -> &Uuid {
        &self.cover_image_id
    }
    pub fn cover_image_url(&self) -> &url::Url {
        &self.cover_image_url
    }
    pub fn cover_image_reference_url(&self) -> &url::Url {
        &self.cover_image_reference_url
    }
    pub fn cover_image_size(&self) -> u64 {
        self.cover_image_size
    }
    pub fn cover_image_format(&self) -> &ImageFormat {
        &self.cover_image_format
    }
}

#[derive(Debug, Clone)]
pub struct UpdateAlbum {
    id: Uuid,
    title: Option<String>,
    visibility: Option<Visibility>,
}

impl UpdateAlbum {
    pub fn new(id: &Uuid, title: Option<&String>, visibility: Option<&Visibility>) -> Self {
        Self { id: id.clone(), title: title.cloned(), visibility: visibility.cloned() }
    }
    pub fn id(&self) -> &Uuid {
        &self.id
    }
    pub fn title(&self) -> &Option<String> {
        &self.title
    }
    pub fn visibility(&self) -> Option<Visibility> {
        self.visibility
    }
}