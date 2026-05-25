//! Market data example — public endpoints, no API key needed.
//!
//! Usage:
//! ```bash
//! cargo run --example market
//! ```

use bybit_rust_api::rest::{ApiKeyPair, MarketClient, RestClient};
use bybit_rust_api::{Category, Interval};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load .env if present (for optional API key)
    dotenvy::dotenv().ok();

    // Public endpoints don't need API keys
    let api_key_pair = ApiKeyPair::new("public".to_string(), String::new(), String::new());
    let rest_client = RestClient::new(
        api_key_pair,
        bybit_rust_api::consts::REST_MAINNET.to_string(),
    );

    let market = MarketClient::new(rest_client);

    // 1. Server time
    println!("=== Server Time ===");
    let time = market.get_server_time().await?;
    println!("  ret_msg: {}", time.ret_msg);
    println!("  server:  {}", time.result.time_second);

    // 2. BTC/USDT kline (last 5 hourly candles)
    println!("\n=== BTCUSDT 1h Klines ===");
    let klines = market
        .get_kline(
            Category::Spot,
            "BTCUSDT",
            Interval::OneHour,
            None,
            None,
            Some(5),
        )
        .await?;
    for k in &klines.result.list {
        println!(
            "  {} open={} high={} low={} close={}",
            k[0], k[1], k[2], k[3], k[4]
        );
    }

    // 3. Orderbook (top 5 levels)
    println!("\n=== BTCUSDT Orderbook (top 5) ===");
    let ob = market
        .get_orderbook(Category::Spot, "BTCUSDT", Some(5))
        .await?;
    println!("  Bids:");
    for bid in ob.result.b.iter().rev() {
        println!("    {} @ {}", bid[1], bid[0]);
    }
    println!("  Asks:");
    for ask in &ob.result.a {
        println!("    {} @ {}", ask[1], ask[0]);
    }

    // 4. Ticker
    println!("\n=== BTCUSDT Ticker ===");
    let ticker = market
        .get_tickers(Category::Spot, Some("BTCUSDT"), None, None)
        .await?;
    if let Some(t) = ticker.result.list.first() {
        println!("  last:    {}", t.last_price);
        println!("  24h_hi:  {}", t.high_price24h);
        println!("  24h_lo:  {}", t.low_price24h);
        println!("  24h_vol: {}", t.volume24h);
        println!("  change:  {}%", t.price24h_pcnt);
    }

    Ok(())
}
