//! WebSocket message types for Bybit V5 WebSocket API.
//!
//! Bybit WS messages follow this structure:
//! ```json
//! {
//!   "topic": "orderbook.1.BTCUSDT",
//!   "type": "snapshot",
//!   "ts": 1672828800000,
//!   "data": { ... }
//! }
//! ```
//!
//! For subscribe/unsubscribe operations:
//! ```json
//! { "op": "subscribe", "args": ["orderbook.1.BTCUSDT"] }
//! ```

use serde::{Deserialize, Serialize};

/// Operation types sent to the server
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum WsOp {
    Subscribe,
    Unsubscribe,
    Auth,
    Ping,
}

/// A subscription/unsubscription request to Bybit
#[derive(Debug, Clone, Serialize)]
pub struct WsRequest {
    pub op: WsOp,
    pub args: Vec<serde_json::Value>,
}

impl WsRequest {
    pub fn subscribe(topics: Vec<String>) -> Self {
        WsRequest {
            op: WsOp::Subscribe,
            args: topics.into_iter().map(serde_json::Value::String).collect(),
        }
    }

    pub fn unsubscribe(topics: Vec<String>) -> Self {
        WsRequest {
            op: WsOp::Unsubscribe,
            args: topics.into_iter().map(serde_json::Value::String).collect(),
        }
    }

    pub fn auth(api_key: &str, expires: u64, signature: &str) -> Self {
        WsRequest {
            op: WsOp::Auth,
            args: vec![
                serde_json::Value::String(api_key.to_string()),
                serde_json::Value::Number(serde_json::Number::from(expires)),
                serde_json::Value::String(signature.to_string()),
            ],
        }
    }

    pub fn ping() -> Self {
        WsRequest {
            op: WsOp::Ping,
            args: vec![],
        }
    }
}

/// Response type from Bybit WS (topic-based data)
#[derive(Debug, Clone, Deserialize)]
pub struct WsResponse {
    /// Topic name (e.g. "orderbook.1.BTCUSDT")
    #[serde(default)]
    pub topic: Option<String>,
    /// Message type: "snapshot" or "delta" for streaming data
    #[serde(rename = "type")]
    #[serde(default)]
    pub msg_type: Option<String>,
    /// Timestamp in milliseconds
    #[serde(default)]
    pub ts: Option<i64>,
    /// The actual data payload
    #[serde(default)]
    pub data: Option<serde_json::Value>,
}

/// Operation-level response (subscribe success/error)
#[derive(Debug, Clone, Deserialize)]
pub struct WsOpResponse {
    /// "subscribe", "unsubscribe", "auth", "pong"
    #[serde(default)]
    pub op: Option<String>,
    /// true if operation succeeded
    #[serde(default)]
    pub success: Option<bool>,
    /// Return message
    #[serde(default)]
    pub ret_msg: Option<String>,
    /// Connection ID
    #[serde(default)]
    pub conn_id: Option<String>,
    /// Request arguments echoed back
    #[serde(default)]
    pub req_id: Option<String>,
}

/// Combined response enum to handle both topic messages and op responses
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum WsMessage {
    /// Topic-based data push
    Data(WsResponse),
    /// Operation-level response (subscribe success, pong, auth result)
    Op(WsOpResponse),
}

impl WsMessage {
    /// Returns true if this is a successful subscription confirmation
    pub fn is_subscribe_success(&self) -> bool {
        matches!(self, WsMessage::Op(WsOpResponse {
            op: Some(op),
            success: Some(true),
            ..
        }) if op == "subscribe")
    }

    /// Returns true if this is a successful auth confirmation
    pub fn is_auth_success(&self) -> bool {
        matches!(self, WsMessage::Op(WsOpResponse {
            op: Some(op),
            success: Some(true),
            ..
        }) if op == "auth")
    }

    /// Returns true if this is a pong response
    pub fn is_pong(&self) -> bool {
        matches!(self, WsMessage::Op(WsOpResponse {
            op: Some(op),
            ..
        }) if op == "pong")
    }

    /// Extract topic name from data messages
    pub fn topic(&self) -> Option<&str> {
        match self {
            WsMessage::Data(r) => r.topic.as_deref(),
            _ => None,
        }
    }

    /// Extract message type from data messages
    pub fn msg_type(&self) -> Option<&str> {
        match self {
            WsMessage::Data(r) => r.msg_type.as_deref(),
            _ => None,
        }
    }
}

/// Subscription topic builder
pub mod topics {
    /// Build orderbook topic: orderbook.{depth}.{symbol}
    pub fn orderbook(depth: u16, symbol: &str) -> String {
        format!("orderbook.{}.{}", depth, symbol)
    }

    /// Build public trade topic
    pub fn trade(symbol: &str) -> String {
        format!("publicTrade.{}", symbol)
    }

    /// Build ticker topic variants
    pub mod ticker {
        pub fn linear(symbol: &str) -> String {
            format!("tickers.{}", symbol)
        }
        pub fn inverse(symbol: &str) -> String {
            format!("tickers.{}", symbol)
        }
        pub fn spot(symbol: &str) -> String {
            format!("tickers.{}", symbol)
        }
        pub fn option(symbol: &str) -> String {
            format!("tickers.{}", symbol)
        }
    }

    /// Build kline topic: kline.{interval}.{symbol}
    pub fn kline(interval: &str, symbol: &str) -> String {
        format!("kline.{}.{}", symbol, interval)
    }

    /// Build liquidation topic
    pub fn liquidation(symbol: &str) -> String {
        format!("liquidation.{}", symbol)
    }

    // --- Private topics ---

    /// Private position topic
    pub mod position {
        pub fn all() -> String {
            "position".to_string()
        }
        pub fn linear() -> String {
            "position.linear".to_string()
        }
        pub fn inverse() -> String {
            "position.inverse".to_string()
        }
        pub fn option() -> String {
            "position.option".to_string()
        }
    }

    /// Private execution topic
    pub mod execution {
        pub fn all() -> String {
            "execution".to_string()
        }
        pub fn linear() -> String {
            "execution.linear".to_string()
        }
    }

    /// Private order topic
    pub mod order {
        pub fn all() -> String {
            "order".to_string()
        }
        pub fn linear() -> String {
            "order.linear".to_string()
        }
    }

    /// Private wallet topic
    pub mod wallet {
        pub fn all() -> String {
            "wallet".to_string()
        }
        pub fn linear() -> String {
            "wallet.linear".to_string()
        }
    }
}
