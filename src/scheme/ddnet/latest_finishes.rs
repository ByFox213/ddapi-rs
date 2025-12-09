use crate::scheme::DDNET_BASE_URL;
use crate::scheme::{deserialize_datetime_timestamp, serialize_datetime_timestamp};
use chrono::NaiveDateTime;
use serde_derive::{Deserialize, Serialize};

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
    pub fn url() -> String {
        format!("https://{}/ranks", DDNET_BASE_URL)
    }

    pub fn api(latest: usize) -> String {
        format!("https://{}/maps/?latest={}", DDNET_BASE_URL, latest)
    }
}
