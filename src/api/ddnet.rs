use crate::api::DDApi;
use crate::scheme::ddnet::prelude::*;
use anyhow::Result;
use std::future::Future;

#[allow(dead_code)]
pub trait DDnetApi {
    fn master(&self) -> impl Future<Output = Result<Master>> + Send;
    fn skins(&self) -> impl Future<Output = Result<DDSkins>> + Send;
    fn custom_master(&self, master: MasterServer) -> impl Future<Output = Result<Master>> + Send;
    fn player(&self, player: &str) -> impl Future<Output = Result<Player>> + Send;
    fn query(&self, player: &str) -> impl Future<Output = Result<Vec<Query>>> + Send;
    fn query_map(&self, player: &str) -> impl Future<Output = Result<Vec<QueryMap>>> + Send;
    fn query_mapper(&self, player: &str) -> impl Future<Output = Result<Vec<QueryMapper>>> + Send;
    fn map(&self, map: &str) -> impl Future<Output = Result<Map>> + Send;
    fn releases_map(&self) -> impl Future<Output = Result<Vec<ReleasesMaps>>> + Send;
    fn status(&self) -> impl Future<Output = Result<Status>> + Send;
    fn latest_finish(&self) -> impl Future<Output = Result<Vec<LatestFinishes>>> + Send;
    fn latest_finish_with_latest(
        &self,
        latest: usize,
    ) -> impl Future<Output = Result<Vec<LatestFinishes>>> + Send;
}

impl DDnetApi for DDApi {
    /// Fetches server list from the default master server
    ///
    /// Returns a list of game servers from the primary master server (master1.ddnet.org).
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use ddapi_rs::prelude::*;
    ///
    /// let api = DDApi::new();
    /// let master = api.master().await?;
    /// println!("Found {} servers", master.servers.len());
    /// ```
    async fn master(&self) -> Result<Master> {
        self.custom_master(MasterServer::One).await
    }

    /// # Examples
    ///
    /// ```rust,ignore
    /// use ddapi_rs::prelude::*;
    /// use ddapi_rs::prelude::ddnet::*;
    ///
    /// let api = DDApi::new();
    /// let skins: DDSkins = api.skins().await?;
    /// println!("Found {} available skins", skins.skins.len());
    /// for skin in skins.skins {
    ///     println!("Skin: {} by {}", skin.name, skin.creator);
    /// }
    /// ```
    async fn skins(&self) -> Result<DDSkins> {
        self._generator(&DDSkins::api()).await
    }

    /// Fetches server list from a specific master server
    ///
    /// Allows selecting which master server to query. DDNet has multiple
    /// master servers for redundancy and load distribution.
    ///
    /// # Arguments
    ///
    /// * `master` - The master server to query (`MasterServer::One`, `MasterServer::Two`, etc.)
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use ddapi_rs::prelude::*;
    /// use ddapi_rs::prelude::ddnet::*;
    ///
    /// let api = DDApi::new();
    /// // Use secondary master server as fallback
    /// let master = api.custom_master(MasterServer::Two).await?;
    /// ```
    async fn custom_master(&self, master: MasterServer) -> Result<Master> {
        self._generator_no_cache(&Master::api(master)).await
    }

    /// # Examples
    ///
    /// ```rust,ignore
    /// use ddapi_rs::prelude::*;
    /// use ddapi_rs::prelude::ddnet::*;
    ///
    /// let api = DDApi::new();
    /// let player: Player = api.player("nameless tee").await?;
    /// println!("{}: {}", player.player, player.points.points.unwrap_or(0));
    /// ```
    async fn player(&self, player: &str) -> Result<Player> {
        self._generator(&Player::api(player)).await
    }

    /// # Examples
    ///
    /// ```rust,ignore
    /// use ddapi_rs::prelude::*;
    /// use ddapi_rs::prelude::ddnet::*;
    ///
    /// let api = DDApi::new();
    /// let query: Vec<Query> = api.query("nameless tee").await?;
    /// for player in &query {
    ///     println!("{}: {}", player.name, player.points);
    /// }
    /// ```
    async fn query(&self, player: &str) -> Result<Vec<Query>> {
        self._generator(&Query::api(player)).await
    }

    /// # Examples
    ///
    /// ```rust,ignore
    /// use ddapi_rs::prelude::*;
    /// use ddapi_rs::prelude::ddnet::*;
    ///
    /// let api = DDApi::new();
    /// let query: Vec<QueryMap> = api.query_map("multi").await?;
    /// for map in &query {
    ///     println!("{}: {} | {}", map.name, map.mapper, map.r#type);
    /// }
    /// ```
    async fn query_map(&self, map: &str) -> Result<Vec<QueryMap>> {
        self._generator(&QueryMap::api(map)).await
    }

    /// # Examples
    ///
    /// ```rust,ignore
    /// use ddapi_rs::prelude::*;
    /// use ddapi_rs::prelude::ddnet::*;
    ///
    /// let api = DDApi::new();
    /// let query: Vec<QueryMapper> = api.query_mapper("Ao").await?;
    /// for player in &query {
    ///     println!("{}: {}", player.mapper, player.num_maps);
    /// }
    /// ```
    async fn query_mapper(&self, player: &str) -> Result<Vec<QueryMapper>> {
        self._generator(&QueryMapper::api(player)).await
    }

    /// # Examples
    ///
    /// ```rust,ignore
    /// use ddapi_rs::prelude::*;
    /// use ddapi_rs::prelude::ddnet::*;
    ///
    /// let api = DDApi::new();
    /// let map: Map = api.map("Fox").await?;
    /// println!("{}: {}", map.mapper, map.web_preview);
    /// ```
    async fn map(&self, map: &str) -> Result<Map> {
        self._generator(&Map::api(map)).await
    }

    /// # Examples
    ///
    /// ```rust,ignore
    /// use ddapi_rs::prelude::*;
    /// use ddapi_rs::prelude::ddnet::*;
    ///
    /// let api = DDApi::new();
    /// let maps: Vec<ReleasesMaps> = api.releases_map().await?;
    /// for map in &maps {
    ///     println!("{}: {} | {}", map.name, map.mapper, map.r#type);
    /// }
    /// ```
    async fn releases_map(&self) -> Result<Vec<ReleasesMaps>> {
        self._generator_no_cache(&ReleasesMaps::api()).await
    }

    /// # Examples
    ///
    /// ```rust,ignore
    /// use ddapi_rs::prelude::*;
    /// use ddapi_rs::prelude::ddnet::*;
    ///
    /// let api = DDApi::new();
    /// let status: Status = api.status().await?;
    /// for data in &status.servers {
    ///     println!("{}: {} | {}", data.name, data.location, data.host);
    /// }
    /// ```
    async fn status(&self) -> Result<Status> {
        self._generator_no_cache(&Status::api()).await
    }

    async fn latest_finish(&self) -> Result<Vec<LatestFinishes>> {
        self.latest_finish_with_latest(0).await
    }

    async fn latest_finish_with_latest(&self, latest: usize) -> Result<Vec<LatestFinishes>> {
        self._generator_no_cache(&LatestFinishes::api(latest)).await
    }
}
