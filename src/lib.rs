//! # Bybit Rust API
//!
//! A comprehensive and type-safe Rust SDK for the Bybit API V5.
//!
//! This library provides a clean and easy-to-use interface for interacting with
//! Bybit's exchange, supporting Market Data, Trading, Account management,
//! Position management, and more.
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use bybit_rust_api::{ApiKeyPair, Category, MarketClient, RestClient};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let api_key_pair = ApiKeyPair::new("env".to_string(), "".to_string(), "".to_string());
//!     let rest_client = RestClient::new(api_key_pair, "https://api.bybit.com".to_string());
//!     let market_client = MarketClient::new(rest_client);
//!
//!     let server_time = market_client.get_server_time().await?;
//!     println!("Server time: {:?}", server_time.result);
//!     Ok(())
//! }
//! ```

pub mod consts;
pub mod handlers;
pub mod rest;
pub mod utils;

// Re-export commonly used types at the top level
pub use rest::{
    AccountClient, AnnouncementsClient, ApiKeyPair, AssetClient, BrokerClient, CryptoLoanClient,
    InstitutionalLoanClient, MarketClient, OrderClient, PositionClient, PreUpgradeClient,
    RestClient, ServerResponse, SpotLeverageTokenClient, SpotMarginTradeClient, UserClient,
};

// Re-export common enums directly for convenience
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
