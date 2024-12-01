pub mod photo;

use std::fmt;

use image::ImageFormat;
use sqlx::types::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Image {
    id: Uuid,
    url: String,
    size: String,
    format: ImageFormat,
}

impl Image {
    pub fn id(&self) -> &Uuid {
        &self.id
    }
    pub fn url(&self) -> &str {
        &self.url
    }
    pub fn size(&self) -> &str {
        &self.size
    }
    pub fn format(&self) -> ImageFormat {
        self.format
    }
}
