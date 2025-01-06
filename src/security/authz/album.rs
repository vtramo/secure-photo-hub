use std::sync::Arc;
use async_trait::async_trait;
use crate::models::service::album::{Album, UpdateAlbum};
use crate::routes;
use crate::security::auth::user::AuthenticatedUser;
use crate::security::authz::AlbumPolicyEnforcer;
use crate::security::authz::claims::CommonClaims;
use crate::security::authz::kc_authz_service::{AuthorizationScope, KcAuthzService};

#[derive(Clone)]
pub struct AlbumPolicyEnforcerKc {
    kc_authz_service: Arc<KcAuthzService>
}

impl AlbumPolicyEnforcerKc {
    pub fn new(kc_authorization_service: Arc<KcAuthzService>) -> Self {
        Self { kc_authz_service: kc_authorization_service }
    }
}

#[async_trait()]
impl AlbumPolicyEnforcer for AlbumPolicyEnforcerKc {
    async fn can_view_album(&self, authenticated_user: &AuthenticatedUser, album: &Album) -> anyhow::Result<bool> {
        let resource_id = self.kc_authz_service.get_resource_id(routes::album::ALBUM_BY_ID_ROUTE).await?;
        
        let album_claims = CommonClaims::new(authenticated_user.id(), *album.visibility());
        let permission_request = self.kc_authz_service.permission_request(
            authenticated_user,
            album_claims,
            &resource_id,
            &[AuthorizationScope::View],
        );
        
        permission_request.decision_response_mode_send().await
    }

    async fn can_create_album(&self, authenticated_user: &AuthenticatedUser) -> anyhow::Result<bool> {       
        let resource_id = self.kc_authz_service.get_resource_id(routes::album::ALBUM_BY_ID_ROUTE).await?;

        let permission_request = self.kc_authz_service.permission_request(
            authenticated_user,
            (),
            &resource_id,
            &[AuthorizationScope::Create],
        );

        permission_request.decision_response_mode_send().await
    }

    async fn can_edit_album(&self, authenticated_user: &AuthenticatedUser, update_album: &UpdateAlbum) -> anyhow::Result<bool> {
        let resource_id = self.kc_authz_service.get_resource_id(routes::album::ALBUM_BY_ID_ROUTE).await?;

        let album_claims = CommonClaims::resource_owner(authenticated_user.id());
        let permission_request = self.kc_authz_service.permission_request(
            authenticated_user,
            album_claims,
            &resource_id,
            &Self::authorization_scopes(update_album),
        );

        permission_request.decision_response_mode_send().await
    }
}

impl AlbumPolicyEnforcerKc {

    fn authorization_scopes(update_album: &UpdateAlbum) -> Vec<AuthorizationScope> {
        let mut authorization_scopes = vec![];

        if let Some(_) = update_album.visibility() {
            authorization_scopes.push(AuthorizationScope::ChangeVisibility);
        }

        if let Some(_) = update_album.title() {
            authorization_scopes.push(AuthorizationScope::EditTitle);
        }

        return authorization_scopes;
    }
}