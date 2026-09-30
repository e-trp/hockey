use reqwest;
use serde::de::DeserializeOwned;
use std::borrow::Cow;

#[derive(Debug)]
pub enum ApiError {
    RequestError(reqwest::Error),
}

impl From<reqwest::Error> for ApiError {
    fn from(err: reqwest::Error) -> Self {
        Self::RequestError(err)
    }
}

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug)]
pub enum ServiceError {
    Api(ApiError),
    Deserialize(serde_json::Error),
    Business(String),
}

impl From<ApiError> for ServiceError {
    fn from(err: ApiError) -> Self {
        Self::Api(err)
    }
}

impl From<serde_json::Error> for ServiceError {
    fn from(err: serde_json::Error) -> Self {
        Self::Deserialize(err)
    }
}

pub type ServicetResult<T> = Result<T, ServiceError>;

pub type Param<'a> = (Cow<'a, str>, Cow<'a, str>);

#[derive(Debug)]
pub struct RequestParts<'a> {
    pub path: Cow<'a, str>,
    pub args: Option<Vec<Param<'a>>>,
}

pub trait Endpoint {
    fn parts(&self) -> RequestParts<'_>;
}

#[allow(async_fn_in_trait)]
pub trait ApiClient: Send + Sync {
    async fn fetch<E, T>(&self, endpoint: E) -> ApiResult<T>
    where
        E: Endpoint + Send,
        T: DeserializeOwned + Send;
}

pub trait Service {
    fn standings<T: DeserializeOwned>(&self) -> ServicetResult<T>;
    fn team<T: DeserializeOwned>(&self, team_id: u8) -> ServicetResult<T>;
    fn all_teams<T: DeserializeOwned>(&self) -> ServicetResult<T>;
    fn schedule<T: DeserializeOwned>(&self) -> ServicetResult<T>;
}
