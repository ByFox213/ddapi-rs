use std::future::Future;
use crate::error_ddapi::ApiError;
use crate::scheme::ddstats::{Map, Maps, Player, Profile};
use crate::DDApi;

#[allow(dead_code)]
pub trait DDstats {
    fn s_player(
        &self,
        player: &str,
    ) -> impl Future<Output = Result<Player, ApiError>> + Send;
    fn s_map(&self, map: &str) -> impl Future<Output = Result<Map, ApiError>> + Send;
    fn s_maps(&self) -> impl Future<Output = Result<Maps, ApiError>> + Send;
    fn s_profile(
        &self,
        player: &str,
    ) -> impl Future<Output = Result<Profile, ApiError>> + Send;
}

impl DDstats for DDApi {
    async fn s_player(&self, player: &str) -> Result<Player, ApiError> {
        self._generator(&format!(
            "https://ddstats.tw/player/json?player={}",
            self.encode(player).await
        ))
        .await
    }

    async fn s_map(&self, map: &str) -> Result<Map, ApiError> {
        self._generator(&format!(
            "https://ddstats.tw/map/json?map={}",
            self.encode(map).await
        ))
        .await
    }

    async fn s_maps(&self) -> Result<Maps, ApiError> {
        self._generator("https://ddstats.tw/maps/json")
        .await
    }

    async fn s_profile(&self, player: &str) -> Result<Profile, ApiError> {
        self._generator(&format!(
            "https://ddstats.tw/profile/json?player={}",
            self.encode(player).await
        ))
        .await
    }
}
