use crate::api::DDApi;
use crate::scheme::ddstats::*;
use anyhow::Result;
use std::future::Future;

pub trait DDstats {
    fn s_player(&self, player: &str) -> impl Future<Output = Result<Player>> + Send;
    fn s_map(&self, map: &str) -> impl Future<Output = Result<Map>> + Send;
    fn s_maps(&self) -> impl Future<Output = Result<Vec<StatsMap>>> + Send;
    fn s_profile(&self, player: &str) -> impl Future<Output = Result<Profile>> + Send;
}

impl DDstats for DDApi {
    async fn s_player(&self, player: &str) -> Result<Player> {
        self._generator(&Player::api(player)).await
    }

    async fn s_map(&self, map: &str) -> Result<Map> {
        self._generator(&Map::api(map)).await
    }

    async fn s_maps(&self) -> Result<Vec<StatsMap>> {
        self._generator(&StatsMap::api()).await
    }

    async fn s_profile(&self, player: &str) -> Result<Profile> {
        self._generator(&Profile::api(player)).await
    }
}
