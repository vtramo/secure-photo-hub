use serde::{Deserialize, Serialize};
use crate::models::entity::VisibilityEntity;

pub mod entity;

#[derive(Clone, Debug, Copy, Serialize, Deserialize, PartialEq)]
enum Visibility {
    Public, Private
}

impl From<VisibilityEntity> for Visibility {
    fn from(value: VisibilityEntity) -> Self {
        match value {
            VisibilityEntity::Public => Visibility::Public,
            VisibilityEntity::Private => Visibility::Private,
        }
    }
}