use crate::integrations::khl::client::{ApiClient, ApiEndpoint, ClientConfig, ReqwestResult};
use crate::integrations::khl::structs::*;

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

    pub async fn fetch_standings(&self) -> ReqwestResult<Table> {
        let data = self
            .api_client
            .fetch::<Table>(ApiEndpoint::StandingsNow)
            .await?;
        Ok(data)
    }

    pub async fn fetch_team(&self, teamid: u32) -> ReqwestResult<TeamDetail> {
        let endpoint = ApiEndpoint::TeamDetails(teamid);
        let data = self.api_client.fetch::<TeamDetail>(endpoint).await?;
        Ok(data)
    }
}
