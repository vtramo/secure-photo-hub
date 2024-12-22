use serde::{Deserialize, Serialize};

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
