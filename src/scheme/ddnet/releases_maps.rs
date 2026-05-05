use crate::scheme::DDNET_BASE_URL;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReleasesMaps {
    pub name: String,
    pub website: String,
    pub thumbnail: String,
    pub web_preview: String,
    pub r#type: String,
    pub points: u8,
    pub difficulty: u8,
    pub mapper: String,
    pub release: String,
    pub width: Option<u64>,
    pub height: Option<u64>,
    #[serde(default)]
    pub tiles: Vec<String>,
}

impl ReleasesMaps {
    pub fn url() -> String {
        format!("https://{}/releases", DDNET_BASE_URL)
    }

    pub fn api() -> String {
        format!("https://{}/releases/maps.json", DDNET_BASE_URL)
    }
}
