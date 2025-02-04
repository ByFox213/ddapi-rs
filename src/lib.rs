pub mod error_ddapi;
mod tests;
mod scheme;

use error_ddapi::ApiError;
use reqwest::{Client, Error};
use serde::de::DeserializeOwned;
use std::borrow::Cow;
use urlencoding::encode;
use crate::scheme::ddnet::*;
use crate::scheme::ddstats::*;

pub enum MasterServer {
    One,
    Two,
    Three,
    Four,
}

impl MasterServer {
    #[allow(dead_code)]
    fn get_index(&self) -> i32 {
        match &self {
            Self::One => 1,
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4,
        }
    }
}

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

#[allow(dead_code)]
pub trait DDstats {
    fn splayer(
        &self,
        player: &str,
    ) -> impl std::future::Future<Output = Result<Player, ApiError>> + Send;
}

pub struct DDApi {
    client: Client,
}

impl DDApi {
    pub fn new(client: Client) -> Self {
        DDApi { client }
    }

    async fn send_request(&self, uri: &str) -> Result<String, Error> {
        self.client.get(uri).send().await?.text().await
    }

    async fn _generator<T>(&self, uri: &str) -> Result<T, ApiError>
    where
        T: DeserializeOwned,
    {
        let response = self.send_request(uri).await?;
        Ok(serde_json::from_str(&response)?)
    }

    pub async fn encode_nickname<'a>(&self, nickname: &'a str) -> Cow<'a, str> {
        encode(nickname)
    }
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

impl DDstats for DDApi {
    async fn splayer(&self, player: &str) -> Result<Player, ApiError> {
        self._generator(&*format!(
            "https://ddstats.tw/player/json?player={}",
            self.encode_nickname(player).await
        ))
        .await
    }
}
