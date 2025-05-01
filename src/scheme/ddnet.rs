use crate::scheme::{deserialize_datetime_timestamp, serialize_datetime_timestamp};
use chrono::NaiveDateTime;
use serde_derive::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

fn default_location() -> String {
    "unknown".to_string()
}

#[allow(dead_code)]
pub type Query = Vec<QueryPlayer>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryPlayer {
    pub points: i64,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Master {
    pub servers: Vec<Server>,
}

impl Master {
    #[allow(dead_code)]
    pub fn count_clients(&self) -> usize {
        self.servers.iter().map(Server::count_client).sum()
    }

    #[allow(dead_code)]
    pub fn get_clans(&self, rm: Option<Vec<&str>>) -> Vec<(String, usize)> {
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

        let mut sorted_dat: Vec<_> = dat.into_iter().collect();
        sorted_dat.sort_by(|&(_, count1), &(_, count2)| count2.cmp(&count1));
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Map {
    pub name: String,
    pub website: String,
    pub thumbnail: String,
    pub web_preview: String,
    #[serde(rename = "type")]
    pub type_field: String,
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
