use futures::future::join_all;

use crate::services::khl::client::{ClientConfig, KHLApiClient, KhlEndpoint};
use crate::services::khl::structs::*;
use crate::services::traits::ApiResult;
use crate::services::traits::*;

pub struct KHLService {
    pub api_client: KHLApiClient,
}

impl Default for KHLService {
    fn default() -> Self {
        Self::new()
    }
}

impl KHLService {
    pub fn new() -> Self {
        Self {
            api_client: KHLApiClient::new(ClientConfig::default()),
        }
    }

    pub async fn standings(&self) -> ApiResult<Table> {
        let data = self
            .api_client
            .fetch::<KhlEndpoint, Table>(KhlEndpoint::StandingsNow)
            .await?;

        Ok(data)
    }

    pub async fn team(&self, team_id: u32) -> ApiResult<TeamDetail> {
        let data = self
            .api_client
            .fetch::<KhlEndpoint, TeamDetail>(KhlEndpoint::TeamDetails(team_id))
            .await?;

        Ok(data)
    }

    pub async fn fetch_all_teams(&self) -> ApiResult<Vec<TeamDetail>> {
        let all_teams = join_all(
            self.standings()
                .await?
                .divisions
                .into_iter()
                .flat_map(|division| division.teams)
                .map(|team| self.team(team.clubid)),
        )
        .await;

        Ok(all_teams.into_iter().flatten().collect())
    }
}
