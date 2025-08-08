use anyhow::{Context, Result};
use moka::future::Cache;
use reqwest::Client;
use serde::de::DeserializeOwned;

#[derive(Clone, Default)]
pub struct DDApi {
    client: Client,
    cache: Option<Cache<String, String>>,
}

impl DDApi {
    pub fn new() -> Self {
        DDApi {
            client: Client::new(),
            cache: None,
        }
    }

    pub fn new_with_client(client: Client) -> Self {
        DDApi {
            client,
            cache: None,
        }
    }

    #[cfg(feature = "cache")]
    pub fn set_cache(&mut self, capacity: u64, time_to_live: u64) {
        use std::time::Duration;

        self.cache = Some(
            Cache::builder()
                .max_capacity(capacity)
                .time_to_live(Duration::from_secs(time_to_live))
                .build(),
        );
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

    pub async fn _generator<T>(&self, uri: &str, cache: bool) -> Result<T>
    where
        T: DeserializeOwned + Send + Sync + 'static,
    {
        if cache {
            self._generator_cached(uri).await
        } else {
            self._generator_no_cache(uri).await
        }
    }

    async fn _generator_cached<T>(&self, uri: &str) -> Result<T>
    where
        T: DeserializeOwned + Send + Sync + 'static,
    {
        let type_name = std::any::type_name::<T>();
        let cache_key = format!("{}:{}", type_name, uri);

        match &self.cache {
            Some(cache) => {
                if let Some(value) = cache.get(&cache_key).await {
                    serde_json::from_str(&value).context("Failed to parse CACHE response")
                } else {
                    let response_text = self.send_request(uri).await?;
                    cache.insert(cache_key, response_text.clone()).await;
                    serde_json::from_str(&response_text).context("Failed to parse API response")
                }
            }
            None => self._generator_no_cache(uri).await,
        }
    }

    async fn _generator_no_cache<T>(&self, uri: &str) -> Result<T>
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
