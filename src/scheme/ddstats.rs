use crate::scheme::DDSTATS_BASE_URL;
use crate::util::prelude::{encode, seconds_to_hours, slugify2};
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Player {
    pub points_graph: Vec<PointsGraph>,
    pub recent_finishes: Vec<RecentFinish>,
    pub favourite_teammates: Vec<FavouriteTeammate>,
    pub profile: Profile,
    pub is_mapper: bool,
    pub finishes: Vec<Finish>,
    pub unfinished_maps: Vec<UnfinishedMap>,
    pub points: Points,
    #[serde(default)]
    pub completion_progress: Vec<CompletionProgress>,
    pub recent_activity: Vec<RecentActivity>,
    pub recent_player_info: Vec<RecentPlayerInfo>,
    pub most_played_maps: Vec<MostPlayedMap>,
    pub most_played_gametypes: Vec<MostPlayed>,
    pub most_played_categories: Vec<MostPlayed>,
    pub most_played_locations: Vec<MostPlayed>,
    pub playtime_per_month: Vec<PlaytimePerMonth>,
    pub general_activity: Option<GeneralActivity>,
    pub favourite_rank1s_teammates: Vec<FavouriteRank1sTeammates>,
    pub all_top_10s: Vec<AllTop10>,
    pub recent_top_10s: Vec<RecentTop10>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompletionProgress {
    pub category: String,
    pub maps_finished: i64,
    pub maps_total: i64,
}

impl Player {
    #[must_use]
    pub fn url(&self) -> String {
        format!(
            "https://{}/player/{}",
            DDSTATS_BASE_URL,
            encode(&self.profile.name)
        )
    }

    #[must_use]
    pub fn url_with_name(player: &str) -> String {
        format!("https://{}/player/{}", DDSTATS_BASE_URL, encode(player))
    }

    #[must_use]
    pub fn api(player: &str) -> String {
        format!(
            "https://{}/player/json?player={}",
            DDSTATS_BASE_URL,
            encode(player)
        )
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PointsGraph {
    pub date: NaiveDate,
    pub points: i64,
    pub rank_points: i64,
    pub team_points: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecentFinish {
    pub map: StatsMap,
    pub name: String,
    pub time: f64,
    pub timestamp: NaiveDateTime,
    pub server: String,
    pub cp1: f64,
    pub cp2: f64,
    pub cp3: f64,
    pub cp4: f64,
    pub cp5: f64,
    pub cp6: f64,
    pub cp7: f64,
    pub cp8: f64,
    pub cp9: f64,
    pub cp10: f64,
    pub cp11: f64,
    pub cp12: f64,
    pub cp13: f64,
    pub cp14: f64,
    pub cp15: f64,
    pub cp16: f64,
    pub cp17: f64,
    pub cp18: f64,
    pub cp19: f64,
    pub cp20: f64,
    pub cp21: f64,
    pub cp22: f64,
    pub cp23: f64,
    pub cp24: f64,
    pub cp25: f64,
    pub team_rank: Option<TeamRankingSMap>,
    pub rank: Option<RankingSMap>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatsMap {
    pub map: String,
    pub server: String,
    pub points: i32,
    pub stars: i32,
    pub mapper: String,
    pub timestamp: Option<NaiveDateTime>,
}

impl StatsMap {
    #[must_use]
    pub fn url(&self) -> String {
        format!(
            "https://{}/map/{}",
            DDSTATS_BASE_URL,
            encode(&slugify2(&self.map))
        )
    }

    #[must_use]
    pub fn url_with_name(map: &str) -> String {
        format!(
            "https://{}/map/{}",
            DDSTATS_BASE_URL,
            encode(&slugify2(map))
        )
    }

    #[must_use]
    pub fn api() -> String {
        format!("https://{DDSTATS_BASE_URL}/maps/json")
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FavouriteTeammate {
    pub name: String,
    pub ranks_together: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    pub points: i32,
    pub clan: Option<String>,
    pub country: Option<i32>,
    pub skin_name: Option<String>,
    pub skin_color_body: Option<i32>,
    pub skin_color_feet: Option<i32>,
    pub most_played_location: Option<String>,
}

impl Profile {
    #[must_use]
    pub fn url(&self) -> String {
        format!("https://{}/player/{}", DDSTATS_BASE_URL, encode(&self.name))
    }

    #[must_use]
    pub fn url_with_name(player: &str) -> String {
        format!("https://{}/player/{}", DDSTATS_BASE_URL, encode(player))
    }

    #[must_use]
    pub fn api(player: &str) -> String {
        format!(
            "https://{}/profile/json?player={}",
            DDSTATS_BASE_URL,
            encode(player)
        )
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Finish {
    pub map: StatsMap,
    pub name: String,
    pub time: f64,
    pub timestamp: NaiveDateTime,
    pub server: String,
    pub rank: i32,
    pub team_rank: Option<i32>,
    pub seconds_played: Option<i32>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnfinishedMap {
    pub map: StatsMap,
    pub finishes: Option<i32>,
    pub finishes_rank: Option<i32>,
    pub median_time: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Points {
    pub weekly_points: Option<LeaderboardRank>,
    pub monthly_points: Option<LeaderboardRank>,
    pub yearly_points: Option<LeaderboardRank>,
    pub points: HashMap<Category, Option<LeaderboardRank>>,
    pub rank_points: HashMap<Category, Option<LeaderboardRank>>,
    pub team_points: HashMap<Category, Option<LeaderboardRank>>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LeaderboardRank {
    pub points: u64,
    pub rank: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Category {
    All,
    Total,
    Novice,
    Moderate,
    Brutal,
    Insane,
    Dummy,
    #[serde(rename = "DDmaX.Easy")]
    DDmaXEasy,
    #[serde(rename = "DDmaX.Next")]
    DDmaXNext,
    #[serde(rename = "DDmaX.Pro")]
    DDmaXPro,
    #[serde(rename = "DDmaX.Nut")]
    DDmaXNut,
    Oldschool,
    Solo,
    Race,
    Fun,
    Event,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecentActivity {
    pub name: String,
    pub date: NaiveDate,
    pub map_name: String,
    pub map: Option<StatsMap>,
    pub seconds_played: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecentPlayerInfo {
    pub name: String,
    pub clan: String,
    pub country: i32,
    pub skin_name: String,
    pub skin_color_body: Option<i32>,
    pub skin_color_feet: Option<i32>,
    pub last_seen: NaiveDate,
    pub seconds_played: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MostPlayedMap {
    pub map_name: String,
    pub seconds_played: i64,
    pub map: Option<StatsMap>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MostPlayed {
    pub key: String,
    pub seconds_played: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlaytimePerMonth {
    pub year_month: String,
    pub month: String,
    pub seconds_played: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeneralActivity {
    pub total_seconds_played: i64,
    pub start_of_playtime: NaiveDate,
    pub average_seconds_played: i64,
}

impl GeneralActivity {
    #[must_use]
    pub fn total_seconds_played_to_hours(&self) -> f64 {
        // Seconds below 2^53 convert losslessly; playtime is nowhere near
        // that, so the cast is safe.
        #[allow(clippy::cast_precision_loss)]
        let seconds = self.total_seconds_played as f64;
        seconds_to_hours(seconds)
    }

    #[must_use]
    pub fn average_seconds_played_to_hours(&self) -> f64 {
        #[allow(clippy::cast_precision_loss)]
        let seconds = self.average_seconds_played as f64;
        seconds_to_hours(seconds)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FavouriteRank1sTeammates {
    pub name: String,
    pub ranks_together: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AllTop10 {
    pub map: StatsMap,
    pub name: String,
    pub time: f64,
    pub rank: i32,
    pub team_rank: Option<i32>,
    pub team_time: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecentTop10 {
    pub rank_type: String,
    pub map: String,
    pub time: f64,
    pub rank: i32,
    pub timestamp: NaiveDateTime,
    pub server: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InfoSMap {
    pub map: StatsMap,
    pub finishes: Option<i32>,
    pub finishes_rank: Option<i32>,
    pub median_time: Option<f64>,
    pub total_playtime: Option<i64>,
    pub total_playtime_rank: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RankingSMap {
    pub rank: i32,
    pub timestamp: Option<NaiveDateTime>,
    pub name: String,
    pub time: f64,
    pub map: String,
    pub server: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeamRankingSMap {
    pub rank: i32,
    pub timestamp: Option<NaiveDateTime>,
    pub id: Vec<u8>,
    pub players: Vec<String>,
    pub time: f64,
    pub map: String,
    pub server: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimeCpsSMap {
    pub name: String,
    pub cp1: f64,
    pub cp2: f64,
    pub cp3: f64,
    pub cp4: f64,
    pub cp5: f64,
    pub cp6: f64,
    pub cp7: f64,
    pub cp8: f64,
    pub cp9: f64,
    pub cp10: f64,
    pub cp11: f64,
    pub cp12: f64,
    pub cp13: f64,
    pub cp14: f64,
    pub cp15: f64,
    pub cp16: f64,
    pub cp17: f64,
    pub cp18: f64,
    pub cp19: f64,
    pub cp20: f64,
    pub cp21: f64,
    pub cp22: f64,
    pub cp23: f64,
    pub cp24: f64,
    pub cp25: f64,
    pub time: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlaytimeSMap {
    pub rank: i64,
    pub name: String,
    pub seconds_played: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Map {
    pub info: InfoSMap,
    pub rankings: Vec<RankingSMap>,
    pub team_rankings: Vec<TeamRankingSMap>,
    pub time_cps: Vec<TimeCpsSMap>,
    pub playtime: Vec<PlaytimeSMap>,
}

impl Map {
    #[must_use]
    pub fn url(&self) -> String {
        format!(
            "https://{}/map/{}",
            DDSTATS_BASE_URL,
            encode(&self.info.map.map)
        )
    }

    #[must_use]
    pub fn url_with_name(map: &str) -> String {
        format!("https://{}/map/{}", DDSTATS_BASE_URL, encode(map))
    }

    #[must_use]
    pub fn api(map: &str) -> String {
        format!("https://{}/map/json?map={}", DDSTATS_BASE_URL, encode(map))
    }
}
