use hockey::nhl::service::NHLService;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let nhl_service = NHLService::new();
    let data = nhl_service.fetch_standings().await?;
    println!("{:?}", data);
    Ok(())
}
