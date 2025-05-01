use std::future::Future;
use crate::error_ddapi::ApiError;
use crate::{DDApi, MasterServer};
use crate::scheme::ddnet::{Player, Map, Master, Query};

#[allow(dead_code)]
pub trait DDnetApi {
    fn master(&self) -> impl Future<Output = Result<Master, ApiError>> + Send;
    fn custom_master(
        &self,
        master: MasterServer,
    ) -> impl Future<Output = Result<Master, ApiError>> + Send;
    fn player(
        &self,
        player: &str,
    ) -> impl Future<Output = Result<Player, ApiError>> + Send;
    fn query(
        &self,
        player: &str,
    ) -> impl Future<Output = Result<Query, ApiError>> + Send;
    fn map(&self, map: &str) -> impl Future<Output = Result<Map, ApiError>> + Send;
}

impl DDnetApi for DDApi {
    async fn master(&self) -> Result<Master, ApiError> {
        self.custom_master(MasterServer::One).await
    }

    async fn custom_master(&self, master: MasterServer) -> Result<Master, ApiError> {
        self._generator(&format!(
            "https://master{}.ddnet.org/ddnet/15/servers.json",
            master.get_index()
        ))
            .await
    }

    async fn player(&self, player: &str) -> Result<Player, ApiError> {
        self._generator(&format!(
            "https://ddnet.org/players/?json2={}",
            self.encode(player).await
        ))
            .await
    }
    async fn query(&self, player: &str) -> Result<Query, ApiError> {
        self._generator(&format!(
            "https://ddnet.org/players/?query={}",
            self.encode(player).await
        ))
            .await
    }
    async fn map(&self, map: &str) -> Result<Map, ApiError> {
        self._generator(&format!(
            "https://ddnet.org/maps/?json={}",
            self.encode(map).await
        ))
            .await
    }
}