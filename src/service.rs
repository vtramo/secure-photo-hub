use uuid::Uuid;
use crate::models::service::album::{Album, CreateAlbum, CreateAlbumWithCover};
use crate::models::service::pagination::Page;
use crate::models::service::photo::{Photo, UpdatePhoto, UploadPhoto};
use crate::security::auth::user::AuthenticatedUser;

pub mod photo;
pub mod album;

#[async_trait::async_trait]
pub trait PhotoService: Clone + Send + Sync + 'static {
    async fn get_all_photos(&self, authenticated_user: &AuthenticatedUser) -> anyhow::Result<Page<Photo>>;
    async fn get_photo_by_id(&self, authenticated_user: &AuthenticatedUser, id: &Uuid) -> anyhow::Result<Option<Photo>>;
    async fn create_photo(&self, authenticated_user: &AuthenticatedUser, upload_photo: &UploadPhoto) -> anyhow::Result<Photo>;
    async fn update_photo(&self, authenticated_user: &AuthenticatedUser, update_photo: &UpdatePhoto) -> anyhow::Result<Photo>;
}

#[async_trait::async_trait]
pub trait AlbumService: Clone + Send + Sync + 'static {
    async fn get_all_albums(&self, authenticated_user: &AuthenticatedUser) -> anyhow::Result<Page<Album>>;
    async fn get_album_by_id(&self, authenticated_user: &AuthenticatedUser, id: &Uuid) -> anyhow::Result<Option<Album>>;
    async fn create_album(&self, authenticated_user: &AuthenticatedUser, create_album: &CreateAlbumWithCover) -> anyhow::Result<Album>; 
}