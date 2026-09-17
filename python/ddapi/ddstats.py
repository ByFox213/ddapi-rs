"""DDStats API models."""

from __future__ import annotations

from dataclasses import dataclass
from datetime import date, datetime
from typing import Dict, List, Optional


@dataclass
class CompletionProgress:
    category: str
    maps_finished: int
    maps_total: int


@dataclass
class PointsGraph:
    date: date
    points: int
    rank_points: int
    team_points: int


@dataclass
class StatsMap:
    map: str
    server: str
    points: int
    stars: int
    mapper: str
    timestamp: Optional[datetime]


@dataclass
class RecentFinish:
    map: StatsMap
    name: str
    time: float
    timestamp: datetime
    server: str
    cp1: float
    cp2: float
    cp3: float
    cp4: float
    cp5: float
    cp6: float
    cp7: float
    cp8: float
    cp9: float
    cp10: float
    cp11: float
    cp12: float
    cp13: float
    cp14: float
    cp15: float
    cp16: float
    cp17: float
    cp18: float
    cp19: float
    cp20: float
    cp21: float
    cp22: float
    cp23: float
    cp24: float
    cp25: float
    team_rank: Optional["TeamRankingSMap"]
    rank: Optional["RankingSMap"]


@dataclass
class FavouriteTeammate:
    name: str
    ranks_together: int


@dataclass
class Profile:
    name: str
    points: int
    clan: Optional[str]
    country: Optional[int]
    skin_name: Optional[str]
    skin_color_body: Optional[int]
    skin_color_feet: Optional[int]
    most_played_location: Optional[str]


@dataclass
class Finish:
    map: StatsMap
    name: str
    time: float
    timestamp: datetime
    server: str
    rank: int
    team_rank: Optional[int]
    seconds_played: Optional[int]


@dataclass
class UnfinishedMap:
    map: StatsMap
    finishes: Optional[int]
    finishes_rank: Optional[int]
    median_time: Optional[float]


@dataclass
class LeaderboardRank:
    points: int
    rank: int


@dataclass
class Points:
    weekly_points: Optional[LeaderboardRank]
    monthly_points: Optional[LeaderboardRank]
    yearly_points: Optional[LeaderboardRank]
    points: Dict[str, Optional[LeaderboardRank]]
    rank_points: Dict[str, Optional[LeaderboardRank]]
    team_points: Dict[str, Optional[LeaderboardRank]]


@dataclass
class RecentActivity:
    name: str
    date: date
    map_name: str
    map: Optional[StatsMap]
    seconds_played: int


@dataclass
class RecentPlayerInfo:
    name: str
    clan: str
    country: int
    skin_name: str
    skin_color_body: Optional[int]
    skin_color_feet: Optional[int]
    last_seen: date
    seconds_played: int


@dataclass
class MostPlayedMap:
    map_name: str
    seconds_played: int
    map: Optional[StatsMap]


@dataclass
class MostPlayedMapsGametype:
    map_name: str
    seconds_played: int
    gametype: str


@dataclass
class Teero:
    most_played_maps_gametype: List[MostPlayedMapsGametype]


@dataclass
class MostPlayed:
    key: str
    seconds_played: int


@dataclass
class PlaytimePerMonth:
    year_month: str
    month: str
    seconds_played: int


@dataclass
class GeneralActivity:
    total_seconds_played: int
    start_of_playtime: date
    average_seconds_played: int


@dataclass
class FavouriteRank1sTeammates:
    name: str
    ranks_together: int


@dataclass
class AllTop10:
    map: StatsMap
    name: str
    time: float
    rank: int
    team_rank: Optional[int]
    team_time: Optional[float]


@dataclass
class RecentTop10:
    rank_type: str
    map: str
    time: float
    rank: int
    timestamp: datetime
    server: str


@dataclass
class InfoSMap:
    map: StatsMap
    finishes: Optional[int]
    finishes_rank: Optional[int]
    median_time: Optional[float]
    total_playtime: Optional[int]
    total_playtime_rank: Optional[int]


@dataclass
class RankingSMap:
    rank: int
    timestamp: Optional[datetime]
    name: str
    time: float
    map: str
    server: str


@dataclass
class TeamRankingSMap:
    rank: int
    timestamp: Optional[datetime]
    id: List[int]
    players: List[str]
    time: float
    map: str
    server: str


@dataclass
class TimeCpsSMap:
    name: str
    cp1: float
    cp2: float
    cp3: float
    cp4: float
    cp5: float
    cp6: float
    cp7: float
    cp8: float
    cp9: float
    cp10: float
    cp11: float
    cp12: float
    cp13: float
    cp14: float
    cp15: float
    cp16: float
    cp17: float
    cp18: float
    cp19: float
    cp20: float
    cp21: float
    cp22: float
    cp23: float
    cp24: float
    cp25: float
    time: float


@dataclass
class PlaytimeSMap:
    rank: int
    name: str
    seconds_played: int


@dataclass
class Map:
    info: InfoSMap
    rankings: List[RankingSMap]
    team_rankings: List[TeamRankingSMap]
    time_cps: List[TimeCpsSMap]
    playtime: List[PlaytimeSMap]


@dataclass
class Player:
    points_graph: List[PointsGraph]
    recent_finishes: List[RecentFinish]
    favourite_teammates: List[FavouriteTeammate]
    profile: Profile
    is_mapper: bool
    finishes: List[Finish]
    unfinished_maps: List[UnfinishedMap]
    points: Points
    completion_progress: List[CompletionProgress]
    recent_activity: List[RecentActivity]
    recent_player_info: List[RecentPlayerInfo]
    most_played_maps: List[MostPlayedMap]
    most_played_gametypes: List[MostPlayed]
    most_played_categories: List[MostPlayed]
    most_played_locations: List[MostPlayed]
    playtime_per_month: List[PlaytimePerMonth]
    general_activity: Optional[GeneralActivity]
    favourite_rank1s_teammates: List[FavouriteRank1sTeammates]
    all_top_10s: List[AllTop10]
    recent_top_10s: List[RecentTop10]