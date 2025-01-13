use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::models::service::photo::{Photo, UpdatePhoto};
use crate::routes;
use crate::security::auth::user::AuthenticatedUser;
use crate::security::authz::claims::CommonClaims;
use crate::security::authz::kc_authz_service::{AuthorizationScope, AuthzPermissionRequest, KcAuthzService};
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

        let photo_claims = CommonClaims::new(photo.owner_user_id(), *photo.visibility());
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

    async fn can_edit_photo(&self, authenticated_user: &AuthenticatedUser, photo: &Photo, update_photo: &UpdatePhoto) -> anyhow::Result<bool> {
        let resource_id = self.kc_authz_service.get_resource_id(routes::photo::PHOTO_BY_ID_ROUTE).await?;

        let photo_claims = CommonClaims::resource_owner(photo.owner_user_id());
        let permission_request = self.kc_authz_service.permission_request(
            authenticated_user,
            photo_claims,
            &resource_id,
            &Self::authorization_scopes(update_photo),
        );

        permission_request.decision_response_mode_send().await
    }

    async fn filter_photos_by_view_permission<'a>(
        &self,
        authenticated_user: &AuthenticatedUser,
        photos: Vec<Photo>
    ) -> anyhow::Result<Vec<Photo>> {
        let resource_id = self.kc_authz_service.get_resource_id(routes::photo::PHOTO_BY_ID_ROUTE).await?;

        let tot_photos = photos.len();
        let join_handles: Vec<_> = photos
            .into_iter()
            .map(|photo| self.can_view_photo_permission_request(photo, authenticated_user, &resource_id))
            .map(|can_view_photo_permission_request| can_view_photo_permission_request.decision_response_mode_send())
            .map(|future| actix_web::rt::spawn(future))
            .collect();

        let mut authorized_photos = Vec::with_capacity(tot_photos);
        for join_handle in join_handles {
            if let Ok(Ok((can_view, photo))) = join_handle.await {
                if can_view {
                    authorized_photos.push(photo);
                }  
            };
        }
        
        Ok(authorized_photos)
    }
}

struct CanViewPhotoPermissionRequest(AuthzPermissionRequest<CommonClaims>, Photo);

impl CanViewPhotoPermissionRequest {
    async fn decision_response_mode_send(self) -> anyhow::Result<(bool, Photo)> {
        let can_view = self.0.decision_response_mode_send().await?;
        Ok((can_view, self.1))
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
    
    fn can_view_photo_permission_request(
        &self,
        photo: Photo,
        authenticated_user: &AuthenticatedUser,
        resource_id: &Uuid
    ) -> CanViewPhotoPermissionRequest {
        let photo_claims = CommonClaims::resource_owner(photo.owner_user_id());

        let permission_request = self.kc_authz_service.permission_request(
            authenticated_user,
            photo_claims,
            &resource_id,
            &[AuthorizationScope::View],
        );

        CanViewPhotoPermissionRequest(permission_request, photo)
    }
}