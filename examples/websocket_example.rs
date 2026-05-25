//! WebSocket streaming example — real-time market data from Bybit V5.
//!
//! Usage:
//! ```bash
//! cargo run --example websocket
//! ```
//!
//! For private channels, set env vars:
//! ```bash
//! BYBIT_API_KEY=*** BYBIT_API_SECRET=*** cargo run --example websocket
//! ```

use bybit_rust_api::ws::{
    public::{OrderBookStream, TickerStream, TradeStream},
    topics, WsClient, WsMessage,
};
use futures_util::StreamExt;
use std::time::Duration;
use tokio::time::timeout;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    // Connect to linear (USDT perpetual) public stream
    let url = bybit_rust_api::consts::WS_LINEAR_MAINNET;
    log::info!("Connecting to {} ...", url);

    let mut client = WsClient::connect(url).await?;
    log::info!("Connected!");

    // Subscribe to real-time data
    let topics = vec![
        topics::orderbook(1, "BTCUSDT"),
        topics::trade("BTCUSDT"),
        topics::ticker::linear("BTCUSDT"),
    ];
    log::info!("Subscribing to {} topics...", topics.len());
    client.subscribe(topics).await?;

    // Optional: authenticate for private channels
    if let (Ok(key), Ok(secret)) = (
        std::env::var("BYBIT_API_KEY"),
        std::env::var("BYBIT_API_SECRET"),
    ) {
        if !key.is_empty() && !secret.is_empty() {
            log::info!("Authenticating for private channels...");
            let (expires, sig) = bybit_rust_api::ws::generate_auth_params(&secret);
            client.authenticate(&key, expires, &sig).await?;
            client
                .subscribe(vec![
                    topics::position::linear(),
                    topics::execution::linear(),
                ])
                .await?;
            log::info!("Private channels subscribed!");
        }
    }

    log::info!("Streaming for 30 seconds...\n");

    let result = timeout(Duration::from_secs(30), async {
        while let Some(msg) = client.next().await {
            handle_message(msg);
        }
    })
    .await;

    match result {
        Ok(()) => log::info!("Stream ended"),
        Err(_) => log::info!("30s elapsed, exiting normally"),
    }

    Ok(())
}

fn handle_message(msg: WsMessage) {
    match &msg {
        WsMessage::Op(ref op) => {
            if let Some(op_type) = &op.op {
                match op_type.as_str() {
                    "subscribe" => log::info!("Subscription: success={:?}", op.success),
                    "pong" => log::debug!("Heartbeat"),
                    _ => log::debug!("Op: {:?}", op),
                }
            }
        }
        WsMessage::Data(ref data) => {
            let topic = data.topic.as_deref().unwrap_or("unknown");

            if OrderBookStream::matches_topic(topic) {
                if let Some(ref payload) = data.data {
                    if let Ok(ob) = OrderBookStream::parse(payload) {
                        let bid = ob.bids.first().map(|b| &b.price);
                        let ask = ob.asks.first().map(|a| &a.price);
                        log::info!(
                            "OB {} {} bid={:?} ask={:?}",
                            ob.symbol,
                            ob.msg_type,
                            bid,
                            ask
                        );
                    }
                }
            } else if TradeStream::matches_topic(topic) {
                if let Some(ref payload) = data.data {
                    if let Ok(trades) = TradeStream::parse(payload) {
                        for t in trades.iter().take(2) {
                            log::info!(
                                "Trade {} {} @ {}",
                                t.side.as_deref().unwrap_or("?"),
                                t.size.as_deref().unwrap_or("?"),
                                t.price.as_deref().unwrap_or("?")
                            );
                        }
                    }
                }
            } else if TickerStream::matches_topic(topic) {
                if let Some(ref payload) = data.data {
                    if let Ok(tk) = TickerStream::parse(payload) {
                        log::info!(
                            "Ticker {} last={:?}",
                            tk.symbol.as_deref().unwrap_or("?"),
                            tk.last_price
                        );
                    }
                }
            }
        }
    }
}
