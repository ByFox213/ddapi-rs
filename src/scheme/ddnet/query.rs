use crate::prelude::encode;
use crate::scheme::DDNET_BASE_URL;
use serde_derive::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Query {
    pub points: i64,
    pub name: String,
}

impl Query {
    pub fn api(player: &str) -> String {
        format!(
            "https://{}/players/?query={}",
            DDNET_BASE_URL,
            encode(player)
        )
    }
}
