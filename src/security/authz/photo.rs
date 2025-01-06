use std::sync::Arc;

use async_trait::async_trait;

use crate::models::service::photo::{Photo, UpdatePhoto};
use crate::routes;
use crate::security::auth::user::AuthenticatedUser;
use crate::security::authz::claims::CommonClaims;
use crate::security::authz::kc_authz_service::{AuthorizationScope, KcAuthzService};
use crate::security::authz::PhotoPolicyEnforcer;

#[derive(Clone)]
pub struct PhotoPolicyEnforcerKc {
    kc_authz_service: Arc<KcAuthzService>
}

impl PhotoPolicyEnforcerKc {
    pub fn new(kc_authz_service: Arc<KcAuthzService>) -> Self {
        Self { kc_authz_service }
    }
}

#[async_trait()]
impl PhotoPolicyEnforcer for PhotoPolicyEnforcerKc {
    async fn can_view_photo(&self, authenticated_user: &AuthenticatedUser, photo: &Photo) -> anyhow::Result<bool> {
        let resource_id = self.kc_authz_service.get_resource_id(routes::photo::PHOTO_BY_ID_ROUTE).await?;

        let photo_claims = CommonClaims::new(authenticated_user.id(), *photo.visibility());
        let permission_request = self.kc_authz_service.permission_request(
            authenticated_user, 
            photo_claims, 
            &resource_id, 
            &[AuthorizationScope::View],
        );

        permission_request.decision_response_mode_send().await
    }

    async fn can_create_photo(&self, authenticated_user: &AuthenticatedUser) -> anyhow::Result<bool> {
        let resource_id = self.kc_authz_service.get_resource_id(routes::photo::PHOTO_BY_ID_ROUTE).await?;

        let permission_request = self.kc_authz_service.permission_request(
            authenticated_user,
            (),
            &resource_id,
            &[AuthorizationScope::Create],
        );
        
        permission_request.decision_response_mode_send().await
    }

    async fn can_edit_photo(&self, authenticated_user: &AuthenticatedUser, update_photo: &UpdatePhoto) -> anyhow::Result<bool> {
        let resource_id = self.kc_authz_service.get_resource_id(routes::photo::PHOTO_BY_ID_ROUTE).await?;

        let photo_claims = CommonClaims::resource_owner(authenticated_user.id());
        let permission_request = self.kc_authz_service.permission_request(
            authenticated_user,
            photo_claims,
            &resource_id,
            &Self::authorization_scopes(update_photo),
        );

        permission_request.decision_response_mode_send().await
    }
}

impl PhotoPolicyEnforcerKc {

    fn authorization_scopes(update_photo: &UpdatePhoto) -> Vec<AuthorizationScope> {
        let mut authorization_scopes = vec![];

        if let Some(_) = update_photo.visibility() {
            authorization_scopes.push(AuthorizationScope::ChangeVisibility);
        }

        if let Some(_) = update_photo.album_id() {
            authorization_scopes.push(AuthorizationScope::ChangeAlbum);
        }

        if let Some(_) = update_photo.title() {
            authorization_scopes.push(AuthorizationScope::EditTitle);
        }

        return authorization_scopes;
    }
}