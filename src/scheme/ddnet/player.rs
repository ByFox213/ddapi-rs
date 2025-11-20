use crate::prelude::{encode, slugify2};
use crate::scheme::{deserialize_datetime_timestamp, serialize_datetime_timestamp, DDNET_BASE_URL};
use chrono::NaiveDateTime;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

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
    #[serde(default)]
    pub favorite_partners: Vec<FavoritePartner>,
    pub types: Types,
    pub activity: Vec<Activity>,
    pub hours_played_past_365_days: i64,
}

impl Player {
    pub fn url(&self) -> String {
        format!(
            "https://{}/players/{}",
            DDNET_BASE_URL,
            encode(&slugify2(&self.player))
        )
    }

    pub fn url_with_name(player: &str) -> String {
        format!(
            "https://{}/players/{}",
            DDNET_BASE_URL,
            encode(&slugify2(player))
        )
    }

    pub fn api(player: &str) -> String {
        format!(
            "https://{}/players/?json2={}",
            DDNET_BASE_URL,
            encode(player)
        )
    }
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
    pub ddmax_easy: Type,
    #[serde(rename = "DDmaX.Next")]
    pub ddmax_next: Type,
    #[serde(rename = "DDmaX.Pro")]
    pub ddmax_pro: Type,
    #[serde(rename = "DDmaX.Nut")]
    pub ddmax_nut: Type,
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
