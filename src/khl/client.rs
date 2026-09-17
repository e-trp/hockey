use regex::Regex;
use reqwest::header::{
    ACCEPT, ACCEPT_LANGUAGE, HeaderMap, HeaderValue, ORIGIN, REFERER, USER_AGENT,
};
use reqwest::{Client, ClientBuilder, retry};
use serde::de::DeserializeOwned;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

const BASE_HOST: &str = "www.khl.ru";
const BASE_URL: &str = "https://www.khl.ru";

pub enum ApiEndpoint {
    StandingsNow,
    TeamDetails(u32),
}

pub struct ApiAgrs<'a> {
    path: &'a str,
    args: Vec<(String, String)>,
}

impl ApiEndpoint {
    fn api_args(&self) -> ApiAgrs<'_> {
        match self {
            ApiEndpoint::StandingsNow => ApiAgrs {
                path: "rest/standings/regular/",
                args: vec![("values[type]".to_string(), "regular".to_string())],
            },

            ApiEndpoint::TeamDetails(clubid) => ApiAgrs {
                path: "rest/clubs/main/",
                args: vec![("values[club_id]".to_string(), clubid.to_string())],
            },
        }
    }
}

#[derive(Debug)]
pub enum ApiError {
    Request(reqwest::Error),
    SessionIdNotFound,
}

impl From<reqwest::Error> for ApiError {
    fn from(err: reqwest::Error) -> Self {
        Self::Request(err)
    }
}

pub type ReqwestResult<T> = Result<T, ApiError>;

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

pub struct ApiClient {
    http_client: Client,
    config: ClientConfig,
    session_id: RwLock<Option<Arc<str>>>,
}

impl ApiClient {
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

    async fn get_session_id(&self) -> ReqwestResult<Arc<str>> {
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
            .ok_or(ApiError::SessionIdNotFound)?;

        // Сохраняем session id.
        *self.session_id.write().await = Some(session_id.clone());

        Ok(session_id)
    }

    pub async fn fetch<T: DeserializeOwned>(&self, endpoint: ApiEndpoint) -> ReqwestResult<T> {
        let session_id = self.get_session_id().await?;

        let endpoint_args = endpoint.api_args();

        let mut params = endpoint_args.args;

        params.push(("sessid".to_string(), session_id.to_string()));

        let response = self
            .http_client
            .post(self.build_url(endpoint_args.path))
            .header("X-Requested-With", "XMLHttpRequest")
            .header(ORIGIN, self.config.api_url)
            .header(REFERER, self.config.api_url)
            .form(&params)
            .send()
            .await?
            .error_for_status()?;

        Ok(response.json::<T>().await?)
    }
}
