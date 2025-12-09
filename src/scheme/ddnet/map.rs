use crate::prelude::{encode, slugify2};
use crate::scheme::{deserialize_datetime_timestamp, serialize_datetime_timestamp, DDNET_BASE_URL};
use chrono::NaiveDateTime;
use serde_derive::{Deserialize, Serialize};

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
    pub average_time: f64,
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
        format!(
            "https://{}/maps/{}",
            DDNET_BASE_URL,
            encode(&slugify2(&self.name))
        )
    }

    pub fn url_with_name(map: &str) -> String {
        format!("https://{}/maps/{}", DDNET_BASE_URL, encode(&slugify2(map)))
    }

    pub fn api(map: &str) -> String {
        format!("https://{}/maps/?json={}", DDNET_BASE_URL, encode(map))
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
