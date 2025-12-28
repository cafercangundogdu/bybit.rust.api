use bybit_rust_api::{ApiKeyPair, Category, Interval, MarketClient, RestClient};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create API key pair (for public endpoints, you can use empty strings)
    let api_key_pair = ApiKeyPair::new("default".to_string(), "".to_string(), "".to_string());

    // Create REST client
    let rest_client = RestClient::new(api_key_pair, "https://api.bybit.com".to_string());

    // Create Market client
    let market_client = MarketClient::new(rest_client);

    // Get server time
    println!("Getting server time...");
    let server_time = market_client.get_server_time().await?;
    println!("Server time: {:?}", server_time.result);

    // Get BTC/USDT kline data for spot market
    println!("\nGetting BTC/USDT kline data...");
    let kline_data = market_client
        .get_kline(
            Category::Spot,
            "BTCUSDT",
            Interval::OneHour,
            None,
            None,
            Some(10),
        )
        .await?;
    println!("Kline data count: {}", kline_data.result.list.len());

    // Get orderbook
    println!("\nGetting BTC/USDT orderbook...");
    let orderbook = market_client
        .get_orderbook(Category::Spot, "BTCUSDT", Some(5))
        .await?;
    println!("Orderbook: {:?}", orderbook.result);

    // Get tickers
    println!("\nGetting tickers...");
    let tickers = market_client
        .get_tickers(Category::Spot, Some("BTCUSDT"), None, None)
        .await?;
    println!("Tickers: {:?}", tickers.result);

    Ok(())
}
