pub mod model;
pub mod error_ddapi;
mod tests;

use std::borrow::Cow;
use reqwest::{Client, Error};
use serde::de::DeserializeOwned;
use urlencoding::encode;
use error_ddapi::ApiError;
use model::*;


pub enum MasterServer
{
    One,
    Two,
    Three,
    Four
}

impl MasterServer {
    #[allow(dead_code)]
    fn get_index(&self) -> i32 {
        match &self {
            Self::One => 1,
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4
        }
    }
}


#[allow(dead_code)]
trait DDnetApi {
    async fn master(&self, master: MasterServer) -> Result<Master, ApiError>;
    async fn player(&self, player: &str) -> Result<DDPlayer, ApiError>;
    async fn query(&self, player: &str) -> Result<Query, ApiError>;
    async fn map(&self, map: &str) -> Result<DMap, ApiError>;
}

#[allow(dead_code)]
trait DDstats {
    async fn splayer(&self, player: &str) -> Result<Player, ApiError>;
}

struct DDApi {
    client: Client,
}

#[allow(dead_code)]
impl<'a> DDApi {
    pub fn new(client: Client) -> DDApi {
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

    fn encode_nickname(&self, nickname: &'a str) -> Cow<'a, str> {
        encode(nickname)
    }
}

impl DDnetApi for DDApi {
    async fn master(&self, master: MasterServer) -> Result<Master, ApiError> {
        self._generator(
            &format!("https://master{}.ddnet.org/ddnet/15/servers.json", master.get_index())
        ).await
    }

    async fn player(&self, player: &str) -> Result<DDPlayer, ApiError> {
        self._generator(
            &format!("https://ddnet.org/players/?json2={}", self.encode_nickname(player))
        ).await
    }
    async fn query(&self, player: &str) -> Result<Query, ApiError> {
        self._generator(
            &format!("https://ddnet.org/players/?query={}", self.encode_nickname(player))
        ).await
    }
    async fn map(&self, map: &str) -> Result<DMap, ApiError> {
        self._generator(
            &format!("https://ddnet.org/maps/?json={}", self.encode_nickname(map))
        ).await
    }
}


impl DDstats for DDApi {
    async fn splayer(&self, player: &str) -> Result<Player, ApiError> {
        self._generator(
            &format!("https://ddstats.tw/player/json?player={}", self.encode_nickname(player))
        ).await
    }
}