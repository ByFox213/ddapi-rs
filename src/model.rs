use std::collections::HashMap;
use serde::{
    Deserialize as SerdeDeserialize,
    Deserializer
};
use serde_derive::Deserialize;
use serde_derive::Serialize;

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
    pub fn count_clients(&self) -> u32 {
        let mut total = 0;
        for server in self.servers.iter() {
            total += Server::count_client(server);
        }
        total as u32
    }

    #[allow(dead_code)]
    pub fn get_clans(&self, rm: Option<Vec<&str>>) -> Vec<(String, usize)> {
        let remove_list: Vec<&str> = rm.unwrap_or(Vec::from(["DD-Persian", "/vDQMHSss8W"]));

        if self.servers.is_empty() {
            return vec![];
        }

        let mut dat: HashMap<String, usize> = HashMap::new();
        for server in &self.servers {
            if let Some(clients) = &server.info.clients {
                for client in clients {
                    if !client.clan.is_empty() {
                        *dat.entry(client.clan.clone()).or_insert(0) += 1;
                    }
                }
            }
        }
        for i in remove_list {
            dat.remove(i);
        }
        let mut sorted_dat: Vec<_> = dat.into_iter().collect();
        sorted_dat.sort_by_key(|&(_, count)| count);
        sorted_dat.reverse();
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
        self.info.clients.as_ref().map_or(0, |clients| clients.len())
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
    pub color_feet: Option<i64>
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
    pub rank: Option<u64>
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rank {
    pub points: Option<u64>,
    pub rank: Option<u64>
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FavoriteServer {
    pub server: String
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FirstFinish {
    #[serde(deserialize_with = "deserialize_timestamp")]
    pub timestamp: u64,
    pub map: String,
    pub time: f64
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
    pub points: StatsPoints,
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
    pub clan: String,
    pub country: u64,
    #[serde(rename = "skin_name")]
    pub skin_name: String,
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
pub struct StatsPoints {
    #[serde(rename = "weekly_points")]
    pub weekly_points: Option<PPoints>,
    #[serde(rename = "monthly_points")]
    pub monthly_points: Option<PPoints>,
    #[serde(rename = "yearly_points")]
    pub yearly_points: Option<PPoints>,
    pub points: StatsPoints2,
    #[serde(rename = "rank_points")]
    pub rank_points: StatsPoints2,
    #[serde(rename = "team_points")]
    pub team_points: StatsPoints2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PPoints {
    pub points: i64,
    pub rank: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatsPoints2 {
    #[serde(rename = "Moderate")]
    pub moderate: Option<TType>,
    #[serde(rename = "Insane")]
    pub insane: Option<TType>,
    #[serde(rename = "Oldschool")]
    pub oldschool: Option<TType>,
    #[serde(rename = "Fun")]
    pub fun: Option<TType>,
    #[serde(rename = "Race")]
    pub race: Option<TType>,
    #[serde(rename = "Total")]
    pub total: Option<TType>,
    #[serde(rename = "DDmaX.Easy")]
    pub ddmax_easy: Option<TType>,
    #[serde(rename = "Novice")]
    pub novice: Option<TType>,
    #[serde(rename = "Dummy")]
    pub dummy: Option<TType>,
    #[serde(rename = "DDmaX.Pro")]
    pub ddmax_pro: Option<TType>,
    #[serde(rename = "Brutal")]
    pub brutal: Option<TType>,
    #[serde(rename = "DDmaX.Nut")]
    pub ddmax_nut: Option<TType>,
    #[serde(rename = "Solo")]
    pub solo: Option<TType>,
    #[serde(rename = "DDmaX.Next")]
    pub ddmax_next: Option<TType>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TType {
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
    pub ranks_together: u64
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