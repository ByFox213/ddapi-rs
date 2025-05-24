use crate::api::DDApi;
use crate::errors::ApiError;
use crate::scheme::ddnet::*;
use std::future::Future;

#[allow(dead_code)]
pub trait DDnetApi {
    fn master(&self) -> impl Future<Output = Result<Master, ApiError>> + Send;
    fn custom_master(
        &self,
        master: MasterServer,
    ) -> impl Future<Output = Result<Master, ApiError>> + Send;
    fn player(&self, player: &str) -> impl Future<Output = Result<Player, ApiError>> + Send;
    fn query(&self, player: &str) -> impl Future<Output = Result<Vec<Query>, ApiError>> + Send;
    fn query_map(
        &self,
        player: &str,
    ) -> impl Future<Output = Result<Vec<QueryMap>, ApiError>> + Send;
    fn query_mapper(
        &self,
        player: &str,
    ) -> impl Future<Output = Result<Vec<QueryMapper>, ApiError>> + Send;
    fn map(&self, map: &str) -> impl Future<Output = Result<Map, ApiError>> + Send;
    fn releases_map(&self) -> impl Future<Output = Result<Vec<ReleasesMaps>, ApiError>> + Send;
    fn status(&self) -> impl Future<Output = Result<Status, ApiError>> + Send;
}

impl DDnetApi for DDApi {
    async fn master(&self) -> Result<Master, ApiError> {
        self.custom_master(MasterServer::One).await
    }

    async fn custom_master(&self, master: MasterServer) -> Result<Master, ApiError> {
        self._generator(&Master::api(master)).await
    }

    async fn player(&self, player: &str) -> Result<Player, ApiError> {
        self._generator(&Player::api(player)).await
    }
    async fn query(&self, player: &str) -> Result<Vec<Query>, ApiError> {
        self._generator(&Query::api(player)).await
    }

    async fn query_map(&self, player: &str) -> Result<Vec<QueryMap>, ApiError> {
        self._generator(&QueryMap::api(player)).await
    }

    async fn query_mapper(&self, player: &str) -> Result<Vec<QueryMapper>, ApiError> {
        self._generator(&QueryMapper::api(player)).await
    }

    async fn map(&self, map: &str) -> Result<Map, ApiError> {
        self._generator(&Map::api(map)).await
    }
    async fn releases_map(&self) -> Result<Vec<ReleasesMaps>, ApiError> {
        self._generator(&ReleasesMaps::api()).await
    }

    async fn status(&self) -> Result<Status, ApiError> {
        self._generator(&Status::api()).await
    }
}
