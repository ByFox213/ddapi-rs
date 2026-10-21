use crate::prelude::encode;
use crate::scheme::DDNET_BASE_URL;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QueryMap {
    pub name: String,
    pub r#type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mapper: Option<String>,
}

impl QueryMap {
    #[must_use]
    pub fn api(map: &str) -> String {
        format!("https://{}/maps/?query={}", DDNET_BASE_URL, encode(map))
    }
}
