use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;
use futures::lock::Mutex;
use anyhow::anyhow;
use async_trait::async_trait;
use serde::Serialize;
use uuid::Uuid;
use crate::models::service::photo::{Photo, UpdatePhoto};
use crate::models::service::Visibility;
use crate::routes;
use crate::security::auth::user::AuthenticatedUser;
use crate::security::authz::kc_authz_service::{AuthorizationScope, AuthzPermissionRequest, AuthzResourceSetRequest};
use crate::security::authz::PhotoPolicyEnforcer;
use crate::setup::OidcConfig;

#[derive(Clone)]
pub struct PhotoPolicyEnforcerKc {
    oidc_config: OidcConfig,
    kc_resource_cache: Arc<Mutex<HashMap<String, Uuid>>>
}

impl PhotoPolicyEnforcerKc {
    pub fn new(oidc_config: &OidcConfig) -> Self {
        Self { oidc_config: oidc_config.clone(), kc_resource_cache: Arc::new(Mutex::new(HashMap::new())) }
    }
}

#[async_trait()]
impl PhotoPolicyEnforcer for PhotoPolicyEnforcerKc {
    async fn can_view_photo(&self, authenticated_user: &AuthenticatedUser, photo: &Photo) -> anyhow::Result<bool> {
        let resource_id = self.get_resource_id(routes::photo::PHOTO_BY_ID_ROUTE).await?;

        let photo_claims = PhotoClaims::new(authenticated_user.id(), *photo.visibility());
        let permission_request = AuthzPermissionRequest::<PhotoClaims>::new(
            self.oidc_config.token_endpoint(),
            resource_id.as_ref(),
            authenticated_user.access_token(),
            self.oidc_config.client_id(),
            self.oidc_config.client_secret(),
            photo_claims,
            vec![AuthorizationScope::View]
        );

        permission_request.decision_response_mode_send().await
    }

    async fn can_create_photo(&self, authenticated_user: &AuthenticatedUser) -> anyhow::Result<bool> {
        dbg!("Can create photo...");
        let resource_id = self.get_resource_id(routes::photo::PHOTO_BY_ID_ROUTE).await?;
        dbg!("Resource id: {}", resource_id);

        let permission_request = AuthzPermissionRequest::<()>::new(
            self.oidc_config.token_endpoint(),
            resource_id.as_ref(),
            authenticated_user.access_token(),
            self.oidc_config.client_id(),
            self.oidc_config.client_secret(),
            (),
            vec![AuthorizationScope::Create]
        );

        permission_request.decision_response_mode_send().await
    }

    async fn can_edit_photo(&self, authenticated_user: &AuthenticatedUser, update_photo: &UpdatePhoto) -> anyhow::Result<bool> {
        let resource_id = self.get_resource_id(routes::photo::PHOTO_BY_ID_ROUTE).await?;

        let photo_claims = PhotoClaims::resource_owner(authenticated_user.id());
        let permission_request = AuthzPermissionRequest::<PhotoClaims>::new(
            self.oidc_config.token_endpoint(),
            resource_id.as_ref(),
            authenticated_user.access_token(),
            self.oidc_config.client_id(),
            self.oidc_config.client_secret(),
            photo_claims,
            Self::authorization_scopes(update_photo)
        );

        permission_request.decision_response_mode_send().await
    }
}

impl PhotoPolicyEnforcerKc {
    async fn get_resource_id(&self, path: &str) -> anyhow::Result<Uuid> {
        dbg!("Get resource id");
        let mut kc_resource_cache = self.kc_resource_cache.lock().await;
        dbg!("Lock!");
        let resource_set_endpoint = self.oidc_config.uma2_well_known_config().resource_registration_endpoint();
        dbg!("resource set endpoint {}", resource_set_endpoint);

        Ok(match kc_resource_cache.entry(path.to_string()) {
            Entry::Vacant(entry) => {
                dbg!("Vacant {}", &entry);
                let resource_ids = dbg!(AuthzResourceSetRequest::new(&resource_set_endpoint, routes::photo::PHOTO_BY_ID_ROUTE, 1, true).send().await)?;
                let resource_id = resource_ids.first().ok_or(anyhow!("Resource not found!"))?;
                entry.insert(resource_id.clone());
                resource_id.clone()
            },
            Entry::Occupied(resource_id) => resource_id.get().clone()
        })
    }

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

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct PhotoClaims {
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_owner: Option<Vec<Uuid>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    visibility: Option<Vec<Visibility>>,
}

impl PhotoClaims {
    pub fn new(resource_owner: &Uuid, visibility: Visibility) -> Self {
        Self { resource_owner: Some(vec![resource_owner.clone()]), visibility: Some(vec![visibility]) }
    }

    pub fn resource_owner(resource_owner: &Uuid) -> Self {
        Self { resource_owner: Some(vec![resource_owner.clone()]), visibility: None }
    }
}