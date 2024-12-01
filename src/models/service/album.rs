use chrono::Utc;
use image::ImageFormat;
use uuid::Uuid;

use crate::models::service::Visibility;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Album {
    id: Uuid,
    title: String,
    description: String,
    visibility: Visibility,
    owner_user_id: Uuid,
    cover_image_id: Uuid,
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
        created_at: chrono::DateTime<Utc>,
    ) -> Self {
        Album {
            id,
            title,
            description,
            visibility,
            owner_user_id,
            cover_image_id,
            created_at,
        }
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
        cover_image_size: u64, 
        cover_image_format: ImageFormat
    ) -> Self {
        Self { title, description, visibility, owner_user_id, cover_image_id, cover_image_url, cover_image_size, cover_image_format }
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
    pub fn cover_image_url(&self) -> &url::Url {
        &self.cover_image_url
    }
    pub fn cover_image_size(&self) -> u64 {
        self.cover_image_size
    }
    pub fn cover_image_format(&self) -> ImageFormat {
        self.cover_image_format
    }
}