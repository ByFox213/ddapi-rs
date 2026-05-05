use crate::error::{Error, Result};
#[cfg(feature = "cache")]
use moka::future::Cache;
use reqwest::header;
use reqwest::Client;
use serde::de::DeserializeOwned;
#[allow(unused_imports)]
use std::time::Duration;

#[cfg(feature = "cache")]
const DEFAULT_CACHE_TTL: Duration = Duration::from_secs(60 * 10);
#[cfg(feature = "cache")]
const DEFAULT_CACHE_CAPACITY: u64 = 10_000;

#[derive(Clone, Default)]
pub(crate) struct ApiCore {
    client: Client,
    #[cfg(feature = "cache")]
    cache: Option<Cache<String, Vec<u8>>>,
}

impl ApiCore {
    #[cfg(feature = "cache")]
    fn default_cache() -> Cache<String, Vec<u8>> {
        Cache::builder()
            .max_capacity(DEFAULT_CACHE_CAPACITY)
            .time_to_live(DEFAULT_CACHE_TTL)
            .build()
    }

    fn new() -> Self {
        let client = Client::builder()
            .user_agent(concat!(
                env!("CARGO_PKG_NAME"),
                "/",
                env!("CARGO_PKG_VERSION")
            ))
            .default_headers({
                let mut h = header::HeaderMap::new();
                h.insert(
                    header::ACCEPT,
                    header::HeaderValue::from_static("application/json"),
                );
                h
            })
            .build()
            .unwrap_or_else(|_| Client::new());
        Self {
            client,
            #[cfg(feature = "cache")]
            cache: Some(Self::default_cache()),
        }
    }

    fn new_with_client(client: Client) -> Self {
        Self {
            client,
            #[cfg(feature = "cache")]
            cache: Some(Self::default_cache()),
        }
    }

    #[cfg(feature = "cache")]
    fn set_cache(&mut self, capacity: u64, time_to_live: Duration) {
        self.cache = Some(
            Cache::builder()
                .max_capacity(capacity)
                .time_to_live(time_to_live)
                .build(),
        );
    }

    /// Sends an HTTP GET request to the specified URL and returns the raw response body.
    async fn send_request(&self, url: &str) -> Result<Vec<u8>> {
        let response = self
            .client
            .get(url)
            // Avoid hanging forever on large responses while still being generous.
            .timeout(Duration::from_secs(30))
            .send()
            .await?;

        let status = response.status();
        let body = response.bytes().await?.to_vec();

        if body.is_empty() {
            return Err(Error::EmptyBody);
        }

        if !status.is_success() {
            let msg = String::from_utf8_lossy(&body).chars().take(2048).collect();
            return Err(Error::HttpStatus { status, body: msg });
        }

        Ok(body)
    }

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
                    self.parse_response::<T>(value.as_slice())
                } else {
                    let body = self.send_request(url).await?;
                    cache.insert(cache_key, body.clone()).await;
                    self.parse_response::<T>(body.as_slice())
                }
            }
            None => self._generator_no_cache(url).await,
        }
    }

    pub async fn _generator_no_cache<T>(&self, url: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let body = self.send_request(url).await?;
        self.parse_response::<T>(body.as_slice())
    }

    fn parse_response<T>(&self, body: &[u8]) -> Result<T>
    where
        T: DeserializeOwned,
    {
        // ddnet "not found" convention: empty JSON object.
        #[cfg(feature = "ddnet")]
        {
            let trimmed = trim_ascii(body);
            if trimmed == b"{}" {
                return Err(Error::NotFound);
            }
        }

        // ddstats sometimes returns HTTP 200 with { "error": "..." }.
        #[cfg(feature = "ddstats")]
        {
            #[derive(serde::Deserialize)]
            #[serde(untagged)]
            enum MaybeError<T> {
                Err { error: String },
                Ok(T),
            }

            // Single-pass parse: either error envelope or expected payload.
            match serde_json::from_slice::<MaybeError<T>>(body)? {
                MaybeError::Err { error } => {
                    if error.eq_ignore_ascii_case("player not found") {
                        Err(Error::NotFound)
                    } else {
                        Err(Error::RemoteMessage(error))
                    }
                }
                MaybeError::Ok(v) => Ok(v),
            }
        }

        #[cfg(not(feature = "ddstats"))]
        {
            Ok(serde_json::from_slice(body)?)
        }
    }
}

fn trim_ascii(mut s: &[u8]) -> &[u8] {
    while let Some((&b, rest)) = s.split_first() {
        if !b.is_ascii_whitespace() {
            break;
        }
        s = rest;
    }
    while let Some((&b, rest)) = s.split_last() {
        if !b.is_ascii_whitespace() {
            break;
        }
        s = rest;
    }
    s
}

pub trait HasApiCore {
    fn core(&self) -> &ApiCore;
}

#[derive(Clone, Default)]
pub struct DDApi {
    core: ApiCore,
}

impl HasApiCore for DDApi {
    fn core(&self) -> &ApiCore {
        &self.core
    }
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
            core: ApiCore::new(),
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
            core: ApiCore::new_with_client(client),
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
    /// api.set_cache(1000, Duration::from_secs(60 * 5)); // Cache 1000 items for 5 minutes
    /// ```
    #[cfg(feature = "cache")]
    pub fn set_cache(&mut self, capacity: u64, time_to_live: Duration) {
        self.core.set_cache(capacity, time_to_live);
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
        self.core._generator(url).await
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
        self.core._generator_no_cache(url).await
    }
}

#[derive(Clone, Default)]
pub struct DDnetClient {
    core: ApiCore,
}

impl HasApiCore for DDnetClient {
    fn core(&self) -> &ApiCore {
        &self.core
    }
}

impl DDnetClient {
    pub fn new() -> Self {
        Self {
            core: ApiCore::new(),
        }
    }

    pub fn new_with_client(client: Client) -> Self {
        Self {
            core: ApiCore::new_with_client(client),
        }
    }

    #[cfg(feature = "cache")]
    pub fn set_cache(&mut self, capacity: u64, time_to_live: Duration) {
        self.core.set_cache(capacity, time_to_live);
    }
}

#[derive(Clone, Default)]
pub struct DDstatsClient {
    core: ApiCore,
}

impl HasApiCore for DDstatsClient {
    fn core(&self) -> &ApiCore {
        &self.core
    }
}

impl DDstatsClient {
    pub fn new() -> Self {
        Self {
            core: ApiCore::new(),
        }
    }

    pub fn new_with_client(client: Client) -> Self {
        Self {
            core: ApiCore::new_with_client(client),
        }
    }

    #[cfg(feature = "cache")]
    pub fn set_cache(&mut self, capacity: u64, time_to_live: Duration) {
        self.core.set_cache(capacity, time_to_live);
    }
}

#[cfg(feature = "ddnet")]
pub mod ddnet;

#[cfg(feature = "ddstats")]
pub mod ddstats;
