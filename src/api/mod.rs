pub mod ddnet;
pub mod ddstats;

use crate::errors::ApiError;
use reqwest::{Client, Error};
use serde::de::DeserializeOwned;

pub struct DDApi {
    client: Client,
}

impl DDApi {
    pub fn new() -> Self {
        let client = Client::new();
        DDApi { client }
    }

    pub fn new_with_client(client: Client) -> Self {
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
}