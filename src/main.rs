use hockey::services::traits::*;
use hockey::services::{KHLService, NHLService};

#[tokio::main]
async fn main() -> ServicetResult<()> {
    let nhl_service = NHLService::new();
    let data = nhl_service.standings().await?;
    println!("{:?}", data);

    let khl_service: KHLService = KHLService::new();
    let response = khl_service.standings().await;
    println!("{:?}", response);

    let response = khl_service.team(7u32).await;
    println!("{:?}", response);

    let response = khl_service.fetch_all_teams().await;
    println!("{:?}", response);

    Ok(())
}
