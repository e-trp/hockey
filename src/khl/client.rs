use regex::Regex;
use reqwest::header::{ACCEPT, ACCEPT_LANGUAGE, HeaderMap, HeaderValue, USER_AGENT};
use reqwest::{Client, ClientBuilder, retry};
use serde::de::DeserializeOwned;
use std::time::Duration;

const BASE_HOST: &str = "www.khl.ru";
const BASE_URL: &str = "https://www.khl.ru";



pub enum ApiEndpoint {
    StandingsNow,
    TeamDetails
}

impl ApiEndpoint{
    fn as_path(&self) -> &str {
        match self {
            ApiEndpoint::StandingsNow => "rest/standings/regular/",
            ApiEndpoint::TeamDetails => "rest/clubs/main/"
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
    session_id: Option<String>,
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
            session_id: None,
        }
    }

    fn build_url(&self, path: &str) -> String {
        format!(
            "{}/{}",
            self.config.api_url,//.trim_end_matches('/'),
            path//.trim_start_matches('/')
        )
    }

    async fn refresh_session(&mut self) -> ReqwestResult<()> {
        let response = self
            .http_client
            .get(self.config.api_url)
            .send()
            .await?
            .error_for_status()?;
        let html_content = response.text().await?;
        match self.config.session_re.captures(&html_content) {
            Some(caps) => {
                self.session_id = Some(caps.get(1).unwrap().as_str().to_string());
                Ok(())
            }
            None => Err(ApiError::SessionIdNotFound),
        }
    }

    pub async fn fetch<T: DeserializeOwned>(&mut self, endpoint: ApiEndpoint) -> ReqwestResult<T> {
        if self.session_id.is_none() {
            self.refresh_session().await?;
        }
        dbg!(self.session_id.clone().unwrap());
        let url = self.build_url(endpoint.as_path());
        dbg!(&url);
        let params = [
            ("values[type]".to_string(), "regular".to_string()),
            ("sessid".to_string(), self.session_id.clone().unwrap()),
        ];
        let response = self
            .http_client
            .post(url)
            .header("X-Requested-With", "XMLHttpRequest")
            .header("Origin", self.config.api_url)
            .header("Referer", self.config.api_url)
            .form(&params)
            .send()
            .await?;
        let json_response = response.json::<T>().await?;
        Ok(json_response)
    }
}
