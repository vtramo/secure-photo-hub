use async_trait::async_trait;
use crate::models::service::album::{Album, UpdateAlbum};
use crate::models::service::photo::{Photo, UpdatePhoto};
use crate::security::auth::user::AuthenticatedUser;

mod photo;
mod kc_authz_service;
mod album;
mod claims;

pub use kc_authz_service::{AuthorizationScope, KcAuthzService};
pub use photo::{PhotoPolicyEnforcerKc};
pub use album::{AlbumPolicyEnforcerKc};

#[async_trait()]
pub trait PhotoPolicyEnforcer: Send + Sync + 'static + Clone {
    async fn can_view_photo(&self, authenticated_user: &AuthenticatedUser, photo: &Photo) -> anyhow::Result<bool>;
    async fn can_create_photo(&self, authenticated_user: &AuthenticatedUser) -> anyhow::Result<bool>;
    async fn can_edit_photo(&self, authenticated_user: &AuthenticatedUser, update_photo: &UpdatePhoto) -> anyhow::Result<bool>;
}

#[async_trait()]
pub trait AlbumPolicyEnforcer: Send + Sync + 'static + Clone {
    async fn can_view_album(&self, authenticated_user: &AuthenticatedUser, album: &Album) -> anyhow::Result<bool>;
    async fn can_create_album(&self, authenticated_user: &AuthenticatedUser) -> anyhow::Result<bool>;
    async fn can_edit_album(&self, authenticated_user: &AuthenticatedUser, update_album: &UpdateAlbum) -> anyhow::Result<bool>;
}