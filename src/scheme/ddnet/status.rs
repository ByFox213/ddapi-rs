use crate::scheme::DDNET_BASE_URL;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct StatusData {
    pub name: String,
    pub r#type: String,
    pub host: String,
    pub location: String,
    pub online4: bool,
    pub online6: bool,
    pub uptime: String,
    pub load: f32,
    pub network_rx: u64,
    pub network_tx: u64,
    pub packets_rx: u64,
    pub packets_tx: u64,
    pub cpu: u32,
    pub memory_total: u64,
    pub memory_used: u64,
    pub swap_total: u64,
    pub swap_used: u64,
    pub hdd_total: u64,
    pub hdd_used: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Status {
    pub servers: Vec<StatusData>,
    pub updated: String,
}

impl Status {
    pub fn url() -> String {
        format!("https://{}/status", DDNET_BASE_URL)
    }

    pub fn api() -> String {
        format!("https://{}/status/json/stats.json", DDNET_BASE_URL)
    }
}
