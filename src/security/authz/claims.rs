use serde::Serialize;
use uuid::Uuid;
use crate::models::service::Visibility;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommonClaims {
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_owner: Option<Vec<Uuid>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    visibility: Option<Vec<Visibility>>,
}

impl CommonClaims {
    pub fn new(resource_owner: &Uuid, visibility: Visibility) -> Self {
        Self { resource_owner: Some(vec![resource_owner.clone()]), visibility: Some(vec![visibility]) }
    }

    pub fn resource_owner(resource_owner: &Uuid) -> Self {
        Self { resource_owner: Some(vec![resource_owner.clone()]), visibility: None }
    }
}