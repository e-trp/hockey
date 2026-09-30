use std::borrow::Cow;
use std::sync::Arc;
use std::time::Duration;

use regex::Regex;
use reqwest::header::{
    ACCEPT, ACCEPT_LANGUAGE, HeaderMap, HeaderValue, ORIGIN, REFERER, USER_AGENT,
};
use reqwest::{Client, ClientBuilder, retry};
use serde::de::DeserializeOwned;
use tokio::sync::RwLock;

use crate::services::traits::{ApiClient as ApiClientTrait, ApiResult, Endpoint, RequestParts};

const BASE_HOST: &str = "www.khl.ru";
const BASE_URL: &str = "https://www.khl.ru";

#[derive(Debug)]
pub enum KhlEndpoint {
    StandingsNow,
    TeamDetails(u32),
}

impl Endpoint for KhlEndpoint {
    fn parts(&self) -> RequestParts<'_> {
        match self {
            KhlEndpoint::StandingsNow => RequestParts {
                path: Cow::Borrowed("rest/standings/regular/"),
                args: Some(vec![(
                    Cow::Borrowed("values[type]"),
                    Cow::Borrowed("regular"),
                )]),
            },

            KhlEndpoint::TeamDetails(club_id) => RequestParts {
                path: Cow::Borrowed("rest/clubs/main/"),
                args: Some(vec![(
                    Cow::Borrowed("values[club_id]"),
                    Cow::Owned(club_id.to_string()),
                )]),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct ClientConfig {
    host: &'static str,
    api_url: &'static str,
    session_re: Regex,
    timeout: Duration,
    retry: u32,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            host: BASE_HOST,
            api_url: BASE_URL,
            session_re: Regex::new(r#"["']bitrix_sessid["']\s*:\s*["']([a-f0-9]{32})["']"#)
                .unwrap(),
            timeout: Duration::from_secs(30),
            retry: 3,
        }
    }
}

#[derive(Debug)]
pub struct KHLApiClient {
    http_client: Client,
    config: ClientConfig,
    session_id: RwLock<Option<Arc<str>>>,
}

impl KHLApiClient {
    pub fn new(config: ClientConfig) -> Self {
        let headers = HeaderMap::from_iter([
            (
                USER_AGENT,
                HeaderValue::from_static(
                    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) \
                     AppleWebKit/537.36 (KHTML, like Gecko) \
                     Chrome/140.0.0.0 Safari/537.36",
                ),
            ),
            (
                ACCEPT_LANGUAGE,
                HeaderValue::from_static("ru-RU,ru;q=0.9,en-US;q=0.8,en;q=0.7"),
            ),
            (
                ACCEPT,
                HeaderValue::from_static("application/json, text/javascript, */*; q=0.01"),
            ),
        ]);

        let http_client = ClientBuilder::new()
            .timeout(config.timeout)
            .cookie_store(true)
            .default_headers(headers)
            .retry(retry::for_host(config.host).max_retries_per_request(config.retry))
            .build()
            .unwrap();

        Self {
            http_client,
            config,
            session_id: RwLock::new(None),
        }
    }

    fn build_url(&self, path: &str) -> String {
        format!("{}/{}", self.config.api_url, path)
    }

    async fn get_session_id(&self) -> ApiResult<Arc<str>> {
        if let Some(session_id) = self.session_id.read().await.as_ref() {
            return Ok(session_id.clone());
        }

        let response = self
            .http_client
            .get(self.config.api_url)
            .send()
            .await?
            .error_for_status()?;

        let html_content = response.text().await?;

        let session_id = self
            .config
            .session_re
            .captures(&html_content)
            .and_then(|caps| caps.get(1))
            .map(|m| Arc::<str>::from(m.as_str()))
            .unwrap();

        *self.session_id.write().await = Some(session_id.clone());

        Ok(session_id)
    }
}

impl ApiClientTrait for KHLApiClient {
    async fn fetch<E, T>(&self, endpoint: E) -> ApiResult<T>
    where
        E: Endpoint + Send,
        T: DeserializeOwned + Send,
    {
        let session_id = self.get_session_id().await?;

        let parts = endpoint.parts();

        let mut params = parts
            .args
            .unwrap_or_default()
            .into_iter()
            .map(|(key, value)| (key.into_owned(), value.into_owned()))
            .collect::<Vec<_>>();

        params.push(("sessid".to_string(), session_id.to_string()));

        let response = self
            .http_client
            .post(self.build_url(&parts.path))
            .header("X-Requested-With", "XMLHttpRequest")
            .header(ORIGIN, self.config.host)
            .header(REFERER, self.config.host)
            .form(&params)
            .send()
            .await?
            .error_for_status()?;

        let data = response.json::<T>().await?;

        Ok(data)
    }
}
