use crate::prelude::encode;
use crate::scheme::DDNET_BASE_URL;
use serde_derive::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QueryMapper {
    pub mapper: String,
    pub num_maps: i64,
}

impl QueryMapper {
    pub fn api(player: &str) -> String {
        format!(
            "https://{}/maps/?qmapper={}",
            DDNET_BASE_URL,
            encode(player)
        )
    }
}
