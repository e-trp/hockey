use crate::khl::client::{ApiClient, ApiEndpoint, ClientConfig, ReqwestResult};
use crate::khl::structs::*;

pub struct KHLService {
    pub api_client: ApiClient,
}

impl Default for KHLService {
    fn default() -> Self {
        Self::new()
    }
}

impl KHLService {
    pub fn new() -> Self {
        Self {
            api_client: ApiClient::new(ClientConfig::default()),
        }
    }

    pub async fn fetch_standings(&mut self) -> ReqwestResult<Table> {
        let data = self
            .api_client
            .fetch::<Table>(ApiEndpoint::StandingsNow)
            .await?;
        Ok(data)
    }
}
