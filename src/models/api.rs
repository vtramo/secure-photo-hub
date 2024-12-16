use serde::{Deserialize, Serialize};

use crate::models::service::Visibility;

pub mod photo;

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub enum VisibilityApi {
    #[serde(alias="public", alias="PUBLIC")]
    Public,

    #[serde(alias="private", alias="PRIVATE")]
    Private
}

impl From<VisibilityApi> for Visibility {
    fn from(visibility_api: VisibilityApi) -> Self {
        match visibility_api {
            VisibilityApi::Public => Visibility::Public,
            VisibilityApi::Private => Visibility::Private,
        }
    }
}

impl From<Visibility> for VisibilityApi {
    fn from(visibility: Visibility) -> Self {
        match visibility {
            Visibility::Public => VisibilityApi::Public,
            Visibility::Private => VisibilityApi::Private,
        }
    }
}
