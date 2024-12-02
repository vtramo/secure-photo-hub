use chrono::Utc;
use uuid::Uuid;
use image::ImageFormat;
use crate::models::entity::photo::PhotoEntity;
use crate::models::service::{Image, Visibility};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Photo {
    id: Uuid,
    title: String,
    description: String,
    category: String,
    tags: Vec<String>,
    owner_user_id: Uuid,
    album_id: Option<Uuid>,
    visibility: Visibility,
    image: Image,
    created_at: chrono::DateTime<Utc>,
}

impl Photo {
    pub fn new(
        id: Uuid,
        title: String,
        description: String,
        category: String,
        tags: Vec<String>,
        owner_user_id: Uuid,
        album_id: Option<Uuid>,
        visibility: Visibility,
        image: Image,
        created_at: chrono::DateTime<Utc>
    ) -> Self {
        Self {
            id,
            title,
            description,
            category,
            tags,
            owner_user_id,
            album_id,
            visibility,
            image,
            created_at
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
    pub fn category(&self) -> &str {
        &self.category
    }
    pub fn tags(&self) -> &Vec<String> {
        &self.tags
    }
    pub fn owner_user_id(&self) -> Uuid {
        self.owner_user_id
    }
    pub fn album_id(&self) -> Option<Uuid> {
        self.album_id
    }
    pub fn visibility(&self) -> &Visibility {
        &self.visibility
    }
    pub fn image(&self) -> &Image {
        &self.image
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreatePhoto {
    title: String,
    description: String,
    category: String,
    tags: Vec<String>,
    owner_user_id: Uuid,
    image_id: Uuid,
    album_id: Option<Uuid>,
    visibility: Visibility,
    url: url::Url,
    size: u64,
    format: ImageFormat,
}

impl CreatePhoto {
    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn category(&self) -> &str {
        &self.category
    }
    pub fn tags(&self) -> &Vec<String> {
        &self.tags
    }
    pub fn owner_user_id(&self) -> &Uuid {
        &self.owner_user_id
    }
    pub fn album_id(&self) -> &Option<Uuid> {
        &self.album_id
    }
    pub fn visibility(&self) -> &Visibility {
        &self.visibility
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
    pub fn image_id(&self) -> Uuid {
        self.image_id
    }

    pub fn new(
        title: String,
        description: String,
        category: String,
        tags: Vec<String>,
        owner_user_id: Uuid,
        image_id: Uuid,
        album_id: Option<Uuid>,
        visibility: Visibility,
        url: url::Url,
        size: u64,
        format: ImageFormat,
    ) -> Self {
        Self {
            title,
            description,
            category,
            tags,
            owner_user_id,
            image_id,
            album_id,
            visibility,
            url,
            size,
            format,
        }
    }
}

impl From<PhotoEntity> for Photo {
    fn from(photo_entity: PhotoEntity) -> Self {
        Self {
            id: photo_entity.id,
            title: photo_entity.title,
            description: photo_entity.description,
            category: photo_entity.category,
            tags: photo_entity.tags,
            owner_user_id: photo_entity.owner_user_id,
            album_id: photo_entity.album_id,
            visibility: Visibility::from(photo_entity.visibility),
            image: Image::from(photo_entity.image),
            created_at: Default::default(),
        }
    }
}