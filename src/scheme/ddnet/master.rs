use crate::prelude::Addr;
use crate::prelude::{addr_serialization, Protocol};
use crate::scheme::DDNET_BASE_URL;
use serde_derive::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

fn default_location() -> String {
    "unknown".to_string()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MasterServer {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
}

impl MasterServer {
    pub fn get_index(&self) -> i32 {
        *self as i32
    }

    pub fn api(&self) -> String {
        format!(
            "https://master{}.{}/ddnet/15/servers.json",
            self.get_index(),
            DDNET_BASE_URL
        )
    }
}

#[derive(Default, Debug, Clone)]
pub struct ClanCount {
    pub name: String,
    pub count: usize,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Master {
    pub communities: Vec<Community>,
    pub servers: Vec<Server>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Community {
    pub id: String,
    pub name: String,
    pub has_finishes: bool,
    pub icon: Icon,
    pub contact_urls: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Icon {
    pub sha256: String,
    pub url: String,
}

impl Master {
    pub fn api(master: MasterServer) -> String {
        master.api()
    }

    pub fn count_clients(&self) -> usize {
        self.servers.iter().map(|s| s.info.clients.len()).sum()
    }

    pub fn get_clans(&self) -> Vec<ClanCount> {
        self.get_filtered_clans(None)
    }

    pub fn get_filtered_clans(&self, filters: Option<Vec<&str>>) -> Vec<ClanCount> {
        if self.servers.is_empty() {
            return Vec::new();
        }

        let filter_set: HashSet<&str> = filters.unwrap_or_default().into_iter().collect();

        let mut clan_counts = HashMap::new();

        for server in &self.servers {
            for client in &server.info.clients {
                if !client.clan.is_empty() && !filter_set.contains(client.clan.as_str()) {
                    *clan_counts.entry(client.clan.clone()).or_insert(0) += 1;
                }
            }
        }

        let mut result: Vec<ClanCount> = clan_counts
            .into_iter()
            .map(|(name, count)| ClanCount { name, count })
            .collect();

        result.sort_by(|a, b| b.count.cmp(&a.count));
        result
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Server {
    #[serde(with = "addr_serialization")]
    pub addresses: Vec<Addr>,
    pub community: Option<String>,
    #[serde(default = "default_location")]
    pub location: String,
    pub info: Info,
}

impl Server {
    pub fn count_client(&self) -> usize {
        self.info.clients.len()
    }

    pub fn ipv4_addresses(&self) -> Vec<&Addr> {
        self.addresses
            .iter()
            .filter(|addr| addr.ip.is_ipv4())
            .collect()
    }

    pub fn ipv6_addresses(&self) -> Vec<&Addr> {
        self.addresses
            .iter()
            .filter(|addr| addr.ip.is_ipv6())
            .collect()
    }

    pub fn addresses_by_protocol(&self, protocol: Protocol) -> Vec<&Addr> {
        self.addresses
            .iter()
            .filter(|addr| addr.protocol == protocol)
            .collect()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Info {
    pub max_clients: i64,
    pub max_players: i64,
    #[serde(default)]
    pub passworded: bool,
    #[serde(rename = "game_type")]
    pub gametype: String,
    pub name: String,
    pub map: IMap,
    pub version: String,
    #[serde(default)]
    pub clients: Vec<Client>,
    #[serde(default)]
    pub requires_login: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IMap {
    pub name: String,
    pub sha256: Option<String>,
    pub size: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Client {
    pub name: String,
    pub clan: String,
    pub country: i32,
    pub score: i64,
    #[serde(default)]
    pub is_player: bool,
    pub skin: Option<Skin>,
    #[serde(default)]
    pub afk: bool,
    #[serde(default)]
    pub team: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Skin {
    pub name: Option<String>,
    pub color_body: Option<i64>,
    pub color_feet: Option<i64>,
}
