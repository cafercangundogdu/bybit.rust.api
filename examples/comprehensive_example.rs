use bybit_rust_api::rest::{
    AccountClient, ApiKeyPair, AssetClient, MarketClient, OrderClient, PositionClient,
    RestClient, UserClient,
};
use bybit_rust_api::rest::enums::category::Category;
use bybit_rust_api::rest::enums::interval::Interval;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Example demonstrating all the major features of the Bybit Rust SDK
    
    // For public endpoints (no authentication required)
    demo_public_endpoints().await?;
    
    // For private endpoints (requires API credentials)
    if env::var("BYBIT_API_KEY").is_ok() {
        demo_private_endpoints().await?;
    } else {
        println!("\n⚠️  Set BYBIT_API_KEY and BYBIT_API_SECRET to test private endpoints");
    }
    
    Ok(())
}

async fn demo_public_endpoints() -> anyhow::Result<()> {
    println!("📊 Testing Public Endpoints (Market Data)");
    println!("=" .repeat(50));
    
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
    
    // Get server time
    println!("\n📅 Server Time:");
    let server_time = market_client.get_server_time().await?;
    println!("   Timestamp: {}", server_time.result.time);
    
    // Get BTC/USDT ticker
    println!("\n💱 BTC/USDT Ticker:");
    let tickers = market_client
        .get_tickers(Category::UTASpot, Some("BTCUSDT"), None, None)
        .await?;
    if let Some(ticker_list) = tickers.result.list.as_array() {
        if let Some(first) = ticker_list.first() {
            if let Some(price) = first.get("lastPrice") {
                println!("   Last Price: ${}", price);
            }
            if let Some(vol) = first.get("volume24h") {
                println!("   24h Volume: {}", vol);
            }
        }
    }
    
    // Get orderbook
    println!("\n📚 BTC/USDT Orderbook (top 5):");
    let orderbook = market_client
        .get_orderbook(Category::UTASpot, "BTCUSDT", Some(5))
        .await?;
    if let Some(bids) = orderbook.result.get("b").and_then(|b| b.as_array()) {
        println!("   Top Bid: {} @ {}", bids[0][1], bids[0][0]);
    }
    if let Some(asks) = orderbook.result.get("a").and_then(|a| a.as_array()) {
        println!("   Top Ask: {} @ {}", asks[0][1], asks[0][0]);
    }
    
    Ok(())
}

async fn demo_private_endpoints() -> anyhow::Result<()> {
    println!("\n\n🔐 Testing Private Endpoints");
    println!("=" .repeat(50));
    
    // Get API credentials from environment
    let api_key = env::var("BYBIT_API_KEY")?;
    let api_secret = env::var("BYBIT_API_SECRET")?;
    
    // Create API key pair
    let api_key_pair = ApiKeyPair::new(
        "private".to_string(),
        api_key,
        api_secret,
    );
    
    // Create REST client for testnet
    let rest_client = RestClient::new(
        api_key_pair,
        "https://api-testnet.bybit.com".to_string(),
        false,
    );
    
    // Test Account endpoints
    println!("\n👤 Account Information:");
    let account_client = AccountClient::new(rest_client.clone());
    match account_client.get_account_info().await {
        Ok(info) => {
            if let Some(uid) = info.result.get("uid") {
                println!("   UID: {}", uid);
            }
            if let Some(account_type) = info.result.get("unifiedMarginStatus") {
                println!("   Account Type: {}", account_type);
            }
        }
        Err(e) => println!("   Error: {}", e),
    }
    
    // Test wallet balance
    println!("\n💰 Wallet Balance:");
    match account_client.get_wallet_balance("UNIFIED", None).await {
        Ok(balance) => {
            if let Some(list) = balance.result.get("list").and_then(|l| l.as_array()) {
                if let Some(account) = list.first() {
                    if let Some(total_equity) = account.get("totalEquity") {
                        println!("   Total Equity: ${}", total_equity);
                    }
                }
            }
        }
        Err(e) => println!("   Error: {}", e),
    }
    
    // Test Position endpoints
    println!("\n📈 Position Information:");
    let position_client = PositionClient::new(rest_client.clone());
    match position_client.get_position_info("linear", None, None, None, Some(5), None).await {
        Ok(positions) => {
            if let Some(list) = positions.result.get("list").and_then(|l| l.as_array()) {
                println!("   Active Positions: {}", list.len());
                for pos in list.iter().take(2) {
                    if let Some(symbol) = pos.get("symbol") {
                        println!("   - {}", symbol);
                    }
                }
            }
        }
        Err(e) => println!("   Error: {}", e),
    }
    
    // Test Asset endpoints
    println!("\n🏦 Asset Information:");
    let asset_client = AssetClient::new(rest_client.clone());
    match asset_client.get_coin_info(Some("USDT")).await {
        Ok(coin_info) => {
            if let Some(rows) = coin_info.result.get("rows").and_then(|r| r.as_array()) {
                if let Some(coin) = rows.first() {
                    if let Some(name) = coin.get("name") {
                        println!("   Coin Name: {}", name);
                    }
                    if let Some(chains) = coin.get("chains").and_then(|c| c.as_array()) {
                        println!("   Available Chains: {}", chains.len());
                    }
                }
            }
        }
        Err(e) => println!("   Error: {}", e),
    }
    
    // Test User endpoints
    println!("\n👥 User Management:");
    let user_client = UserClient::new(rest_client.clone());
    match user_client.get_member_type().await {
        Ok(member_info) => {
            if let Some(accounts) = member_info.result.get("accounts").and_then(|a| a.as_array()) {
                println!("   Account Count: {}", accounts.len());
            }
        }
        Err(e) => println!("   Error: {}", e),
    }
    
    // Test Order endpoints (without placing real orders)
    println!("\n📋 Order Management:");
    let order_client = OrderClient::new(rest_client.clone());
    match order_client.get_open_orders(
        Category::UTASpot,
        Some("BTCUSDT"),
        None,
        None,
        None,
        None,
        None,
        None,
        Some(5),
        None,
    ).await {
        Ok(orders) => {
            if let Some(list) = orders.result.list.as_array() {
                println!("   Open Orders: {}", list.len());
            }
        }
        Err(e) => println!("   Error: {}", e),
    }
    
    println!("\n✅ All endpoint tests completed!");
    
    Ok(())
}