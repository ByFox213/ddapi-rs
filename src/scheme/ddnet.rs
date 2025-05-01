use crate::scheme::{deserialize_datetime_timestamp, serialize_datetime_timestamp};
use crate::util::encoding::{encode, slugify2};
use chrono::NaiveDateTime;
use serde_derive::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

fn default_location() -> String {
    "unknown".to_string()
}

pub enum MasterServer {
    One,
    Two,
    Three,
    Four,
}

impl MasterServer {
    fn get_index(&self) -> i32 {
        match &self {
            Self::One => 1,
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Query {
    pub points: i64,
    pub name: String,
}

impl Query {
    pub fn api(player: &str) -> String {
        format!("https://ddnet.org/players/?query={}", encode(player))
    }
}

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
    pub width: u64,
    pub height: u64,
    pub tiles: Vec<String>,
}

impl ReleasesMaps {
    pub fn api() -> String {
        "https://ddnet.org/releases/maps.json".to_string()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    pub fn api() -> String {
        "https://ddnet.org/status/json/stats.json".to_string()
    }
}

#[derive(Default, Debug, Clone)]
pub struct ClansCount {
    pub name: String,
    pub count: usize,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Master {
    pub servers: Vec<Server>,
}

impl Master {
    pub fn api(master: MasterServer) -> String {
        format!(
            "https://master{}.ddnet.org/ddnet/15/servers.json",
            master.get_index()
        )
    }

    pub fn count_clients(&self) -> usize {
        self.servers.iter().map(Server::count_client).sum()
    }

    pub fn get_clans(&self) -> Vec<ClansCount> {
        self.get_clans_with_custom_rm(None)
    }

    pub fn get_clans_with_custom_rm(&self, rm: Option<Vec<&str>>) -> Vec<ClansCount> {
        let remove_list: HashSet<&str> = rm
            .unwrap_or_else(|| vec!["DD-Persian", "/vDQMHSss8W"])
            .into_iter()
            .collect();

        if self.servers.is_empty() {
            return vec![];
        }

        let mut dat: HashMap<String, usize> = HashMap::new();

        self.servers.iter().for_each(|server| {
            server
                .info
                .clients
                .iter()
                .filter(|client| !client.clan.is_empty())
                .for_each(|client| {
                    *dat.entry(client.clan.clone()).or_insert(0) += 1;
                });
        });

        for clan in remove_list {
            dat.remove(clan);
        }

        let mut sorted_dat: Vec<ClansCount> = dat
            .into_iter()
            .map(|(name, count)| ClansCount { name, count })
            .collect();

        sorted_dat.sort_by(|a, b| b.count.cmp(&a.count));
        sorted_dat
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Server {
    pub addresses: Vec<String>,
    #[serde(default = "default_location")]
    pub location: String,
    pub info: Info,
}

impl Server {
    #[allow(dead_code)]
    pub fn count_client(&self) -> usize {
        self.info.clients.len()
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
    pub community: Option<Community>,
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Community {
    pub id: String,
    pub icon: String,
    pub admin: Vec<String>,
    pub public_key: Option<String>,
    pub signature: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Player {
    pub player: String,
    pub points: Points,
    pub team_rank: Option<Rank>,
    pub rank: Option<Rank>,
    pub points_last_year: Option<Rank>,
    pub points_last_month: Option<Rank>,
    pub points_last_week: Option<Rank>,
    pub favorite_server: FavoriteServer,
    pub first_finish: FirstFinish,
    pub last_finishes: Vec<LastFinish>,
    pub favorite_partners: Option<Vec<FavoritePartner>>,
    pub types: Types,
    pub activity: Vec<Activity>,
    pub hours_played_past_365_days: i64,
}

impl Player {
    pub fn url(&self) -> String {
        format!(
            "https://ddnet.org/players/{}",
            encode(&slugify2(&self.player))
        )
    }

    pub fn url_with_name(player: &str) -> String {
        format!("https://ddnet.org/players/{}", encode(&slugify2(player)))
    }

    pub fn api(player: &str) -> String {
        format!("https://ddnet.org/players/?json2={}", encode(player))
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Map {
    pub name: String,
    pub website: String,
    pub thumbnail: String,
    pub web_preview: String,
    pub r#type: String,
    pub points: i64,
    pub difficulty: i64,
    pub mapper: String,
    pub release: Option<f64>,
    pub median_time: f64,
    pub first_finish: f64,
    pub last_finish: f64,
    pub finishes: i64,
    pub finishers: i64,
    pub biggest_team: i64,
    pub width: i64,
    pub height: i64,
    pub tiles: Vec<String>,
    pub team_ranks: Vec<DTeamRank>,
    pub ranks: Vec<DRank>,
    pub max_finishes: Vec<MaxFinish>,
}

impl Map {
    pub fn url(&self) -> String {
        format!("https://ddnet.org/maps/{}", encode(&slugify2(&self.name)))
    }

    pub fn url_with_name(map: &str) -> String {
        format!("https://ddnet.org/maps/{}", encode(&slugify2(map)))
    }

    pub fn api(map: &str) -> String {
        format!("https://ddnet.org/maps/?json={}", encode(map))
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DTeamRank {
    pub rank: i64,
    pub players: Vec<String>,
    pub time: f64,
    #[serde(
        serialize_with = "serialize_datetime_timestamp",
        deserialize_with = "deserialize_datetime_timestamp"
    )]
    pub timestamp: NaiveDateTime,
    pub country: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DRank {
    pub rank: i64,
    pub player: String,
    pub time: f64,
    #[serde(
        serialize_with = "serialize_datetime_timestamp",
        deserialize_with = "deserialize_datetime_timestamp"
    )]
    pub timestamp: NaiveDateTime,
    pub country: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MaxFinish {
    pub rank: i64,
    pub player: String,
    pub num: i64,
    pub time: f64,
    #[serde(
        serialize_with = "serialize_datetime_timestamp",
        deserialize_with = "deserialize_datetime_timestamp"
    )]
    pub min_timestamp: NaiveDateTime,
    #[serde(
        serialize_with = "serialize_datetime_timestamp",
        deserialize_with = "deserialize_datetime_timestamp"
    )]
    pub max_timestamp: NaiveDateTime,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Points {
    pub total: u64,
    pub points: Option<u64>,
    pub rank: Option<u64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rank {
    pub points: Option<u64>,
    pub rank: Option<u64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FavoriteServer {
    pub server: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FirstFinish {
    #[serde(
        serialize_with = "serialize_datetime_timestamp",
        deserialize_with = "deserialize_datetime_timestamp"
    )]
    pub timestamp: NaiveDateTime,
    pub map: String,
    pub time: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LastFinish {
    #[serde(
        serialize_with = "serialize_datetime_timestamp",
        deserialize_with = "deserialize_datetime_timestamp"
    )]
    pub timestamp: NaiveDateTime,
    pub map: String,
    pub time: f64,
    pub country: String,
    #[serde(rename = "type")]
    pub type_map: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FavoritePartner {
    pub name: String,
    pub finishes: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Types {
    pub novice: Type,
    pub moderate: Type,
    pub brutal: Type,
    pub insane: Type,
    pub dummy: Type,
    #[serde(rename = "DDmaX.Easy")]
    pub ddma_x_easy: Type,
    #[serde(rename = "DDmaX.Next")]
    pub ddma_x_next: Type,
    #[serde(rename = "DDmaX.Pro")]
    pub ddma_x_pro: Type,
    #[serde(rename = "DDmaX.Nut")]
    pub ddma_x_nut: Type,
    pub oldschool: Type,
    pub solo: Type,
    pub race: Type,
    pub fun: Type,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Type {
    pub points: Points,
    pub team_rank: Option<Rank>,
    pub rank: Option<Rank>,
    pub maps: HashMap<String, DDMap>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DDMap {
    pub points: i64,
    pub total_finishes: i64,
    pub finishes: i64,
    pub team_rank: Option<i64>,
    pub rank: Option<i64>,
    pub time: Option<f64>,
    pub first_finish: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Activity {
    pub date: String,
    pub hours_played: i64,
}
