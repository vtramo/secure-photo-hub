use std::fmt;
use serde::{Deserialize, Serialize};

pub mod photo;
pub mod album;
pub mod pagination;
pub mod image;

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq)]
pub enum Visibility {
    Public,
    Private,
}

impl From<String> for Visibility {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Public" => Self::Public,
            "Private" => Self::Private,
            _ => panic!("impl From<String> for Visibility"),
        }
    }
}

impl fmt::Display for Visibility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let visibility_str = match *self {
            Visibility::Public => "Public",
            Visibility::Private => "Private",
        };
        write!(f, "{}", visibility_str)
    }
}