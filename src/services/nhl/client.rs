use std::borrow::Cow;
use std::time::Duration;

use reqwest::{Client, ClientBuilder, retry};
use serde::de::DeserializeOwned;

use crate::services::traits::*;

const BASE_HOST: &str = "api-web.nhle.com";
const BASE_URL: &str = "https://api-web.nhle.com/v1";

#[allow(unused)]
#[derive(Debug)]
pub enum NhlEndpoint<'a> {
    StandingsNow,
    StandingsByYear(u32),
    TeamDetails { id: &'a str },
}

impl<'a> Endpoint for NhlEndpoint<'a> {
    fn parts(&self) -> RequestParts<'_> {
        match self {
            NhlEndpoint::StandingsNow => RequestParts {
                path: Cow::Borrowed("standings/now"),
                args: None,
            },
            NhlEndpoint::StandingsByYear(year) => RequestParts {
                path: Cow::Owned(format!("standings/{year}")),
                args: None,
            },
            NhlEndpoint::TeamDetails { id } => RequestParts {
                path: Cow::Owned(format!("teams/{id}")),
                args: None,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct ClientConfig {
    host: &'static str,
    api_url: &'static str,
    timeout: Duration,
    retry: u32,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            host: BASE_HOST,
            api_url: BASE_URL,
            timeout: Duration::from_secs(30),
            retry: 3,
        }
    }
}

#[derive(Debug)]
pub struct NHLApiClient {
    config: ClientConfig,
    http_client: Client,
}

impl NHLApiClient {
    pub fn new(config: ClientConfig) -> Self {
        let http_client = ClientBuilder::new()
            .timeout(config.timeout)
            .retry(retry::for_host(config.host).max_retries_per_request(config.retry))
            .build()
            .unwrap();

        Self {
            config,
            http_client,
        }
    }

    fn build_url(&self, path: &str) -> String {
        format!("{}/{}", self.config.api_url, path)
    }
}

impl ApiClient for NHLApiClient {
    async fn fetch<E, T>(&self, endpoint: E) -> ApiResult<T>
    where
        E: Endpoint + Send,
        T: DeserializeOwned + Send,
    {
        let parts = endpoint.parts();
        let url = self.build_url(&parts.path);

        let response = self.http_client.get(url).send().await?;

        let data = response.json::<T>().await?;

        Ok(data)
    }
}
