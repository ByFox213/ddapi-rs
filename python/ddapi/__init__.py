"""Async Python bindings for the DDNet and DDStats public APIs.

Usage::

    import asyncio

    import ddapi

    async def main() -> None:
        api = ddapi.DDApi()
        player = await api.player("nameless tee")
        print(player.player, player.points.points)

    asyncio.run(main())
"""

from typing import List

from . import ddnet, ddstats
from ._convert import _from_dict
from ._ddapi import DDError
from ._ddapi import DDApi as _DDApi
from ._ddapi import DDnetClient as _DDnetClient
from ._ddapi import DDstatsClient as _DDstatsClient
from ._ddapi import __version__

__all__ = [
    "DDApi",
    "DDnetClient",
    "DDstatsClient",
    "DDError",
    "ddnet",
    "ddstats",
    "__version__",
]


class _DDnetWrapper:
    """Shared DDNet API surface used by :class:`DDApi` and :class:`DDnetClient`."""

    def __init__(self, api: object) -> None:
        self._api = api

    def set_cache(self, capacity: int, ttl_seconds: int) -> None:
        """Enable an in-memory response cache (capacity entries, TTL in seconds)."""
        self._api.set_cache(capacity, ttl_seconds)

    async def master(self) -> ddnet.Master:
        return _from_dict(ddnet.Master, await self._api.master())

    async def custom_master(self, master: int) -> ddnet.Master:
        """Fetch servers from a specific master server (index 1-4)."""
        return _from_dict(ddnet.Master, await self._api.custom_master(master))

    async def skins(self) -> ddnet.DDSkins:
        return _from_dict(ddnet.DDSkins, await self._api.skins())

    async def player(self, player: str) -> ddnet.Player:
        return _from_dict(ddnet.Player, await self._api.player(player))

    async def query(self, player: str) -> List[ddnet.Query]:
        return _from_dict(List[ddnet.Query], await self._api.query(player))

    async def query_map(self, map: str) -> List[ddnet.QueryMap]:
        return _from_dict(List[ddnet.QueryMap], await self._api.query_map(map))

    async def query_mapper(self, player: str) -> List[ddnet.QueryMapper]:
        return _from_dict(List[ddnet.QueryMapper], await self._api.query_mapper(player))

    async def map(self, map: str) -> ddnet.Map:
        return _from_dict(ddnet.Map, await self._api.map(map))

    async def releases_map(self) -> List[ddnet.ReleasesMaps]:
        return _from_dict(List[ddnet.ReleasesMaps], await self._api.releases_map())

    async def status(self) -> ddnet.Status:
        return _from_dict(ddnet.Status, await self._api.status())

    async def latest_finish(self) -> List[ddnet.LatestFinishes]:
        return _from_dict(List[ddnet.LatestFinishes], await self._api.latest_finish())

    async def latest_finish_with_latest(self, latest: int) -> List[ddnet.LatestFinishes]:
        return _from_dict(
            List[ddnet.LatestFinishes], await self._api.latest_finish_with_latest(latest)
        )


class DDApi(_DDnetWrapper):
    """Combined client. DDNet methods take precedence over DDStats."""

    def __init__(self) -> None:
        super().__init__(_DDApi())


class DDnetClient(_DDnetWrapper):
    """DDNet API client."""

    def __init__(self) -> None:
        super().__init__(_DDnetClient())


class DDstatsClient:
    """DDStats API client."""

    def __init__(self) -> None:
        self._api = _DDstatsClient()

    def set_cache(self, capacity: int, ttl_seconds: int) -> None:
        """Enable an in-memory response cache (capacity entries, TTL in seconds)."""
        self._api.set_cache(capacity, ttl_seconds)

    async def player(self, player: str) -> ddstats.Player:
        return _from_dict(ddstats.Player, await self._api.player(player))

    async def map(self, map: str) -> ddstats.Map:
        return _from_dict(ddstats.Map, await self._api.map(map))

    async def maps(self) -> List[ddstats.StatsMap]:
        return _from_dict(List[ddstats.StatsMap], await self._api.maps())

    async def profile(self, player: str) -> ddstats.Profile:
        return _from_dict(ddstats.Profile, await self._api.profile(player))

    async def teero(self, player: str) -> ddstats.Teero:
        return _from_dict(ddstats.Teero, await self._api.teero(player))