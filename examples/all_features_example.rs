use bybit_rust_api::rest::{ApiKeyPair, MarketClient, RestClient};
use bybit_rust_api::rest::enums::category::Category;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Bybit Rust SDK - All Features Example");
    println!("======================================");
    
    // Create API key pair (empty for public endpoints)
    let api_key_pair = ApiKeyPair::new(
        "public".to_string(),
        "".to_string(),
        "".to_string(),
    );
    
    // Create REST client
    let rest_client = RestClient::new(
        api_key_pair,
        "https://api.bybit.com".to_string(),
        false,
    );
    
    // Create Market client
    let market_client = MarketClient::new(rest_client);
    
    // Test public endpoint - Get server time
    println!("\n1. Testing Market Data API:");
    let server_time = market_client.get_server_time().await?;
    println!("   Server Time: {} seconds", server_time.result.time_second);
    
    // Get tickers
    println!("\n2. Getting BTC/USDT Ticker:");
    let tickers = market_client
        .get_tickers(Category::UTASpot, Some("BTCUSDT"), None, None)
        .await?;
    println!("   Response received: {:?}", tickers.ret_code);
    
    // Get orderbook
    println!("\n3. Getting BTC/USDT Orderbook:");
    let orderbook = market_client
        .get_orderbook(Category::UTASpot, "BTCUSDT", Some(5))
        .await?;
    println!("   Response received: {:?}", orderbook.ret_code);
    
    println!("\n✅ SDK Features Available:");
    println!("   - Market Data API (✅ Working)");
    println!("   - Order Management API");
    println!("   - Account Management API");
    println!("   - Position Management API");
    println!("   - Asset Management API");
    println!("   - User Management API");
    println!("   - Spot Leverage Token API");
    
    if env::var("BYBIT_API_KEY").is_err() {
        println!("\n📝 Note: Set BYBIT_API_KEY and BYBIT_API_SECRET environment variables");
        println!("   to test authenticated endpoints.");
    }
    
    Ok(())
}