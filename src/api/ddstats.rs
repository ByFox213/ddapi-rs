use crate::api::{DDApi, DDstatsClient, HasApiCore};
use crate::error::Result;
use crate::scheme::ddstats::*;
use std::future::Future;

pub trait DDstats {
    fn player(&self, player: &str) -> impl Future<Output = Result<Player>> + Send;
    fn map(&self, map: &str) -> impl Future<Output = Result<Map>> + Send;
    fn maps(&self) -> impl Future<Output = Result<Vec<StatsMap>>> + Send;
    fn profile(&self, player: &str) -> impl Future<Output = Result<Profile>> + Send;
}

impl DDstats for DDApi {
    /// # Examples
    ///
    /// ```rust,ignore
    /// use ddapi_rs::prelude::*;
    /// use ddapi_rs::prelude::ddstats::*;
    ///
    /// let api = DDApi::new();
    /// let player: Player = api.player("Aoe").await?;
    /// println!("{}: {} | {}", player.profile.name, player.profile.points, player.profile.clan.unwrap_or(String::default()));
    /// ```
    async fn player(&self, player: &str) -> Result<Player> {
        self._generator(&Player::api(player)).await
    }

    /// # Examples
    ///
    /// ```rust,ignore
    /// use ddapi_rs::prelude::*;
    /// use ddapi_rs::prelude::ddstats::*;
    ///
    /// let api = DDApi::new();
    /// let map: Map = api.map("Fox").await?;
    /// println!("{}: {} | {}", map.info.map.map, map.info.map.stars, map.info.finishes);
    /// ```
    async fn map(&self, map: &str) -> Result<Map> {
        self._generator(&Map::api(map)).await
    }

    /// # Examples
    ///
    /// ```rust,ignore
    /// use ddapi_rs::prelude::*;
    /// use ddapi_rs::prelude::ddstats::*;
    ///
    /// let api = DDApi::new();
    /// let maps: Vec<StatsMap> = api.maps().await?;
    /// for map in &maps {
    ///     println!("{}: {} | {}", map.map, map.stars, map.points);
    /// }
    /// ```
    async fn maps(&self) -> Result<Vec<StatsMap>> {
        self._generator(&StatsMap::api()).await
    }

    /// # Examples
    ///
    /// ```rust,ignore
    /// use ddapi_rs::prelude::*;
    /// use ddapi_rs::prelude::ddstats::*;
    ///
    /// let api = DDApi::new();
    /// let player: Profile = api.profile("ByFox").await?;
    /// println!("{}: {}", player.name, player.clan.unwrap_or(String::default()));
    /// ```
    async fn profile(&self, player: &str) -> Result<Profile> {
        self._generator(&Profile::api(player)).await
    }
}

impl DDstats for DDstatsClient {
    async fn player(&self, player: &str) -> Result<Player> {
        self.core()._generator(&Player::api(player)).await
    }

    async fn map(&self, map: &str) -> Result<Map> {
        self.core()._generator(&Map::api(map)).await
    }

    async fn maps(&self) -> Result<Vec<StatsMap>> {
        self.core()._generator(&StatsMap::api()).await
    }

    async fn profile(&self, player: &str) -> Result<Profile> {
        self.core()._generator(&Profile::api(player)).await
    }
}
