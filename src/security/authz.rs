use async_trait::async_trait;
use crate::models::service::album::{Album, UpdateAlbum};
use crate::models::service::photo::{Photo, UpdatePhoto};
use crate::security::auth::user::AuthenticatedUser;

mod photo;
mod kc_authz_service;
mod album;
mod claims;
mod image;

pub use kc_authz_service::{AuthorizationScope, KcAuthzService};
pub use photo::{PhotoPolicyEnforcerKc};
pub use album::{AlbumPolicyEnforcerKc};
pub use image::{ImagePolicyEnforcerKc};
use crate::models::service::image::{ImageReference};

#[async_trait()]
pub trait PhotoPolicyEnforcer: Send + Sync + 'static + Clone {
    async fn can_view_photo(&self, authenticated_user: &AuthenticatedUser, photo: &Photo) -> anyhow::Result<bool>;
    async fn can_create_photo(&self, authenticated_user: &AuthenticatedUser) -> anyhow::Result<bool>;
    async fn can_edit_photo(&self, authenticated_user: &AuthenticatedUser, photo: &Photo, update_photo: &UpdatePhoto) -> anyhow::Result<bool>;
    async fn filter_photos_by_view_permission<'a>(
        &self,
        authenticated_user: &AuthenticatedUser,
        photos: Vec<Photo>
    ) -> anyhow::Result<Vec<Photo>>;
}

#[async_trait()]
pub trait AlbumPolicyEnforcer: Send + Sync + 'static + Clone {
    async fn can_view_album(&self, authenticated_user: &AuthenticatedUser, album: &Album) -> anyhow::Result<bool>;
    async fn can_create_album(&self, authenticated_user: &AuthenticatedUser) -> anyhow::Result<bool>;
    async fn can_edit_album(&self, authenticated_user: &AuthenticatedUser, album: &Album, update_album: &UpdateAlbum) -> anyhow::Result<bool>;
    async fn filter_albums_by_view_permission(
        &self,
        authenticated_user: &AuthenticatedUser,
        albums: Vec<Album>
    ) -> anyhow::Result<Vec<Album>>;
}

#[async_trait()]
pub trait ImagePolicyEnforcer: Send + Sync + 'static + Clone {
    async fn can_download(&self, authenticated_user: &AuthenticatedUser, image_reference: &ImageReference) -> anyhow::Result<bool>;
    async fn can_transform(&self, authenticated_user: &AuthenticatedUser, image_reference: &ImageReference) -> anyhow::Result<bool>;
    async fn can_download_then_transform(&self, authenticated_user: &AuthenticatedUser, image_reference: &ImageReference) -> anyhow::Result<bool>;
    async fn can_create(&self, authenticated_user: &AuthenticatedUser) -> anyhow::Result<bool>;
    async fn can_view(&self, authenticated_user: &AuthenticatedUser, image_reference: &ImageReference) -> anyhow::Result<bool>;
}