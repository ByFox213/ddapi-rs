use crate::api::DDApi;
use crate::scheme::ddnet::*;
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
}

impl DDnetApi for DDApi {
    async fn master(&self) -> Result<Master> {
        self.custom_master(MasterServer::One).await
    }

    async fn skins(&self) -> Result<DDSkins> {
        self._generator(&DDSkins::api()).await
    }

    async fn custom_master(&self, master: MasterServer) -> Result<Master> {
        self._generator(&Master::api(master)).await
    }

    async fn player(&self, player: &str) -> Result<Player> {
        self._generator(&Player::api(player)).await
    }
    async fn query(&self, player: &str) -> Result<Vec<Query>> {
        self._generator(&Query::api(player)).await
    }

    async fn query_map(&self, player: &str) -> Result<Vec<QueryMap>> {
        self._generator(&QueryMap::api(player)).await
    }

    async fn query_mapper(&self, player: &str) -> Result<Vec<QueryMapper>> {
        self._generator(&QueryMapper::api(player)).await
    }

    async fn map(&self, map: &str) -> Result<Map> {
        self._generator(&Map::api(map)).await
    }
    async fn releases_map(&self) -> Result<Vec<ReleasesMaps>> {
        self._generator(&ReleasesMaps::api()).await
    }

    async fn status(&self) -> Result<Status> {
        self._generator(&Status::api()).await
    }
}
