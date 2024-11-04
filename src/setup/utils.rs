use reqwest::Url;
use serde::{de, Deserialize, Deserializer, Serializer};

pub fn deserialize_url<'de, D>(deserializer: D) -> Result<Url, D::Error>
    where
        D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    Url::parse(&s).map_err(de::Error::custom)
}

pub fn serialize_url<S>(url: &Url, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
{
    let url_str = url.as_str();
    serializer.serialize_str(url_str)
}