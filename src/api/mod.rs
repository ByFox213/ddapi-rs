use crate::error::ApiError;
use anyhow::{Context, Result};
#[cfg(feature = "cache")]
use moka::future::Cache;
use reqwest::Client;
use serde::de::DeserializeOwned;
#[allow(unused_imports)]
use std::time::Duration;

#[derive(Clone, Default)]
pub struct DDApi {
    client: Client,
    #[cfg(feature = "cache")]
    cache: Option<Cache<String, String>>,
}

impl DDApi {
    /// Creates a new DDApi instance with default settings
    ///
    /// # Examples
    ///
    /// ```
    /// use ddapi_rs::prelude::*;
    ///
    /// let api = DDApi::new();
    /// ```
    pub fn new() -> Self {
        DDApi {
            client: Client::new(),
            #[cfg(feature = "cache")]
            cache: None,
        }
    }

    /// Creates a new DDApi instance with a custom HTTP client
    ///
    /// This allows you to configure your own client with custom timeouts,
    /// headers, or other settings.
    ///
    /// # Arguments
    ///
    /// * `client` - A pre-configured `reqwest::Client` instance
    ///
    /// # Examples
    ///
    /// ```
    /// use ddapi_rs::prelude::*;
    /// use reqwest::Client;
    ///
    /// let client = Client::builder()
    ///     .timeout(std::time::Duration::from_secs(10))
    ///     .build()
    ///     .unwrap();
    /// let api = DDApi::new_with_client(client);
    /// ```
    pub fn new_with_client(client: Client) -> Self {
        DDApi {
            client,
            #[cfg(feature = "cache")]
            cache: None,
        }
    }

    /// Configures caching for API responses
    ///
    /// When the `cache` feature is enabled, this method allows you to set up
    /// an in-memory cache to reduce API calls and improve performance.
    ///
    /// # Arguments
    ///
    /// * `capacity` - Maximum number of entries to store in the cache
    /// * `time_to_live` - Time in seconds before cached entries expire
    ///
    /// # Examples
    ///
    ///
    /// ```ignore
    /// use ddapi_rs::prelude::*;
    /// use std::time::Duration;
    ///
    /// let mut api = DDApi::new();
    /// api.set_cache(1000, Duration::from_mins(5)); // Cache 1000 items for 5 minutes
    /// ```
    #[cfg(feature = "cache")]
    pub fn set_cache(&mut self, capacity: u64, time_to_live: Duration) {
        self.cache = Some(
            Cache::builder()
                .max_capacity(capacity)
                .time_to_live(time_to_live)
                .build(),
        );
    }

    /// Sends an HTTP GET request to the specified URL and returns the response text
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to send the request to
    ///
    /// # Returns
    ///
    /// Returns `Result<String>` with the response body on success
    async fn send_request(&self, url: &str) -> Result<String> {
        let response = self
            .client
            .get(url)
            .send()
            .await
            .context("Failed to send request")?;

        let text = response
            .text()
            .await
            .context("Failed to read response body")?;

        if text.is_empty() {
            anyhow::bail!("API returned empty response");
        }

        #[cfg(feature = "ddnet")]
        if text == "{}" {
            return Err(anyhow::Error::from(ApiError::NotFound));
        }

        Ok(text)
    }

    /// Executes an API request and deserializes the JSON response
    ///
    /// This method handles API requests and automatically deserializes the JSON response
    /// into the specified type. The caching behavior is determined by the `cache` feature flag.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type to deserialize the response into. Must implement
    ///   `DeserializeOwned + Send + Sync + 'static`
    ///
    /// # Arguments
    ///
    /// * `url` - The API endpoint URL to request
    /// # Returns
    ///
    /// # Returns
    ///
    /// `Result<T>` containing the deserialized data on success, or an error on failure
    pub async fn _generator<T>(&self, url: &str) -> Result<T>
    where
        T: DeserializeOwned + Send + Sync + 'static,
    {
        #[cfg(feature = "cache")]
        {
            self._generator_cached(url).await
        }
        #[cfg(not(feature = "cache"))]
        {
            self._generator_no_cache(url).await
        }
    }

    /// Executes a cached API request
    ///
    /// Checks the cache first for existing responses. If not found in cache,
    /// fetches from the API and stores the response in cache.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type to deserialize the response into
    ///
    /// # Arguments
    ///
    /// * `url` - The API endpoint URL to request
    ///
    /// # Returns
    ///
    /// Returns `Result<T>` with deserialized data from cache or API
    #[cfg(feature = "cache")]
    async fn _generator_cached<T>(&self, url: &str) -> Result<T>
    where
        T: DeserializeOwned + Send + Sync + 'static,
    {
        let type_name = std::any::type_name::<T>();
        let cache_key = format!("{}:{}", type_name, url);

        match &self.cache {
            Some(cache) => {
                if let Some(value) = cache.get(&cache_key).await {
                    self.parse_response::<T>(&value)
                } else {
                    let response_text = self.send_request(url).await?;
                    cache.insert(cache_key, response_text.clone()).await;
                    self.parse_response::<T>(&response_text)
                }
            }
            None => self._generator_no_cache(url).await,
        }
    }

    /// Executes an API request without caching
    ///
    /// Always fetches fresh data from the API, bypassing any cache.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type to deserialize the response into
    ///
    /// # Arguments
    ///
    /// * `url` - The API endpoint URL to request
    ///
    /// # Returns
    ///
    /// Returns `Result<T>` with freshly fetched deserialized data
    pub async fn _generator_no_cache<T>(&self, url: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let response_text = self.send_request(url).await?;
        self.parse_response::<T>(&response_text)
    }

    /// Parses the API response text into the desired type
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type to deserialize the response into
    ///
    /// # Arguments
    ///
    /// * `response_text` - The raw response text from the API
    ///
    /// # Returns
    ///
    /// Returns `Result<T>` with the deserialized data on success, or an appropriate error
    fn parse_response<T>(&self, response_text: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        // ddnet
        #[cfg(feature = "ddnet")]
        if response_text == "{}" {
            return Err(anyhow::Error::from(ApiError::NotFound));
        }

        // ddstats
        #[cfg(feature = "ddstats")]
        if let Ok(error_response) = serde_json::from_str::<serde_json::Value>(response_text) {
            if let Some(error_msg) = error_response.get("error").and_then(|e| e.as_str()) {
                return match error_msg.to_lowercase().as_str() {
                    "player not found" => Err(anyhow::Error::from(ApiError::NotFound)),
                    _ => Err(anyhow::anyhow!(error_msg.to_string())),
                };
            }
        }

        serde_json::from_str(response_text).map_err(Into::into)
    }
}

#[cfg(feature = "ddnet")]
pub mod ddnet;

#[cfg(feature = "ddstats")]
pub mod ddstats;
