"""DDNet API models."""

from __future__ import annotations

from dataclasses import dataclass, field
from datetime import date, datetime
from typing import Dict, List, Optional


@dataclass
class Icon:
    sha256: str
    url: str


@dataclass
class Community:
    id: str
    name: str
    has_finishes: bool
    icon: Icon
    contact_urls: List[str]


@dataclass
class ClanCount:
    name: str
    count: int


@dataclass
class Master:
    communities: List[Community]
    servers: List["Server"]


@dataclass
class Server:
    addresses: List[str]
    community: Optional[str]
    location: str
    info: "Info"


@dataclass
class Info:
    max_clients: int
    max_players: int
    passworded: bool
    gametype: str = field(metadata={"json": "game_type"})
    name: str
    map: "IMap"
    version: str
    clients: List["Client"]
    requires_login: bool
    client_score_kind: Optional[str]
    country: Optional[str]
    flags: Optional[List[str]]
    flag: Optional[int]
    identity_key: Optional[str]


@dataclass
class IMap:
    name: str
    sha256: Optional[str]
    size: Optional[int]
    url: Optional[str]
    tw_crc: Optional[str]


@dataclass
class Client:
    name: str
    clan: str
    country: int
    score: int
    is_player: bool
    skin: Optional["Skin"]
    afk: bool
    team: int


@dataclass
class Skin:
    name: Optional[str]
    color_body: Optional[int]
    color_feet: Optional[int]
    body: Optional["SkinPart"]
    marking: Optional["SkinPart"]
    decoration: Optional["SkinPart"]
    eyes: Optional["SkinPart"]
    feet: Optional["SkinPart"]
    hands: Optional["SkinPart"]


@dataclass
class SkinPart:
    name: str
    color: Optional[int]


@dataclass
class DDSkinHD:
    uhd: bool


@dataclass
class DDSkin:
    name: str
    type: str
    hd: DDSkinHD
    creator: str
    license: str
    bodypart: str
    gameversion: str
    date: date
    skinpack: str
    imgtype: str


@dataclass
class DDSkins:
    skins: List[DDSkin]
    version: str


@dataclass
class Player:
    player: str
    points: "Points"
    team_rank: Optional["Rank"]
    rank: Optional["Rank"]
    points_last_year: Optional["Rank"]
    points_last_month: Optional["Rank"]
    points_last_week: Optional["Rank"]
    favorite_server: "FavoriteServer"
    first_finish: "FirstFinish"
    last_finishes: List["LastFinish"]
    favorite_partners: List["FavoritePartner"]
    deleted_ranks: List["DeletedRank"]
    types: "Types"
    activity: List["Activity"]
    hours_played_past_365_days: int


@dataclass
class Points:
    total: int
    points: Optional[int]
    rank: Optional[int]


@dataclass
class Rank:
    points: Optional[int]
    rank: Optional[int]


@dataclass
class FavoriteServer:
    server: str


@dataclass
class FirstFinish:
    timestamp: datetime
    map: str
    time: float


@dataclass
class LastFinish:
    timestamp: datetime
    map: str
    time: float
    country: str
    type: Optional[str]


@dataclass
class FavoritePartner:
    name: str
    finishes: int


@dataclass
class DeletedRank:
    map: str
    time: float
    timestamp: datetime
    country: str
    deleted_at: datetime


@dataclass
class Types:
    novice: "Type" = field(metadata={"json": "Novice"})
    moderate: "Type" = field(metadata={"json": "Moderate"})
    brutal: "Type" = field(metadata={"json": "Brutal"})
    insane: "Type" = field(metadata={"json": "Insane"})
    dummy: "Type" = field(metadata={"json": "Dummy"})
    ddmax_easy: "Type" = field(metadata={"json": "DDmaX.Easy"})
    ddmax_next: "Type" = field(metadata={"json": "DDmaX.Next"})
    ddmax_pro: "Type" = field(metadata={"json": "DDmaX.Pro"})
    ddmax_nut: "Type" = field(metadata={"json": "DDmaX.Nut"})
    oldschool: "Type" = field(metadata={"json": "Oldschool"})
    solo: "Type" = field(metadata={"json": "Solo"})
    race: "Type" = field(metadata={"json": "Race"})
    fun: "Type" = field(metadata={"json": "Fun"})
    event: "Type" = field(metadata={"json": "Event"})


@dataclass
class Type:
    points: Points
    team_rank: Optional[Rank]
    rank: Optional[Rank]
    maps: Dict[str, "DDMap"]


@dataclass
class DDMap:
    points: int
    total_finishes: int
    finishes: int
    team_rank: Optional[int]
    rank: Optional[int]
    time: Optional[float]
    first_finish: Optional[float]


@dataclass
class Activity:
    date: date
    hours_played: int


@dataclass
class Map:
    name: str
    website: str
    thumbnail: str
    web_preview: str
    type: str
    points: int
    difficulty: int
    mapper: str
    release: Optional[float]
    average_time: float
    first_finish: float
    last_finish: float
    finishes: int
    finishers: int
    biggest_team: int
    width: int
    height: int
    tiles: List[str]
    team_ranks: List["DTeamRank"]
    ranks: List["DRank"]
    max_finishes: List["MaxFinish"]


@dataclass
class DTeamRank:
    rank: int
    players: List[str]
    time: float
    timestamp: datetime
    country: str


@dataclass
class DRank:
    rank: int
    player: str
    time: float
    timestamp: datetime
    country: str


@dataclass
class MaxFinish:
    rank: int
    player: str
    num: int
    time: float
    min_timestamp: datetime
    max_timestamp: datetime


@dataclass
class Query:
    points: int
    name: str


@dataclass
class QueryMap:
    name: str
    type: str
    mapper: str


@dataclass
class QueryMapper:
    mapper: str
    num_maps: int


@dataclass
class ReleasesMaps:
    name: str
    website: str
    thumbnail: str
    web_preview: str
    type: str
    points: int
    difficulty: int
    mapper: str
    release: str
    width: Optional[int]
    height: Optional[int]
    tiles: List[str]


@dataclass
class StatusData:
    name: str
    type: str
    host: str
    location: str
    online4: bool
    online6: bool
    uptime: str
    load: float
    network_rx: int
    network_tx: int
    packets_rx: int
    packets_tx: int
    cpu: int
    memory_total: int
    memory_used: int
    swap_total: int
    swap_used: int
    hdd_total: int
    hdd_used: int


@dataclass
class Status:
    servers: List[StatusData]
    updated: str


@dataclass
class LatestFinishes:
    timestamp: datetime
    map: str
    name: str
    time: float
    server: str