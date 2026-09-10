#[allow(dead_code)]
#[allow(unused)]
use reqwest::{Client, ClientBuilder};
use serde::de::DeserializeOwned;
use std::borrow::Cow;
use std::time::Duration;

const BASE_URL: &str = "https://api-web.nhle.com/v1";

pub type ReqwestResult<T> = Result<T, reqwest::Error>;

#[derive(Debug)]
pub enum ApiEndpoint<'a> {
    StandingsNow,
    StandingsByYear(u32),
    TeamDetails { id: &'a str },
}

impl<'a> ApiEndpoint<'a> {
    fn as_path(&self) -> Cow<'a, str> {
        match self {
            ApiEndpoint::StandingsNow => Cow::Borrowed("standings/now"),
            ApiEndpoint::StandingsByYear(year) => Cow::Owned(format!("standings/{}", year)),
            ApiEndpoint::TeamDetails { id } => Cow::Owned(format!("teams/{}", id)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ClientConfig {
    base_url:  &'static str,
    timeout: Duration,
}

#[derive(Debug)]
pub struct ApiClient {
    config: ClientConfig,
    http_client: Client,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            base_url: BASE_URL,
            timeout: Duration::from_secs(30),
        }
    }
}

impl ApiClient {
    pub fn new(config: ClientConfig) -> Self {
        let timeout = config.timeout;
        Self {
            config,
            http_client: ClientBuilder::new().timeout(timeout).build().unwrap(),
        }
    }

    fn build_url(&self, path: &str) -> String {
        format!("{}/{}", self.config.base_url, path)
    }

    pub async fn fetch<T: DeserializeOwned>(&self, endpoint: ApiEndpoint<'_>) -> ReqwestResult<T> {
        let url = self.build_url(&endpoint.as_path());
        let data = self.http_client.get(&url).send().await?.json::<T>().await?;
        Ok(data)
    }
}
