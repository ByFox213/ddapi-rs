use serde_derive::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    #[serde(rename = "points_graph")]
    pub points_graph: Vec<PointsGraph>,
    #[serde(rename = "recent_finishes")]
    pub recent_finishes: Vec<RecentFinish>,
    #[serde(rename = "favourite_teammates")]
    pub favourite_teammates: Option<Vec<FavouriteTeammate>>,
    pub profile: Profile,
    #[serde(rename = "is_mapper")]
    pub is_mapper: bool,
    pub finishes: Vec<Finish>,
    #[serde(rename = "unfinished_maps")]
    pub unfinished_maps: Option<Vec<UnfinishedMap>>,
    pub points: Points,
    #[serde(rename = "recent_activity")]
    pub recent_activity: Option<Vec<RecentActivity>>,
    #[serde(rename = "recent_player_info")]
    pub recent_player_info: Option<Vec<RecentPlayerInfo>>,
    #[serde(rename = "most_played_maps")]
    pub most_played_maps: Option<Vec<MostPlayedMap>>,
    #[serde(rename = "most_played_gametypes")]
    pub most_played_gametypes: Option<Vec<MostPlayed>>,
    #[serde(rename = "most_played_categories")]
    pub most_played_categories: Option<Vec<MostPlayed>>,
    #[serde(rename = "most_played_locations")]
    pub most_played_locations: Option<Vec<MostPlayed>>,
    #[serde(rename = "playtime_per_month")]
    pub playtime_per_month: Vec<PlaytimePerMonth>,
    #[serde(rename = "favourite_rank1s_teammates")]
    pub favourite_rank1s_teammates: Option<Vec<FavouriteRank1sTeammates>>,
    #[serde(rename = "all_top_10s")]
    pub all_top_10s: Option<Vec<AllTop10>>,
    #[serde(rename = "recent_top_10s")]
    pub recent_top_10s: Option<Vec<RecentTop10>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PointsGraph {
    pub date: String,
    pub points: i64,
    #[serde(rename = "rank_points")]
    pub rank_points: i64,
    #[serde(rename = "team_points")]
    pub team_points: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
pub struct StatsMap {
    pub map: String,
    pub server: String,
    pub points: u8,
    pub stars: u8,
    pub mapper: String,
    pub timestamp: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FavouriteTeammate {
    pub name: String,
    #[serde(rename = "ranks_together")]
    pub ranks_together: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub name: String,
    pub points: u64,
    pub clan: Option<String>,
    pub country: Option<u64>,
    #[serde(rename = "skin_name")]
    pub skin_name: Option<String>,
    #[serde(rename = "skin_color_body")]
    pub skin_color_body: Option<i64>,
    #[serde(rename = "skin_color_feet")]
    pub skin_color_feet: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Finish {
    pub map: StatsMap,
    pub name: String,
    pub time: f64,
    pub timestamp: String,
    pub server: String,
    pub rank: u64,
    #[serde(rename = "team_rank")]
    pub team_rank: Option<u64>,
    #[serde(rename = "seconds_played")]
    pub seconds_played: Option<u64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnfinishedMap {
    pub map: StatsMap,
    pub finishes: u64,
    #[serde(rename = "finishes_rank")]
    pub finishes_rank: Option<u64>,
    #[serde(rename = "median_time")]
    pub median_time: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Points {
    #[serde(rename = "weekly_points")]
    pub weekly_points: Option<PPoints>,
    #[serde(rename = "monthly_points")]
    pub monthly_points: Option<PPoints>,
    #[serde(rename = "yearly_points")]
    pub yearly_points: Option<PPoints>,
    pub points: StatsPoints,
    #[serde(rename = "rank_points")]
    pub rank_points: StatsPoints,
    #[serde(rename = "team_points")]
    pub team_points: StatsPoints,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PPoints {
    pub points: i64,
    pub rank: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatsPoints {
    #[serde(rename = "Moderate")]
    pub moderate: Option<Type>,
    #[serde(rename = "Insane")]
    pub insane: Option<Type>,
    #[serde(rename = "Oldschool")]
    pub oldschool: Option<Type>,
    #[serde(rename = "Fun")]
    pub fun: Option<Type>,
    #[serde(rename = "Race")]
    pub race: Option<Type>,
    #[serde(rename = "Total")]
    pub total: Option<Type>,
    #[serde(rename = "DDmaX.Easy")]
    pub ddmax_easy: Option<Type>,
    #[serde(rename = "Novice")]
    pub novice: Option<Type>,
    #[serde(rename = "Dummy")]
    pub dummy: Option<Type>,
    #[serde(rename = "DDmaX.Pro")]
    pub ddmax_pro: Option<Type>,
    #[serde(rename = "Brutal")]
    pub brutal: Option<Type>,
    #[serde(rename = "DDmaX.Nut")]
    pub ddmax_nut: Option<Type>,
    #[serde(rename = "Solo")]
    pub solo: Option<Type>,
    #[serde(rename = "DDmaX.Next")]
    pub ddmax_next: Option<Type>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Type {
    pub points: i64,
    pub rank: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecentActivity {
    pub name: String,
    pub date: String,
    #[serde(rename = "map_name")]
    pub map_name: String,
    pub map: Option<StatsMap>,
    #[serde(rename = "seconds_played")]
    pub seconds_played: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecentPlayerInfo {
    pub name: String,
    pub clan: String,
    pub country: u64,
    #[serde(rename = "skin_name")]
    pub skin_name: Option<String>,
    #[serde(rename = "skin_color_body")]
    pub skin_color_body: Option<u64>,
    #[serde(rename = "skin_color_feet")]
    pub skin_color_feet: Option<u64>,
    #[serde(rename = "last_seen")]
    pub last_seen: String,
    #[serde(rename = "seconds_played")]
    pub seconds_played: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MostPlayedMap {
    #[serde(rename = "map_name")]
    pub map_name: String,
    #[serde(rename = "seconds_played")]
    pub seconds_played: u64,
    pub map: Option<StatsMap>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MostPlayed {
    pub key: String,
    #[serde(rename = "seconds_played")]
    pub seconds_played: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaytimePerMonth {
    #[serde(rename = "year_month")]
    pub year_month: String,
    pub month: String,
    #[serde(rename = "seconds_played")]
    pub seconds_played: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FavouriteRank1sTeammates {
    pub name: String,
    pub ranks_together: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllTop10 {
    pub map: StatsMap,
    pub name: String,
    pub time: f64,
    pub rank: u64,
    #[serde(rename = "team_rank")]
    pub team_rank: Option<u64>,
    #[serde(rename = "team_time")]
    pub team_time: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecentTop10 {
    #[serde(rename = "rank_type")]
    pub rank_type: String,
    pub map: String,
    pub time: f64,
    pub rank: u64,
    pub timestamp: String,
    pub server: String,
}
