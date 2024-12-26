use serde::{Deserialize, Serialize};
use crate::models::service::image::ImageTransformOptions;

use crate::models::service::Visibility;

pub mod photo;
pub mod album;

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

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct ImageTransformOptionsApi {
    huerotate: Option<i32>,
    #[serde(deserialize_with = "serde_tuple::deserialize_tuple")]
    #[serde(default)]
    thumbnail: Option<(u32, u32)>
}

impl From<ImageTransformOptionsApi> for ImageTransformOptions {
    fn from(convert_options_api: ImageTransformOptionsApi) -> Self {
        Self::new(convert_options_api.huerotate, convert_options_api.thumbnail)
    }
}

pub mod serde_date {
    use chrono::{DateTime, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(dt: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_str(&dt.to_rfc3339())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        DateTime::parse_from_rfc3339(&s)
            .map(|dt| dt.with_timezone(&Utc))
            .map_err(serde::de::Error::custom)
    }
}

pub mod serde_url {
    use serde::{self, Deserialize, Deserializer, Serializer};
    use url::Url;

    pub fn serialize<S>(url: &Url, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_str(url.as_str())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Url, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        Url::parse(&s).map_err(serde::de::Error::custom)
    }
}

pub mod serde_tuple {
    use serde::{Deserialize, Deserializer};

    pub fn deserialize_tuple<'de, D>(deserializer: D) -> Result<Option<(u32, u32)>, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s: Option<String> = Option::deserialize(deserializer)?;

        // If the value is `None`, return None
        if let Some(value) = s {
            // Split the string by ',' and attempt to parse the two numbers
            let parts: Vec<&str> = value.split(',').collect();
            if parts.len() == 2 {
                let width = parts[0].parse().map_err(serde::de::Error::custom)?;
                let height = parts[1].parse().map_err(serde::de::Error::custom)?;
                return Ok(Some((width, height)));
            } else {
                return Err(serde::de::Error::custom("expected two comma-separated values"));
            }
        }
        Ok(None)
    }
}