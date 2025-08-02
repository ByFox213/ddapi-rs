use anyhow::{Context, Result};
use reqwest::Client;
use serde::de::DeserializeOwned;

#[derive(Debug, Clone, Default)]
pub struct DDApi {
    client: Client,
}

impl DDApi {
    pub fn new() -> Self {
        DDApi {
            client: Client::new(),
        }
    }

    pub fn new_with_client(client: Client) -> Self {
        DDApi { client }
    }

    async fn send_request(&self, uri: &str) -> Result<String> {
        let response = self
            .client
            .get(uri)
            .send()
            .await
            .context("Failed to send request")?
            .error_for_status()
            .context("Server returned error status")?;

        let text = response
            .text()
            .await
            .context("Failed to read response body")?;

        if text.is_empty() {
            anyhow::bail!("API returned empty response");
        }

        Ok(text)
    }

    async fn _generator<T>(&self, uri: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let response_text = self.send_request(uri).await?;

        serde_json::from_str(&response_text).context("Failed to parse API response")
    }
}

#[cfg(feature = "ddnet")]
pub mod ddnet;

#[cfg(feature = "ddstats")]
pub mod ddstats;
