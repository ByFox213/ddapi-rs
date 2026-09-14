use crate::prelude::Addr;
use crate::prelude::{addr_serialization, Protocol};
use crate::scheme::DDNET_BASE_URL;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

fn default_location() -> String {
    "unknown".to_string()
}

/// The API sends `country` as a string on some servers and as an integer on
/// others; accept both and normalize to a string.
fn deserialize_optional_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error;
    let value = Option::<serde_json::Value>::deserialize(deserializer)?;
    Ok(match value {
        None => None,
        Some(serde_json::Value::String(s)) => Some(s),
        Some(serde_json::Value::Number(n)) => Some(n.to_string()),
        Some(other) => {
            return Err(D::Error::custom(format!(
                "expected string or number for country, got `{other}`"
            )))
        }
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MasterServer {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
}

impl MasterServer {
    #[must_use]
    pub fn get_index(&self) -> i32 {
        *self as i32
    }

    #[must_use]
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
    #[must_use]
    pub fn api(master: MasterServer) -> String {
        master.api()
    }

    #[must_use]
    pub fn count_clients(&self) -> usize {
        self.servers.iter().map(|s| s.info.clients.len()).sum()
    }

    #[must_use]
    pub fn get_clans(&self) -> Vec<ClanCount> {
        self.get_filtered_clans(None)
    }

    #[must_use]
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

        result.sort_by_key(|x| std::cmp::Reverse(x.count));
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
    #[must_use]
    pub fn count_client(&self) -> usize {
        self.info.clients.len()
    }

    #[must_use]
    pub fn ipv4_addresses(&self) -> Vec<&Addr> {
        self.addresses
            .iter()
            .filter(|addr| addr.ip.is_ipv4())
            .collect()
    }

    #[must_use]
    pub fn ipv6_addresses(&self) -> Vec<&Addr> {
        self.addresses
            .iter()
            .filter(|addr| addr.ip.is_ipv6())
            .collect()
    }

    #[must_use]
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
    #[serde(default)]
    pub client_score_kind: Option<String>,
    #[serde(default, deserialize_with = "deserialize_optional_string")]
    pub country: Option<String>,
    #[serde(default)]
    pub flags: Option<Vec<String>>,
    #[serde(default)]
    pub flag: Option<i64>,
    #[serde(default)]
    pub identity_key: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IMap {
    pub name: String,
    pub sha256: Option<String>,
    pub size: Option<i64>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default, deserialize_with = "deserialize_optional_string")]
    pub tw_crc: Option<String>,
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
    #[serde(default)]
    pub body: Option<SkinPart>,
    #[serde(default)]
    pub marking: Option<SkinPart>,
    #[serde(default)]
    pub decoration: Option<SkinPart>,
    #[serde(default)]
    pub eyes: Option<SkinPart>,
    #[serde(default)]
    pub feet: Option<SkinPart>,
    #[serde(default)]
    pub hands: Option<SkinPart>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SkinPart {
    pub name: String,
    #[serde(default)]
    pub color: Option<i64>,
}
