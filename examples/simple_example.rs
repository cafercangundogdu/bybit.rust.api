use bybit_rust_api::rest::enums::category::Category;
use bybit_rust_api::rest::{ApiKeyPair, MarketClient, RestClient};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Bybit Rust API - Simple Example");
    println!("{}", "=".repeat(50));

    // Public endpoints (no API key required)
    let api_key_pair = ApiKeyPair::new("public".to_string(), "".to_string(), "".to_string());

    let rest_client = RestClient::new(api_key_pair, "https://api.bybit.com".to_string());

    let market_client = MarketClient::new(rest_client);

    // Get server time
    println!("\n📍 Server Time:");
    match market_client.get_server_time().await {
        Ok(response) => {
            println!("  Time: {}", response.result.time_second);
        }
        Err(e) => println!("  Error: {}", e),
    }

    // Get tickers
    println!("\n📊 Market Tickers:");
    match market_client
        .get_tickers(Category::Spot, None, None, None)
        .await
    {
        Ok(response) => {
            // The response is already deserialized, just print it
            println!("  Response received successfully: {:#?}", response);
        }
        Err(e) => println!("  Error: {}", e),
    }

    Ok(())
}
