use crate::scheme::DDNET_BASE_URL;
use serde_derive::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DDSkinHD {
    pub uhd: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DDSkin {
    pub name: String,
    pub r#type: String,
    pub hd: DDSkinHD,
    pub creator: String,
    pub license: String,
    pub bodypart: String,
    pub gameversion: String,
    pub date: String,
    pub skinpack: String,
    pub imgtype: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DDSkins {
    pub skins: Vec<DDSkin>,
    pub version: String,
}

impl DDSkins {
    pub fn api() -> String {
        format!("https://skins.{}/skin/skins.json", DDNET_BASE_URL)
    }
}
