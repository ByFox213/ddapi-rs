use serde::{Deserialize as SerdeDeserialize, Deserializer};
use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::collections::{HashMap, HashSet};

fn deserialize_timestamp<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let value: f64 = SerdeDeserialize::deserialize(deserializer)?;
    Ok(value as u64)
}

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
            if let Some(clients) = &server.info.clients {
                clients
                    .iter()
                    .filter(|client| !client.clan.is_empty())
                    .for_each(|client| {
                        *dat.entry(client.clan.clone()).or_insert(0) += 1;
                    });
            }
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
#[serde(rename_all = "camelCase")]
pub struct Server {
    pub addresses: Vec<String>,
    #[serde(default = "default_location")]
    pub location: String,
    pub info: Info,
}

impl Server {
    #[allow(dead_code)]
    pub fn count_client(&self) -> usize {
        self.info
            .clients
            .as_ref()
            .map_or(0, |clients| clients.len())
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    #[serde(rename = "max_clients")]
    pub max_clients: i64,
    #[serde(rename = "max_players")]
    pub max_players: i64,
    pub passworded: bool,
    #[serde(rename = "game_type")]
    pub game_type: String,
    pub name: String,
    pub map: Map,
    pub version: String,
    pub clients: Option<Vec<Client>>,
    #[serde(rename = "requires_login")]
    pub requires_login: Option<bool>,
    pub community: Option<Community>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Map {
    pub name: String,
    pub sha256: Option<String>,
    pub size: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Client {
    pub name: String,
    pub clan: String,
    pub country: i32,
    pub score: i64,
    pub is_player: Option<bool>,
    pub skin: Option<Skin>,
    pub afk: Option<bool>,
    pub team: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Skin {
    pub name: Option<String>,
    pub color_body: Option<i64>,
    pub color_feet: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Community {
    pub id: String,
    pub icon: String,
    pub admin: Vec<String>,
    pub public_key: Option<String>,
    pub signature: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DDPlayer {
    pub player: String,
    pub points: Points,
    #[serde(rename = "team_rank")]
    pub team_rank: Option<Rank>,
    pub rank: Option<Rank>,
    #[serde(rename = "points_last_year")]
    pub points_last_year: Option<Rank>,
    #[serde(rename = "points_last_month")]
    pub points_last_month: Option<Rank>,
    #[serde(rename = "points_last_week")]
    pub points_last_week: Option<Rank>,
    #[serde(rename = "favorite_server")]
    pub favorite_server: FavoriteServer,
    #[serde(rename = "first_finish")]
    pub first_finish: FirstFinish,
    #[serde(rename = "last_finishes")]
    pub last_finishes: Vec<LastFinish>,
    #[serde(rename = "favorite_partners")]
    pub favorite_partners: Option<Vec<FavoritePartner>>,
    pub types: Types,
    pub activity: Vec<Activity>,
    #[serde(rename = "hours_played_past_365_days")]
    pub hours_played_past_365_days: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DMap {
    pub name: String,
    pub website: String,
    pub thumbnail: String,
    #[serde(rename = "web_preview")]
    pub web_preview: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub points: i64,
    pub difficulty: i64,
    pub mapper: String,
    pub release: Option<f64>,
    #[serde(rename = "median_time")]
    pub median_time: f64,
    #[serde(rename = "first_finish")]
    pub first_finish: f64,
    #[serde(rename = "last_finish")]
    pub last_finish: f64,
    pub finishes: i64,
    pub finishers: i64,
    #[serde(rename = "biggest_team")]
    pub biggest_team: i64,
    pub width: i64,
    pub height: i64,
    pub tiles: Vec<String>,
    #[serde(rename = "team_ranks")]
    pub team_ranks: Vec<DTeamRank>,
    pub ranks: Vec<DRank>,
    #[serde(rename = "max_finishes")]
    pub max_finishes: Vec<MaxFinish>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DTeamRank {
    pub rank: i64,
    pub players: Vec<String>,
    pub time: f64,
    #[serde(deserialize_with = "deserialize_timestamp")]
    pub timestamp: u64,
    pub country: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DRank {
    pub rank: i64,
    pub player: String,
    pub time: f64,
    #[serde(deserialize_with = "deserialize_timestamp")]
    pub timestamp: u64,
    pub country: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaxFinish {
    pub rank: i64,
    pub player: String,
    pub num: i64,
    pub time: f64,
    #[serde(rename = "min_timestamp")]
    #[serde(deserialize_with = "deserialize_timestamp")]
    pub min_timestamp: u64,
    #[serde(rename = "max_timestamp")]
    #[serde(deserialize_with = "deserialize_timestamp")]
    pub max_timestamp: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Points {
    pub total: u64,
    pub points: Option<u64>,
    pub rank: Option<u64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rank {
    pub points: Option<u64>,
    pub rank: Option<u64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FavoriteServer {
    pub server: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FirstFinish {
    #[serde(deserialize_with = "deserialize_timestamp")]
    pub timestamp: u64,
    pub map: String,
    pub time: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LastFinish {
    #[serde(deserialize_with = "deserialize_timestamp")]
    pub timestamp: u64,
    pub map: String,
    pub time: f64,
    pub country: String,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FavoritePartner {
    pub name: String,
    pub finishes: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Types {
    #[serde(rename = "Novice")]
    pub novice: TypeDDType,
    #[serde(rename = "Moderate")]
    pub moderate: TypeDDType,
    #[serde(rename = "Brutal")]
    pub brutal: TypeDDType,
    #[serde(rename = "Insane")]
    pub insane: TypeDDType,
    #[serde(rename = "Dummy")]
    pub dummy: TypeDDType,
    #[serde(rename = "DDmaX.Easy")]
    pub ddma_x_easy: TypeDDType,
    #[serde(rename = "DDmaX.Next")]
    pub ddma_x_next: TypeDDType,
    #[serde(rename = "DDmaX.Pro")]
    pub ddma_x_pro: TypeDDType,
    #[serde(rename = "DDmaX.Nut")]
    pub ddma_x_nut: TypeDDType,
    #[serde(rename = "Oldschool")]
    pub oldschool: TypeDDType,
    #[serde(rename = "Solo")]
    pub solo: TypeDDType,
    #[serde(rename = "Race")]
    pub race: TypeDDType,
    #[serde(rename = "Fun")]
    pub fun: TypeDDType,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypeDDType {
    pub points: Points,
    #[serde(rename = "team_rank")]
    pub team_rank: Option<Rank>,
    pub rank: Option<Rank>,
    pub maps: HashMap<String, DDMap>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DDMap {
    pub points: i64,
    #[serde(rename = "total_finishes")]
    pub total_finishes: i64,
    pub finishes: i64,
    #[serde(rename = "team_rank")]
    pub team_rank: Option<i64>,
    pub rank: Option<i64>,
    pub time: Option<f64>,
    #[serde(rename = "first_finish")]
    pub first_finish: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Activity {
    pub date: String,
    #[serde(rename = "hours_played")]
    pub hours_played: i64,
}
