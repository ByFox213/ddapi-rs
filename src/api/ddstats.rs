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
    /// # Examples
    ///
    /// ```rust,ignore
    /// use ddapi_rs::prelude::*;
    /// use ddapi_rs::prelude::ddstats::*;
    ///
    /// let api = DDApi::new();
    /// let player: Player = api.s_player("Aoe").await?;
    /// println!("{}: {} | {}", player.profile.name, player.profile.points, player.profile.clan.unwrap_or(String::default()));
    /// ```
    async fn s_player(&self, player: &str) -> Result<Player> {
        self._generator(&Player::api(player)).await
    }

    /// # Examples
    ///
    /// ```rust,ignore
    /// use ddapi_rs::prelude::*;
    /// use ddapi_rs::prelude::ddstats::*;
    ///
    /// let api = DDApi::new();
    /// let map: Map = api.s_map("Fox").await?;
    /// println!("{}: {} | {}", map.info.map.map, map.info.map.stars, map.info.finishes);
    /// ```
    async fn s_map(&self, map: &str) -> Result<Map> {
        self._generator(&Map::api(map)).await
    }

    /// # Examples
    ///
    /// ```rust,ignore
    /// use ddapi_rs::prelude::*;
    /// use ddapi_rs::prelude::ddstats::*;
    ///
    /// let api = DDApi::new();
    /// let maps: Vec<StatsMap> = api.s_maps().await?;
    /// for map in &maps {
    ///     println!("{}: {} | {}", map.map, map.stars, map.points);
    /// }
    /// ```
    async fn s_maps(&self) -> Result<Vec<StatsMap>> {
        self._generator(&StatsMap::api()).await
    }

    /// # Examples
    ///
    /// ```rust,ignore
    /// use ddapi_rs::prelude::*;
    /// use ddapi_rs::prelude::ddstats::*;
    ///
    /// let api = DDApi::new();
    /// let player: Profile = api.s_profile("ByFox").await?;
    /// println!("{}: {}", player.name, player.clan.unwrap_or(String::default()));
    /// ```
    async fn s_profile(&self, player: &str) -> Result<Profile> {
        self._generator(&Profile::api(player)).await
    }
}
