use crate::nhl::client::{ApiClient, ApiEndpoint, ClientConfig, ReqwestResult};
use crate::nhl::structs::*;

pub struct NHLService {
    pub api_client: ApiClient,
}

impl Default for NHLService {
    fn default() -> Self {
        Self::new()
    }
}

impl NHLService {
    pub fn new() -> Self {
        Self {
            api_client: ApiClient::new(ClientConfig::default()),
        }
    }

    pub async fn fetch_standings(&self) -> ReqwestResult<Table> {
        let data = self
            .api_client
            .fetch::<Table>(ApiEndpoint::StandingsNow)
            .await?;
        Ok(data)
    }
}
