use crate::integrations::khl::client::{ApiClient, ApiEndpoint, ClientConfig, ReqwestResult};
use crate::integrations::khl::structs::*;
use futures::future::join_all;

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

    pub async fn fetch_all_teams(&self) -> ReqwestResult<Vec<TeamDetail>> {
        let all_team = join_all(
            self.fetch_standings()
                .await?
                .divisions
                .into_iter()
                .flat_map(|divisions| divisions.teams)
                .map(|team| self.fetch_team(team.clubid)),
        )
        .await;
        Ok(all_team.into_iter().flatten().collect())
    }
}
