# Bybit Rust SDK

A Rust SDK for the Bybit API V5, providing easy access to market data and trading functionality.

## Features

- ✅ Market Data API (public endpoints)
- ✅ Order Management (private endpoints)
- ✅ Account Management
- ✅ Position Management
- ✅ Asset Management (deposits, withdrawals, transfers)
- ✅ User Management (sub-accounts, API keys)
- ✅ Spot Leverage Token
- ✅ Comprehensive error handling
- ✅ Type-safe request/response structures
- ✅ Authentication and request signing
- 🚧 WebSocket support (coming soon)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
bybit_rust_api = "0.2.0"
```

## Quick Start

### Market Data (Public)

```rust
use bybit_rust_api::rest::{ApiKeyPair, MarketClient, RestClient};
use bybit_rust_api::rest::enums::category::Category;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create API key pair (empty for public endpoints)
    let api_key_pair = ApiKeyPair::new(
        "default".to_string(),
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
    let server_time = market_client.get_server_time().await?;
    println!("Server time: {:?}", server_time.result);
    
    Ok(())
}
```

### Trading (Private)

```rust
use bybit_rust_api::rest::{ApiKeyPair, OrderClient, RestClient};
use bybit_rust_api::rest::order::dto::PlaceOrderRequest;
use bybit_rust_api::rest::enums::{Category, OrderType, Side, TimeInForce};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Get API credentials from environment
    let api_key = std::env::var("BYBIT_API_KEY")?;
    let api_secret = std::env::var("BYBIT_API_SECRET")?;
    
    // Create API key pair
    let api_key_pair = ApiKeyPair::new(
        "trading".to_string(),
        api_key,
        api_secret,
    );
    
    // Create REST client
    let rest_client = RestClient::new(
        api_key_pair,
        "https://api.bybit.com".to_string(),
        false,
    );
    
    // Create Order client
    let order_client = OrderClient::new(rest_client);
    
    // Place a limit order
    let order_request = PlaceOrderRequest {
        category: Category::UTASpot,
        symbol: "BTCUSDT".to_string(),
        side: Side::Buy,
        order_type: OrderType::Limit,
        qty: "0.001".to_string(),
        price: Some("40000".to_string()),
        time_in_force: Some(TimeInForce::GTC),
        // ... other optional fields
    };
    
    let response = order_client.place_order(order_request).await?;
    println!("Order placed: {}", response.result.order_id);
    
    Ok(())
}
```

## API Coverage

### Market Data
- [x] Server Time
- [x] Kline/Candlestick
- [x] Mark Price Kline
- [x] Index Price Kline
- [x] Premium Index Price Kline
- [x] Orderbook
- [x] Instruments Info
- [x] Tickers
- [x] Funding Rate History
- [x] Risk Limit
- [x] Open Interest
- [x] Insurance
- [x] Recent Trades
- [x] Delivery Price
- [x] Long/Short Ratio
- [x] Historical Volatility

### Order Management
- [x] Place Order
- [x] Batch Place Orders
- [x] Amend Order
- [x] Batch Amend Orders
- [x] Cancel Order
- [x] Batch Cancel Orders
- [x] Cancel All Orders
- [x] Get Open Orders
- [x] Get Order History
- [x] Get Trade History

### Account Management
- [x] Get Wallet Balance
- [x] Get Fee Rate
- [x] Get Account Info
- [x] Get Transaction Log
- [x] Set Margin Mode
- [x] Set/Reset MMP
- [x] Get MMP State
- [x] Get Collateral Info
- [x] Get Borrow History
- [x] Upgrade to Unified Account

### Position Management
- [x] Get Position Info
- [x] Set Leverage
- [x] Switch Margin Mode
- [x] Switch Position Mode
- [x] Set Trading Stop (TP/SL)
- [x] Set Auto Add Margin
- [x] Get Closed PnL
- [x] Set Risk Limit
- [x] Move Positions
- [x] Add/Reduce Margin

### Asset Management
- [x] Deposit/Withdrawal Operations
- [x] Internal & Universal Transfers
- [x] Get Asset Info
- [x] Convert Operations (Quote/Execute)
- [x] Get Coin Info
- [x] Delivery & Settlement Records

### User Management
- [x] Create/Manage Sub Members
- [x] Create/Manage API Keys
- [x] Get Member Type
- [x] Freeze/Delete Sub Members

### Spot Leverage Token
- [x] Get Leverage Token Info
- [x] Purchase/Redeem Operations
- [x] Get Order Records

## Environment Variables

For private endpoints, set these environment variables:

```bash
export BYBIT_API_KEY="your-api-key"
export BYBIT_API_SECRET="your-api-secret"
```

## Examples

Run the examples:

```bash
# Market data example
cargo run --example market

# Trading example (requires API credentials)
cargo run --example trading
```

## Testing

For testing, use the Bybit testnet:

```rust
let rest_client = RestClient::new(
    api_key_pair,
    "https://api-testnet.bybit.com".to_string(),
    true, // enable debug mode
);
```

## License

MIT

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Disclaimer

This is an unofficial SDK. Please use at your own risk.