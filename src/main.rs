use hockey::khl::service::KHLService;
use hockey::nhl::service::NHLService;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let nhl_service = NHLService::new();
    let data = nhl_service.fetch_standings().await?;
    println!("{:?}", data);

    let khl_service: KHLService = KHLService::new();
    let response = khl_service.fetch_standings().await;
    println!("{:?}", response);

    let response = khl_service.fetch_team(7u32).await;
    println!("{:?}", response);

    Ok(())
}
