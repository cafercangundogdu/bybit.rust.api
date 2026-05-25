//! Public trade stream — real-time executed trades.
//!
//! # Topic format
//! `publicTrade.{symbol}` — e.g. `publicTrade.BTCUSDT`

use serde::Deserialize;

/// A single public trade execution.
#[derive(Debug, Clone, Deserialize)]
pub struct PublicTrade {
    /// Trade ID
    #[serde(rename = "i")]
    #[serde(default)]
    pub trade_id: Option<String>,
    /// Trade time in milliseconds
    #[serde(rename = "T")]
    #[serde(default)]
    pub timestamp: Option<i64>,
    /// Symbol
    #[serde(rename = "s")]
    #[serde(default)]
    pub symbol: Option<String>,
    /// Side: "Buy" or "Sell"
    #[serde(rename = "S")]
    #[serde(default)]
    pub side: Option<String>,
    /// Trade price
    #[serde(rename = "p")]
    #[serde(default)]
    pub price: Option<String>,
    /// Trade size/quantity
    #[serde(rename = "v")]
    #[serde(default)]
    pub size: Option<String>,
    /// Direction (for linear/inverse futures)
    #[serde(rename = "L")]
    #[serde(default)]
    pub direction: Option<String>,
    /// Is this a block trade?
    #[serde(rename = "BT")]
    #[serde(default)]
    pub block_trade: Option<bool>,
}

/// Typed wrapper for public trade stream data.
///
/// Bybit sends an array of trades in each message.
pub struct TradeStream;

impl TradeStream {
    /// Parse raw WS data into a vector of trades.
    pub fn parse(data: &serde_json::Value) -> serde_json::Result<Vec<PublicTrade>> {
        serde_json::from_value(data.clone())
    }

    /// Check if the given topic matches a public trade channel.
    pub fn matches_topic(topic: &str) -> bool {
        topic.starts_with("publicTrade.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_trades() {
        let json = serde_json::json!([
            {
                "i": "123456",
                "T": 1672828800000_i64,
                "s": "BTCUSDT",
                "S": "Buy",
                "p": "50000.00",
                "v": "0.01",
                "L": "PlusTick",
                "BT": false
            }
        ]);

        let trades = TradeStream::parse(&json).unwrap();
        assert_eq!(trades.len(), 1);
        assert_eq!(trades[0].side.as_deref(), Some("Buy"));
        assert_eq!(trades[0].price.as_deref(), Some("50000.00"));
    }

    #[test]
    fn test_matches_topic() {
        assert!(TradeStream::matches_topic("publicTrade.BTCUSDT"));
        assert!(!TradeStream::matches_topic("orderbook.1.BTCUSDT"));
    }
}
