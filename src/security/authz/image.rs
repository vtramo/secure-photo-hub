use std::sync::Arc;
use async_trait::async_trait;
use crate::models::service::image::{ImageReference};
use crate::routes;
use crate::security::auth::user::AuthenticatedUser;
use crate::security::authz::{AuthorizationScope, ImagePolicyEnforcer, KcAuthzService};
use crate::security::authz::claims::CommonClaims;

#[derive(Clone)]
pub struct ImagePolicyEnforcerKc {
    kc_authz_service: Arc<KcAuthzService>,
}

impl ImagePolicyEnforcerKc {
    pub fn new(kc_authz_service: Arc<KcAuthzService>) -> Self {
        Self { kc_authz_service }
    }
}

#[async_trait()]
impl ImagePolicyEnforcer for ImagePolicyEnforcerKc {
    async fn can_download(&self, authenticated_user: &AuthenticatedUser, image_reference: &ImageReference) -> anyhow::Result<bool> {
        let resource_id = self.kc_authz_service.get_resource_id(routes::image::IMAGE_BY_ID_ROUTE).await?;

        let image_claims = CommonClaims::new(image_reference.owner_user_id(), image_reference.visibility());
        let permission_request = self.kc_authz_service.permission_request(
            authenticated_user,
            image_claims,
            &resource_id,
            &[AuthorizationScope::Download],
        );
        
        permission_request.decision_response_mode_send().await
    }

    async fn can_transform(&self, authenticated_user: &AuthenticatedUser, image_reference: &ImageReference) -> anyhow::Result<bool> {
        let resource_id = self.kc_authz_service.get_resource_id(routes::image::IMAGE_BY_ID_ROUTE).await?;

        let image_claims = CommonClaims::new(image_reference.owner_user_id(), image_reference.visibility());
        let permission_request = self.kc_authz_service.permission_request(
            authenticated_user,
            image_claims,
            &resource_id,
            &[AuthorizationScope::Transform],
        );

        permission_request.decision_response_mode_send().await
    }

    async fn can_download_then_transform(&self, authenticated_user: &AuthenticatedUser, image_reference: &ImageReference) -> anyhow::Result<bool> {
        let resource_id = self.kc_authz_service.get_resource_id(routes::image::IMAGE_BY_ID_ROUTE).await?;

        let image_claims = CommonClaims::new(image_reference.owner_user_id(), image_reference.visibility());
        let permission_request = self.kc_authz_service.permission_request(
            authenticated_user,
            image_claims,
            &resource_id,
            &[AuthorizationScope::Download, AuthorizationScope::Transform],
        );

        permission_request.decision_response_mode_send().await
    }

    async fn can_create(&self, authenticated_user: &AuthenticatedUser) -> anyhow::Result<bool> {
        let resource_id = self.kc_authz_service.get_resource_id(routes::image::IMAGE_BY_ID_ROUTE).await?;

        let permission_request = self.kc_authz_service.permission_request(
            authenticated_user,
            (),
            &resource_id,
            &[AuthorizationScope::Download, AuthorizationScope::Transform],
        );

        permission_request.decision_response_mode_send().await
    }

    async fn can_view(&self, authenticated_user: &AuthenticatedUser, image_reference: &ImageReference) -> anyhow::Result<bool> {
        let resource_id = self.kc_authz_service.get_resource_id(routes::image::IMAGE_BY_ID_ROUTE).await?;
        
        let image_claims = CommonClaims::new(image_reference.owner_user_id(), image_reference.visibility());
        let permission_request = self.kc_authz_service.permission_request(
            authenticated_user,
            image_claims,
            &resource_id,
            &[AuthorizationScope::View],
        );

        permission_request.decision_response_mode_send().await
    }
}