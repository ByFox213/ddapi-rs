use crate::api::DDApi;
use crate::errors::ApiError;
use crate::scheme::ddstats::*;
use std::future::Future;

#[allow(dead_code)]
pub trait DDstats {
    fn s_player(&self, player: &str) -> impl Future<Output = Result<Player, ApiError>> + Send;
    fn s_map(&self, map: &str) -> impl Future<Output = Result<Map, ApiError>> + Send;
    fn s_maps(&self) -> impl Future<Output = Result<Vec<StatsMap>, ApiError>> + Send;
    fn s_profile(&self, player: &str) -> impl Future<Output = Result<Profile, ApiError>> + Send;
}

impl DDstats for DDApi {
    async fn s_player(&self, player: &str) -> Result<Player, ApiError> {
        self._generator(&Player::api(player)).await
    }

    async fn s_map(&self, map: &str) -> Result<Map, ApiError> {
        self._generator(&Map::api(map)).await
    }

    async fn s_maps(&self) -> Result<Vec<StatsMap>, ApiError> {
        self._generator(&StatsMap::api()).await
    }

    async fn s_profile(&self, player: &str) -> Result<Profile, ApiError> {
        self._generator(&Profile::api(player)).await
    }
}
