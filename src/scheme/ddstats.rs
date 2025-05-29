use crate::util::encoding::{encode, slugify2};
use crate::util::time::seconds_to_hours;
use serde_derive::{Deserialize, Serialize};

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

impl Player {
    pub fn url(&self) -> String {
        format!("https://ddstats.tw/player/{}", encode(&self.profile.name))
    }

    pub fn url_with_name(player: &str) -> String {
        format!("https://ddstats.tw/player/{}", encode(player))
    }

    pub fn api(player: &str) -> String {
        format!("https://ddstats.tw/player/json?player={}", encode(player))
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PointsGraph {
    pub date: String,
    pub points: i64,
    pub rank_points: i64,
    pub team_points: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecentFinish {
    pub map: StatsMap,
    pub name: String,
    pub time: f64,
    pub timestamp: String,
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
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatsMap {
    pub map: String,
    pub server: String,
    pub points: u8,
    pub stars: u8,
    pub mapper: String,
    pub timestamp: Option<String>,
}

impl StatsMap {
    pub fn url(&self) -> String {
        format!("https://ddstats.tw/map/{}", encode(&slugify2(&self.map)))
    }

    pub fn url_with_name(map: &str) -> String {
        format!("https://ddstats.tw/map/{}", encode(&slugify2(map)))
    }

    pub fn api() -> String {
        "https://ddstats.tw/maps/json".to_string()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FavouriteTeammate {
    pub name: String,
    pub ranks_together: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    pub points: u64,
    pub clan: Option<String>,
    pub country: Option<u64>,
    pub skin_name: Option<String>,
    pub skin_color_body: Option<i64>,
    pub skin_color_feet: Option<i64>,
}

impl Profile {
    pub fn url(&self) -> String {
        format!("https://ddstats.tw/player/{}", encode(&self.name))
    }

    pub fn url_with_name(player: &str) -> String {
        format!("https://ddstats.tw/player/{}", encode(player))
    }

    pub fn api(player: &str) -> String {
        format!("https://ddstats.tw/profile/json?player={}", encode(player))
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Finish {
    pub map: StatsMap,
    pub name: String,
    pub time: f64,
    pub timestamp: String,
    pub server: String,
    pub rank: u64,
    pub team_rank: Option<u64>,
    pub seconds_played: Option<u64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnfinishedMap {
    pub map: StatsMap,
    pub finishes: u64,
    pub finishes_rank: Option<u64>,
    pub median_time: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Points {
    pub weekly_points: Option<PPoints>,
    pub monthly_points: Option<PPoints>,
    pub yearly_points: Option<PPoints>,
    pub points: StatsPoints,
    pub rank_points: StatsPoints,
    pub team_points: StatsPoints,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PPoints {
    pub points: i64,
    pub rank: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StatsPoints {
    pub moderate: Option<Type>,
    pub insane: Option<Type>,
    pub oldschool: Option<Type>,
    pub fun: Option<Type>,
    pub race: Option<Type>,
    pub total: Option<Type>,
    #[serde(rename = "DDmaX.Easy")]
    pub ddmax_easy: Option<Type>,
    pub novice: Option<Type>,
    pub dummy: Option<Type>,
    #[serde(rename = "DDmaX.Pro")]
    pub ddmax_pro: Option<Type>,
    pub brutal: Option<Type>,
    #[serde(rename = "DDmaX.Nut")]
    pub ddmax_nut: Option<Type>,
    pub solo: Option<Type>,
    #[serde(rename = "DDmaX.Next")]
    pub ddmax_next: Option<Type>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Type {
    pub points: i64,
    pub rank: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecentActivity {
    pub name: String,
    pub date: String,
    pub map_name: String,
    pub map: Option<StatsMap>,
    pub seconds_played: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecentPlayerInfo {
    pub name: String,
    pub clan: String,
    pub country: i16,
    pub skin_name: Option<String>,
    pub skin_color_body: Option<u64>,
    pub skin_color_feet: Option<u64>,
    pub last_seen: String,
    pub seconds_played: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MostPlayedMap {
    pub map_name: String,
    pub seconds_played: u64,
    pub map: Option<StatsMap>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MostPlayed {
    pub key: String,
    pub seconds_played: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlaytimePerMonth {
    pub year_month: String,
    pub month: String,
    pub seconds_played: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeneralActivity {
    pub total_seconds_played: u64,
    pub start_of_playtime: String,
    pub average_seconds_played: u64,
}

impl GeneralActivity {
    pub fn total_seconds_played_to_hours(&self) -> f64 {
        seconds_to_hours(self.total_seconds_played)
    }

    pub fn average_seconds_played_to_hours(&self) -> f64 {
        seconds_to_hours(self.average_seconds_played)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FavouriteRank1sTeammates {
    pub name: String,
    pub ranks_together: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AllTop10 {
    pub map: StatsMap,
    pub name: String,
    pub time: f64,
    pub rank: u64,
    pub team_rank: Option<u64>,
    pub team_time: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecentTop10 {
    pub rank_type: String,
    pub map: String,
    pub time: f64,
    pub rank: u64,
    pub timestamp: String,
    pub server: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InfoSMap {
    pub map: StatsMap,
    pub finishes: u64,
    pub finishes_rank: u64,
    pub median_time: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RankingSMap {
    pub rank: u64,
    pub timestamp: Option<String>,
    pub name: String,
    pub time: f64,
    pub map: String,
    pub server: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeamRankingSMap {
    pub rank: u64,
    pub timestamp: Option<String>,
    pub id: Vec<u64>,
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
    pub name: String,
    pub seconds_played: u64,
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
    pub fn url(&self) -> String {
        format!("https://ddstats.tw/map/{}", encode(&self.info.map.map))
    }

    pub fn url_with_name(map: &str) -> String {
        format!("https://ddstats.tw/map/{}", encode(map))
    }
    pub fn api(map: &str) -> String {
        format!("https://ddstats.tw/map/json?map={}", encode(map))
    }
}
