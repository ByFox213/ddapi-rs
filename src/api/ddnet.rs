use crate::error_ddapi::ApiError;
use crate::{DDApi, MasterServer};
use crate::scheme::ddnet::{DDPlayer, DMap, Master, Query};

#[allow(dead_code)]
pub trait DDnetApi {
    fn master(&self) -> impl std::future::Future<Output = Result<Master, ApiError>> + Send;
    fn master_custom(
        &self,
        master: MasterServer,
    ) -> impl std::future::Future<Output = Result<Master, ApiError>> + Send;
    fn player(
        &self,
        player: &str,
    ) -> impl std::future::Future<Output = Result<DDPlayer, ApiError>> + Send;
    fn query(
        &self,
        player: &str,
    ) -> impl std::future::Future<Output = Result<Query, ApiError>> + Send;
    fn map(&self, map: &str) -> impl std::future::Future<Output = Result<DMap, ApiError>> + Send;
}

impl DDnetApi for DDApi {
    async fn master(&self) -> Result<Master, ApiError> {
        self._generator(&format!(
            "https://master{}.ddnet.org/ddnet/15/servers.json",
            MasterServer::One.get_index()
        ))
            .await
    }

    async fn master_custom(&self, master: MasterServer) -> Result<Master, ApiError> {
        self._generator(&format!(
            "https://master{}.ddnet.org/ddnet/15/servers.json",
            master.get_index()
        ))
            .await
    }

    async fn player(&self, player: &str) -> Result<DDPlayer, ApiError> {
        self._generator(&format!(
            "https://ddnet.org/players/?json2={}",
            self.encode_nickname(player).await
        ))
            .await
    }
    async fn query(&self, player: &str) -> Result<Query, ApiError> {
        self._generator(&format!(
            "https://ddnet.org/players/?query={}",
            self.encode_nickname(player).await
        ))
            .await
    }
    async fn map(&self, map: &str) -> Result<DMap, ApiError> {
        self._generator(&format!(
            "https://ddnet.org/maps/?json={}",
            self.encode_nickname(map).await
        ))
            .await
    }
}