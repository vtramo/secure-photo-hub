mod resource_set;
mod permission_request;

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};
use anyhow::anyhow;
use async_trait::async_trait;
use futures_util::lock::Mutex;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
pub use permission_request::{AuthzPermissionRequest};
pub use resource_set::{AuthzResourceSetRequest};
use crate::security::auth::oauth::{OAuthAccessTokenHolder, OAuthClientSession};
use crate::security::auth::user::AuthenticatedUser;
use crate::setup::OidcConfig;

#[derive(Deserialize, Copy, Clone, Debug)]
pub enum AuthorizationScope {
    View,
    ViewAll,
    ViewOwn,
    Create,
    Transform,
    Download,
    ChangeAlbum,
    ChangeVisibility,
    EditTitle,
}

impl Display for AuthorizationScope {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AuthorizationScope::View => f.write_str("View"),
            AuthorizationScope::ViewAll => f.write_str("ViewAll"),
            AuthorizationScope::ViewOwn => f.write_str("ViewOwn"),
            AuthorizationScope::Create => f.write_str("Create"),
            AuthorizationScope::Transform => f.write_str("Transform"),
            AuthorizationScope::Download => f.write_str("Download"),
            AuthorizationScope::ChangeAlbum => f.write_str("ChangeAlbum"),
            AuthorizationScope::ChangeVisibility => f.write_str("ChangeVisibility"),
            AuthorizationScope::EditTitle => f.write_str("EditTitle"),
        }
    }
}

pub struct KcAuthzService {
    oidc_config: OidcConfig,
    kc_resource_cache: Mutex<HashMap<String, Uuid>>,
    client_session: Mutex<OAuthClientSession>,    
}

impl KcAuthzService {
    pub fn new(oidc_config: &OidcConfig, client_session: OAuthClientSession) -> Self {
        Self {
            oidc_config: oidc_config.clone(),
            kc_resource_cache: Mutex::new(HashMap::new()),
            client_session: Mutex::new(client_session),
        }
    }

    pub async fn get_resource_id(&self, path: &str) -> anyhow::Result<Uuid> {
        let mut kc_resource_cache = self.kc_resource_cache.lock().await;
        let resource_set_endpoint = self.oidc_config.uma2_well_known_config().resource_registration_endpoint();

        Ok(match kc_resource_cache.entry(path.to_string()) {
            Entry::Vacant(entry) => {
                let resource_ids = AuthzResourceSetRequest::new(
                    &resource_set_endpoint,
                    self.get_access_token().await?.as_str(),
                    path,
                    1,
                    false
                ).send().await?;

                let resource_id = resource_ids.first().ok_or(anyhow!("Resource not found!"))?;
                entry.insert(resource_id.clone());
                resource_id.clone()
            },
            
            Entry::Occupied(resource_id) => resource_id.get().clone()
        })
    }
    
    pub fn permission_request<T: Serialize>(
        &self, authenticated_user: &AuthenticatedUser, 
        claims: T,
        resource_id: &Uuid,
        authorization_scopes: &[AuthorizationScope]
    ) -> AuthzPermissionRequest<T> {
        return AuthzPermissionRequest::<T>::new(
            self.oidc_config.token_endpoint(),
            resource_id.as_ref(),
            authenticated_user.access_token(),
            self.oidc_config.client_id(),
            self.oidc_config.client_secret(),
            claims,
            authorization_scopes.to_vec()
        );
    }
}

#[async_trait()]
impl OAuthAccessTokenHolder for KcAuthzService {
    async fn get_access_token(&self) -> anyhow::Result<String> {
        let mut mutex_guard = self.client_session.lock().await;
        Ok(mutex_guard.get_access_token().await?)
    }
}