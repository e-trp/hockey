use crate::services::nhl::client::{ClientConfig, NHLApiClient, NhlEndpoint};
use crate::services::nhl::structs::*;
use crate::services::traits::ApiClient;
use crate::services::traits::ApiResult;

pub struct NHLService {
    pub api_client: NHLApiClient,
}

impl Default for NHLService {
    fn default() -> Self {
        Self::new()
    }
}

impl NHLService {
    pub fn new() -> Self {
        Self {
            api_client: NHLApiClient::new(ClientConfig::default()),
        }
    }

    pub async fn standings(&self) -> ApiResult<Table> {
        let data = self
            .api_client
            .fetch::<NhlEndpoint, Table>(NhlEndpoint::StandingsNow)
            .await?;

        Ok(data)
    }
}
