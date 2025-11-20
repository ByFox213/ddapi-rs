use crate::prelude::encode;
use crate::scheme::DDNET_BASE_URL;
use serde_derive::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QueryMap {
    pub name: String,
    pub r#type: String,
    pub mapper: String,
}

impl QueryMap {
    pub fn api(map: &str) -> String {
        format!("https://{}/maps/?query={}", DDNET_BASE_URL, encode(map))
    }
}
