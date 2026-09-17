import asyncio
import tomllib
from datetime import date, datetime
from pathlib import Path

import pytest

import ddapi
from ddapi._convert import _from_dict

CARGO_TOML = Path(__file__).resolve().parent.parent.parent / "Cargo.toml"


def test_version_matches_cargo():
    with open(CARGO_TOML, "rb") as f:
        cargo = tomllib.load(f)
    assert ddapi.__version__ == cargo["package"]["version"]


def test_module_exports():
    for name in ("DDApi", "DDnetClient", "DDstatsClient", "DDError", "ddnet", "ddstats"):
        assert hasattr(ddapi, name)


def test_skins():
    async def main():
        api = ddapi.DDApi()
        skins = await api.skins()
        assert isinstance(skins, ddapi.ddnet.DDSkins)
        assert len(skins.skins) > 0
        assert isinstance(skins.skins[0], ddapi.ddnet.DDSkin)

    asyncio.run(main())


def test_player():
    async def main():
        api = ddapi.DDApi()
        player = await api.player("nameless tee")
        assert isinstance(player, ddapi.ddnet.Player)
        assert player.player == "nameless tee"
        assert isinstance(player.first_finish.timestamp, datetime)
        assert isinstance(player.activity[0].date, date)
        assert player.points.points is not None

    asyncio.run(main())


def test_player_not_found_raises_dderror():
    async def main():
        api = ddapi.DDApi()
        with pytest.raises(ddapi.DDError):
            await api.player("no such player 12345")

    asyncio.run(main())


def test_custom_master_invalid_index_raises():
    async def main():
        api = ddapi.DDApi()
        with pytest.raises(ddapi.DDError):
            await api.custom_master(99)

    asyncio.run(main())


def test_custom_master():
    async def main():
        api = ddapi.DDApi()
        master = await api.custom_master(1)
        assert isinstance(master, ddapi.ddnet.Master)
        assert len(master.servers) > 0

    asyncio.run(main())


def test_ddstats_profile():
    async def main():
        client = ddapi.DDstatsClient()
        profile = await client.profile("ByFox")
        assert isinstance(profile, ddapi.ddstats.Profile)
        assert profile.name == "ByFox"

    asyncio.run(main())


def test_ddstats_player():
    async def main():
        client = ddapi.DDstatsClient()
        player = await client.player("ByFox")
        assert isinstance(player, ddapi.ddstats.Player)
        assert isinstance(player.recent_finishes[0].timestamp, datetime)
        assert isinstance(player.points_graph[0].date, date)

    asyncio.run(main())


def test_ddnet_client():
    async def main():
        client = ddapi.DDnetClient()
        status = await client.status()
        assert isinstance(status, ddapi.ddnet.Status)
        assert len(status.servers) > 0

    asyncio.run(main())


def test_missing_data_is_none():
    last_finish = _from_dict(
        ddapi.ddnet.LastFinish,
        {"timestamp": 0, "map": "m", "time": 1.0, "country": "c", "type": None},
    )
    assert last_finish.type is None
    assert isinstance(last_finish.timestamp, datetime)


def test_datetime_timestamp_conversion():
    last_finish = _from_dict(
        ddapi.ddnet.LastFinish,
        {"timestamp": 0, "map": "m", "time": 1.0, "country": "c", "type": None},
    )
    assert last_finish.timestamp == datetime(1970, 1, 1, tzinfo=__import__("datetime").timezone.utc)