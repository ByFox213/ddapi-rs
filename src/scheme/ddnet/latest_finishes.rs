use crate::scheme::DDNET_BASE_URL;
use crate::scheme::{deserialize_datetime_timestamp, serialize_datetime_timestamp};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LatestFinishes {
    #[serde(
        serialize_with = "serialize_datetime_timestamp",
        deserialize_with = "deserialize_datetime_timestamp"
    )]
    pub timestamp: NaiveDateTime,
    pub map: String,
    pub name: String,
    pub time: f64,
    pub server: String,
}

impl LatestFinishes {
    #[must_use]
    pub fn url() -> String {
        format!("https://{DDNET_BASE_URL}/ranks")
    }

    #[must_use]
    pub fn api(latest: usize) -> String {
        format!("https://{DDNET_BASE_URL}/maps/?latest={latest}")
    }
}
