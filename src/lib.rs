//! # Bybit Rust API
//!
//! A comprehensive, type-safe Rust SDK for the [Bybit V5 API](https://bybit-exchange.github.io/docs/v5/guide).
//!
//! Covers **all 129 REST endpoints** across 13 modules plus
//! **full WebSocket support** with 17 streaming channels.
//!
//! ## Features

// Bybit API endpoints naturally have many optional parameters;
// this is an intentional design choice for ergonomic usage.
#![allow(clippy::too_many_arguments)]
//!
//! - **REST API**: 129/129 endpoints (Market, Trade, Account, Position, Asset, User, Broker, etc.)
//! - **WebSocket Public**: Orderbook (snapshot/delta), Trade, Ticker, Kline, Liquidation
//! - **WebSocket Private**: Position, Execution, Order, Wallet, Greeks, DCP
//! - **WebSocket Trade**: Place, amend, cancel orders via WebSocket
//! - **Auto-reconnect**: Exponential backoff with automatic re-subscription
//! - **Rate limiting**: Token-bucket limiter for REST and WS
//! - **Authentication**: HMAC-SHA256 signing (REST + WS)
//! - **Type-safe**: Full serde types for all request/response structures
//! - **Error handling**: Structured `BybitError` with all API error codes
//! - **Async**: Built on tokio, implements `futures::Stream` for WS
//!
//! ## Quick Start
//!
//! ### REST (public endpoint, no API key)
//!
//! ```rust,no_run
//! use bybit_rust_api::rest::{ApiKeyPair, MarketClient, RestClient};
//! use bybit_rust_api::{Category, Interval};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let keys = ApiKeyPair::new("public".into(), String::new(), String::new());
//!     let client = RestClient::new(keys, "https://api.bybit.com".into());
//!     let market = MarketClient::new(client);
//!
//!     let time = market.get_server_time().await?;
//!     println!("Server time: {}", time.result.time_second);
//!     Ok(())
//! }
//! ```
//!
//! ### REST (private endpoint, with API key from env)
//!
//! ```rust,no_run
//! use bybit_rust_api::rest::{ApiKeyPair, OrderClient, RestClient};
//! use bybit_rust_api::rest::order::dto::PlaceOrderRequest;
//! use bybit_rust_api::{Category, OrderType, Side};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     dotenvy::dotenv().ok();
//!     let keys = ApiKeyPair::from_env()?;
//!     let client = RestClient::new(keys, "https://api.bybit.com".into());
//!     let order = OrderClient::new(client);
//!
//!     let req = PlaceOrderRequest {
//!         category: Category::Spot,
//!         symbol: "BTCUSDT".into(),
//!         side: Side::Buy,
//!         order_type: OrderType::Limit,
//!         qty: "0.001".into(),
//!         price: Some("40000".into()),
//!         ..Default::default()
//!     };
//!     let resp = order.place_order(req).await?;
//!     println!("Order ID: {}", resp.result.order_id);
//!     Ok(())
//! }
//! ```
//!
//! ### WebSocket (real-time streaming)
//!
//! ```rust,no_run
//! use bybit_rust_api::ws::{WsClient, topics};
//! use futures_util::StreamExt;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let mut client = WsClient::connect(
//!         "wss://stream.bybit.com/v5/public/linear"
//!     ).await?;
//!
//!     client.subscribe(vec![
//!         topics::orderbook(1, "BTCUSDT"),
//!         topics::trade("BTCUSDT"),
//!     ]).await?;
//!
//!     while let Some(msg) = client.next().await {
//!         println!("Topic: {:?}", msg.topic());
//!     }
//!     Ok(())
//! }
//! ```
//!
//! ### CLI
//!
//! ```bash
//! cargo install bybit-rust-api
//! bybit-cli time
//! bybit-cli ticker BTCUSDT
//! bybit-cli kline BTCUSDT 1h 5
//! bybit-cli orderbook BTCUSDT 5
//! ```
//!
//! ## Environment Variables
//!
//! | Variable | Description |
//! |---|---|
//! | `BYBIT_API_KEY` | API key for private endpoints |
//! | `BYBIT_API_SECRET` | API secret for signing |
//! | `BYBIT_TESTNET_API_KEY` | Testnet API key |
//! | `BYBIT_TESTNET_API_SECRET` | Testnet API secret |
//!
//! ## Module Overview
//!
//! | Module | Description |
//! |---|---|
//! | `rest` | REST API clients (Market, Order, Account, Position, Asset, etc.) |
//! | `ws` | WebSocket streams (public, private, trade) |
//! | `rest::enums` | Type-safe enums (Category, Side, Interval, OrderType, etc.) |
//! | `rest::errors` | Error types (`BybitError`, `ErrorCodes`) |
//! | `consts` | API endpoint URLs |
//! | `utils` | HMAC signing, rate limiter |
//!
//! ## Crate Features
//!
//! No feature flags — everything is included by default.
//! TLS is handled via `rustls` (no OpenSSL dependency).

pub mod consts;
pub mod handlers;
pub mod rest;
pub mod utils;
pub mod ws;

// Re-export commonly used types at the top level
pub use rest::{
    AccountClient, AnnouncementsClient, ApiKeyPair, AssetClient, BrokerClient, CryptoLoanClient,
    InstitutionalLoanClient, MarketClient, OrderClient, PositionClient, PreUpgradeClient,
    RestClient, ServerResponse, SpotLeverageTokenClient, SpotMarginTradeClient, UserClient,
};

// Re-export common enums directly
pub use rest::enums::{CancelType, Category, Interval, OrderStatus, OrderType, Side, TimeInForce};

// Organized sub-modules for easier exploration
pub mod enums {
    pub use crate::rest::enums::*;
}

pub mod dto {
    pub use crate::rest::account::dto::*;
    pub use crate::rest::market::dto::*;
    pub use crate::rest::order::dto::*;
}
