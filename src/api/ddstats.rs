use crate::api::{DDApi, DDstatsClient, HasApiCore};
use crate::error::Result;
use crate::scheme::ddstats::{Map, Player, Profile, StatsMap};
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
        self.generator(&Player::api(player)).await
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
        self.generator(&Map::api(map)).await
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
        self.generator(&StatsMap::api()).await
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
        self.generator(&Profile::api(player)).await
    }
}

impl DDstats for DDstatsClient {
    async fn player(&self, player: &str) -> Result<Player> {
        self.core().generator(&Player::api(player)).await
    }

    async fn map(&self, map: &str) -> Result<Map> {
        self.core().generator(&Map::api(map)).await
    }

    async fn maps(&self) -> Result<Vec<StatsMap>> {
        self.core().generator(&StatsMap::api()).await
    }

    async fn profile(&self, player: &str) -> Result<Profile> {
        self.core().generator(&Profile::api(player)).await
    }
}
