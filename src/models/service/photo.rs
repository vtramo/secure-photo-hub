use chrono::Utc;
use image::ImageFormat;
use uuid::Uuid;

use crate::models::entity::photo::PhotoEntity;
use crate::models::service::Visibility;
use crate::models::service::image::{ImageReference, UploadImage};

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
    image: ImageReference,
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
        image: ImageReference,
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
    pub fn id(&self) -> &Uuid {
        &self.id
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
    pub fn owner_user_id(&self) -> &Uuid {
        &self.owner_user_id
    }
    pub fn album_id(&self) -> &Option<Uuid> {
        &self.album_id
    }
    pub fn visibility(&self) -> &Visibility {
        &self.visibility
    }
    pub fn image(&self) -> &ImageReference {
        &self.image
    }
    pub fn created_at(&self) -> chrono::DateTime<Utc> {
        self.created_at
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdatePhoto {
    id: Uuid,
    title: Option<String>,
    album_id: Option<Uuid>
}

impl UpdatePhoto {
    pub fn new(id: &Uuid, title: Option<&String>, album_id: Option<&Uuid>) -> Self {
        Self { id: id.clone(), title: title.cloned(), album_id: album_id.cloned() }
    }
    pub fn id(&self) -> &Uuid {
        &self.id
    }
    pub fn title(&self) -> &Option<String> {
        &self.title
    }
    pub fn album_id(&self) -> &Option<Uuid> {
        &self.album_id
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
    image_url: url::Url,
    image_size: u64,
    image_format: ImageFormat,
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
    pub fn image_url(&self) -> &url::Url {
        &self.image_url
    }
    pub fn image_size(&self) -> u64 {
        self.image_size
    }
    pub fn image_format(&self) -> &ImageFormat {
        &self.image_format
    }
    pub fn image_id(&self) -> &Uuid {
        &self.image_id
    }

    pub fn new(
        title: &str,
        description: &str,
        category: &str,
        tags: &Vec<String>,
        owner_user_id: &Uuid,
        image_id: &Uuid,
        album_id: &Option<Uuid>,
        visibility: &Visibility,
        url: &url::Url,
        size: u64,
        format: &ImageFormat,
    ) -> Self {
        Self {
            title: title.to_string(),
            description: description.to_string(),
            category: category.to_string(),
            tags: tags.iter().cloned().collect(),
            owner_user_id: owner_user_id.clone(),
            image_id: image_id.clone(),
            album_id: album_id.clone(),
            visibility: visibility.clone(),
            image_url: url.clone(),
            image_size: size,
            image_format: format.clone(),
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
            image: ImageReference::from(photo_entity.image),
            created_at: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct UploadPhoto {
    title: String,
    album_id: Option<Uuid>,
    description: String,
    category: String,
    tags: Vec<String>,
    visibility: Visibility,
    upload_image: UploadImage,
}

impl UploadPhoto {
    pub fn new(
        title: String, 
        album_id: Option<Uuid>, 
        description: String, 
        category: String, 
        tags: Vec<String>, 
        visibility: Visibility, 
        upload_image: UploadImage
    ) -> Self {
        Self { title, album_id, description, category, tags, visibility, upload_image }
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
    pub fn visibility(&self) -> &Visibility {
        &self.visibility
    }
    pub fn upload_image(&self) -> &UploadImage {
        &self.upload_image
    }
    pub fn album_id(&self) -> &Option<Uuid> {
        &self.album_id
    }
}