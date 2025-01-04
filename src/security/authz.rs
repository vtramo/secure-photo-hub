use async_trait::async_trait;
use crate::models::service::photo::{Photo, UpdatePhoto};
use crate::security::auth::user::AuthenticatedUser;

mod kc_authz_service;
pub mod photo;

#[async_trait()]
pub trait PhotoPolicyEnforcer: Send + Sync + 'static + Clone {
    async fn can_view_photo(&self, authenticated_user: &AuthenticatedUser, photo: &Photo) -> anyhow::Result<bool>;
    async fn can_create_photo(&self, authenticated_user: &AuthenticatedUser) -> anyhow::Result<bool>;
    async fn can_edit_photo(&self, authenticated_user: &AuthenticatedUser, update_photo: &UpdatePhoto) -> anyhow::Result<bool>;
}