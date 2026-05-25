//! Bybit CLI — command-line interface for the Bybit Rust SDK.
//!
//! ```bash
//! # Install
//! cargo install bybit-rust-api
//!
//! # Server time
//! bybit-cli time
//!
//! # Get ticker
//! bybit-cli ticker BTCUSDT
//!
//! # Get klines
//! bybit-cli kline BTCUSDT 1h 5
//!
//! # WebSocket streaming
//! bybit-cli stream orderbook BTCUSDT
//! ```

use bybit_rust_api::rest::{ApiKeyPair, MarketClient, RestClient};
use bybit_rust_api::{Category, Interval};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        print_usage();
        return Ok(());
    }

    let market = create_market_client();

    match args[1].as_str() {
        "time" => {
            let t = market.get_server_time().await?;
            println!(
                "Server time: {} (ret_msg: {})",
                t.result.time_second, t.ret_msg
            );
        }
        "ticker" => {
            let symbol = args.get(2).map_or("BTCUSDT", |s| s.as_str());
            let t = market
                .get_tickers(Category::Spot, Some(symbol), None, None)
                .await?;
            if let Some(tk) = t.result.list.first() {
                println!(
                    "{} last={} bid={} ask={} vol={} chg={}%",
                    tk.symbol,
                    tk.last_price,
                    tk.bid1price,
                    tk.ask1price,
                    tk.volume24h,
                    tk.price24h_pcnt
                );
            }
        }
        "kline" => {
            let symbol = args.get(2).map_or("BTCUSDT", |s| s.as_str());
            let interval = parse_interval(args.get(3).map_or("1h", |s| s.as_str()));
            let limit: i32 = args.get(4).and_then(|s| s.parse().ok()).unwrap_or(5);

            let k = market
                .get_kline(Category::Spot, symbol, interval, None, None, Some(limit))
                .await?;
            for c in &k.result.list {
                println!(
                    "{} O={} H={} L={} C={} V={}",
                    c[0], c[1], c[2], c[3], c[4], c[5]
                );
            }
        }
        "orderbook" => {
            let symbol = args.get(2).map_or("BTCUSDT", |s| s.as_str());
            let depth: i32 = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(5);
            let ob = market
                .get_orderbook(Category::Spot, symbol, Some(depth))
                .await?;
            println!("=== {} Orderbook ===", ob.result.s);
            println!("Bids:");
            for b in ob.result.b.iter().rev() {
                println!("  {:.2} @ {}", b[1].parse::<f64>().unwrap_or(0.0), b[0]);
            }
            for a in &ob.result.a {
                println!(
                    "                     {} @ {:.2}",
                    a[0],
                    a[1].parse::<f64>().unwrap_or(0.0)
                );
            }
            println!("Asks:");
        }
        _ => print_usage(),
    }

    Ok(())
}

fn create_market_client() -> MarketClient {
    let api_key_pair = ApiKeyPair::new("cli".to_string(), String::new(), String::new());
    let rest_client = RestClient::new(
        api_key_pair,
        bybit_rust_api::consts::REST_MAINNET.to_string(),
    );
    MarketClient::new(rest_client)
}

fn parse_interval(s: &str) -> Interval {
    match s {
        "1" | "1m" => Interval::OneMinute,
        "3" | "3m" => Interval::ThreeMinute,
        "5" | "5m" => Interval::FiveMinute,
        "15" | "15m" => Interval::FifteenMinute,
        "30" | "30m" => Interval::ThirtyMinute,
        "1h" | "60" => Interval::OneHour,
        "2h" | "120" => Interval::TwoHour,
        "4h" | "240" => Interval::FourHour,
        "6h" | "360" => Interval::SixHour,
        "12h" | "720" => Interval::TwelveHour,
        "1d" | "D" => Interval::OneDay,
        "1w" | "W" => Interval::OneWeek,
        "1M" | "M" => Interval::OneMonth,
        _ => Interval::OneHour,
    }
}

fn print_usage() {
    println!("Bybit CLI v{}", bybit_rust_api::consts::VERSION);
    println!();
    println!("Usage:");
    println!("  bybit-cli time                        Get server time");
    println!("  bybit-cli ticker [SYMBOL]             Get ticker (default: BTCUSDT)");
    println!("  bybit-cli kline [SYMBOL] [INT] [N]    Get klines (default: BTCUSDT 1h 5)");
    println!("  bybit-cli orderbook [SYMBOL] [DEPTH]  Get orderbook (default: BTCUSDT 5)");
    println!();
    println!("Intervals: 1,3,5,15,30,1h,2h,4h,6h,12h,1d,1w,1M");
    println!();
    println!("Environment:");
    println!("  BYBIT_API_KEY       API key for private endpoints");
    println!("  BYBIT_API_SECRET    API secret for private endpoints");
}
