mod resource_set;
mod permission_request;

use std::fmt;
use std::fmt::{Display, Formatter};
use serde::Deserialize;
pub use permission_request::{AuthzPermissionRequest};
pub use resource_set::{AuthzResourceSetRequest};

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