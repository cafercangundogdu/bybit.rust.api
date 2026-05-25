//! WebSocket module for Bybit V5 real-time data streams.
//!
//! Provides connection management, authentication, and typed data
//! structures for all Bybit WebSocket channels.
//!
//! # Quick Start
//!
//! ```ignore
//! use bybit_rust_api::ws::{WsClient, topics};
//! use futures_util::StreamExt;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let client = WsClient::connect(
//!         "wss://stream.bybit.com/v5/public/linear"
//!     ).await?;
//!
//!     client.subscribe(vec![
//!         topics::orderbook(1, "BTCUSDT"),
//!         topics::trade("BTCUSDT"),
//!         topics::kline("1", "BTCUSDT"),
//!     ]).await?;
//!
//!     while let Some(msg) = client.next().await {
//!         println!("Topic: {:?}", msg.topic());
//!     }
//!     Ok(())
//! }
//! ```

pub mod auth;
pub mod client;
pub mod messages;
pub mod private;
pub mod public;
pub mod trade;

// Re-export key types
pub use auth::generate_auth_params;
pub use client::WsClient;
pub use messages::{topics, WsMessage, WsOpResponse, WsRequest, WsResponse};
