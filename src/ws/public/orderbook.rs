//! Real-time orderbook stream with snapshot/delta processing.
//!
//! Bybit pushes a full `snapshot` on subscription, then incremental `delta`
//! updates. This module provides a simple consumer that yields each update
//! as-is — local orderbook reconstruction is left to the caller.
//!
//! # Topic format
//! `orderbook.{depth}.{symbol}` — e.g. `orderbook.1.BTCUSDT`
//!
//! # Depths
//! - Linear/Inverse: 1, 50, 200, 1000
//! - Spot: 1, 50, 200, 1000
//! - Option: 25, 100

use serde::Deserialize;

/// A single price level in the orderbook.
#[derive(Debug, Clone, Deserialize)]
pub struct OrderBookLevel {
    /// Price as string (to preserve precision)
    #[serde(rename = "0")]
    pub price: String,
    /// Size/quantity as string
    #[serde(rename = "1")]
    pub size: String,
}

/// Orderbook snapshot or delta data from Bybit.
#[derive(Debug, Clone, Deserialize)]
pub struct OrderBookData {
    /// Symbol (e.g. "BTCUSDT")
    #[serde(rename = "s")]
    pub symbol: String,
    /// Bids: array of [price, size] pairs
    #[serde(rename = "b")]
    pub bids: Vec<OrderBookLevel>,
    /// Asks: array of [price, size] pairs
    #[serde(rename = "a")]
    pub asks: Vec<OrderBookLevel>,
    /// Update ID (monotonically increasing)
    #[serde(rename = "u")]
    pub update_id: u64,
    /// Sequence number
    #[serde(rename = "seq")]
    #[serde(default)]
    pub seq: Option<u64>,
    /// Cross sequence (for snapshot)
    #[serde(rename = "cts")]
    #[serde(default)]
    pub cts: Option<u64>,
    /// Message type: "snapshot" or "delta"
    #[serde(rename = "type")]
    pub msg_type: String,
}

/// High-level typed wrapper around raw WsMessage for orderbook.
///
/// Callers should:
/// 1. Filter by topic matching `orderbook.{depth}.{symbol}`
/// 2. Deserialize `data` field into `OrderBookData`
/// 3. On first message (type="snapshot"), initialize local OB
/// 4. On subsequent messages (type="delta"), apply updates
pub struct OrderBookStream;

impl OrderBookStream {
    /// Parse raw WS data into typed OrderBookData.
    pub fn parse(data: &serde_json::Value) -> serde_json::Result<OrderBookData> {
        serde_json::from_value(data.clone())
    }

    /// Check if the given topic matches an orderbook channel.
    pub fn matches_topic(topic: &str) -> bool {
        topic.starts_with("orderbook.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_snapshot() {
        let json = serde_json::json!({
            "s": "BTCUSDT",
            "b": [["50000.00", "1.5"], ["49900.00", "2.0"]],
            "a": [["50100.00", "1.0"], ["50200.00", "0.5"]],
            "u": 1,
            "seq": 100,
            "type": "snapshot"
        });

        let data: OrderBookData = serde_json::from_value(json).unwrap();
        assert_eq!(data.symbol, "BTCUSDT");
        assert_eq!(data.bids.len(), 2);
        assert_eq!(data.asks.len(), 2);
        assert_eq!(data.bids[0].price, "50000.00");
        assert_eq!(data.bids[0].size, "1.5");
        assert_eq!(data.msg_type, "snapshot");
    }

    #[test]
    fn test_parse_delta() {
        let json = serde_json::json!({
            "s": "ETHUSDT",
            "b": [],
            "a": [["3000.00", "0.1"]],
            "u": 105,
            "seq": 105,
            "type": "delta"
        });

        let data: OrderBookData = serde_json::from_value(json).unwrap();
        assert_eq!(data.symbol, "ETHUSDT");
        assert_eq!(data.asks.len(), 1);
        assert_eq!(data.msg_type, "delta");
    }

    #[test]
    fn test_matches_topic() {
        assert!(OrderBookStream::matches_topic("orderbook.1.BTCUSDT"));
        assert!(OrderBookStream::matches_topic("orderbook.50.ETHUSDT"));
        assert!(!OrderBookStream::matches_topic("publicTrade.BTCUSDT"));
    }
}
