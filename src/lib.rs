pub mod api;
pub mod error_ddapi;
pub mod scheme;
mod tests;

use error_ddapi::ApiError;
use reqwest::{Client, Error};
use serde::de::DeserializeOwned;
use std::borrow::Cow;
use urlencoding::encode;

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

pub struct DDApi {
    client: Client,
}

impl DDApi {
    pub fn new() -> Self {
        let client = Client::new();
        DDApi { client }
    }

    pub fn new_client(client: Client) -> Self {
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
