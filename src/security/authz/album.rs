use std::sync::Arc;
use async_trait::async_trait;
use uuid::Uuid;
use crate::models::service::album::{Album, UpdateAlbum};
use crate::models::service::photo::Photo;
use crate::routes;
use crate::security::auth::user::AuthenticatedUser;
use crate::security::authz::AlbumPolicyEnforcer;
use crate::security::authz::claims::CommonClaims;
use crate::security::authz::kc_authz_service::{AuthorizationScope, AuthzPermissionRequest, KcAuthzService};

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
        
        let album_claims = CommonClaims::new(&album.owner_user_id(), *album.visibility());
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

    async fn can_edit_album(&self, authenticated_user: &AuthenticatedUser, album: &Album, update_album: &UpdateAlbum) -> anyhow::Result<bool> {
        let resource_id = self.kc_authz_service.get_resource_id(routes::album::ALBUM_BY_ID_ROUTE).await?;
        
        let album_claims = CommonClaims::resource_owner(&album.owner_user_id());
        let permission_request = self.kc_authz_service.permission_request(
            authenticated_user,
            album_claims,
            &resource_id,
            &Self::authorization_scopes(update_album),
        );

        permission_request.decision_response_mode_send().await
    }

    async fn filter_albums_by_view_permission(
        &self,
        authenticated_user: &AuthenticatedUser,
        albums: Vec<Album>
    ) -> anyhow::Result<Vec<Album>> {
        let resource_id = self.kc_authz_service.get_resource_id(routes::album::ALBUM_BY_ID_ROUTE).await?;

        let tot_albums = albums.len();
        let join_handles: Vec<_> = albums
            .into_iter()
            .map(|album| self.can_view_album_permission_request(album, authenticated_user, &resource_id))
            .map(|can_view_album_permission_request| can_view_album_permission_request.decision_response_mode_send())
            .map(|future| actix_web::rt::spawn(future))
            .collect();

        let mut authorized_albums = Vec::with_capacity(tot_albums);
        for join_handle in join_handles {
            if let Ok(Ok((can_view, album))) = join_handle.await {
                if can_view {
                    authorized_albums.push(album);
                }
            };
        }

        Ok(authorized_albums)
    }
}

struct CanViewAlbumPermissionRequest(AuthzPermissionRequest<CommonClaims>, Album);

impl CanViewAlbumPermissionRequest {
    async fn decision_response_mode_send(self) -> anyhow::Result<(bool, Album)> {
        let can_view = self.0.decision_response_mode_send().await?;
        Ok((can_view, self.1))
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
    
    fn can_view_album_permission_request(
        &self,
        album: Album,
        authenticated_user: &AuthenticatedUser,
        resource_id: &Uuid
    ) -> CanViewAlbumPermissionRequest {
        let album_claims = CommonClaims::resource_owner(&album.owner_user_id());

        let permission_request = self.kc_authz_service.permission_request(
            authenticated_user,
            album_claims,
            &resource_id,
            &[AuthorizationScope::View],
        );

        CanViewAlbumPermissionRequest(permission_request, album)
    }
}