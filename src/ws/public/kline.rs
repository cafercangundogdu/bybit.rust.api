//! Kline/Candlestick stream — real-time OHLCV updates.
//!
//! # Topic format
//! `kline.{interval}.{symbol}` — e.g. `kline.1.BTCUSDT`
//!
//! # Intervals
//! 1, 3, 5, 15, 30, 60, 120, 240, 360, 720, D, W, M
//!
//! Each message contains a single kline (the current candle being updated).

use serde::Deserialize;

/// A single kline/candlestick update.
#[derive(Debug, Clone, Deserialize)]
pub struct KlineData {
    /// Kline start timestamp in ms
    #[serde(rename = "start")]
    #[serde(default)]
    pub start: Option<i64>,
    /// Kline end timestamp in ms
    #[serde(rename = "end")]
    #[serde(default)]
    pub end: Option<i64>,
    /// Kline interval (e.g. "1", "5", "D")
    #[serde(rename = "interval")]
    #[serde(default)]
    pub interval: Option<String>,
    /// Open price
    #[serde(rename = "open")]
    #[serde(default)]
    pub open: Option<String>,
    /// Close price
    #[serde(rename = "close")]
    #[serde(default)]
    pub close: Option<String>,
    /// High price
    #[serde(rename = "high")]
    #[serde(default)]
    pub high: Option<String>,
    /// Low price
    #[serde(rename = "low")]
    #[serde(default)]
    pub low: Option<String>,
    /// Volume
    #[serde(rename = "volume")]
    #[serde(default)]
    pub volume: Option<String>,
    /// Turnover (USDT value)
    #[serde(rename = "turnover")]
    #[serde(default)]
    pub turnover: Option<String>,
    /// Whether this kline is confirmed (final) or still updating
    #[serde(rename = "confirm")]
    #[serde(default)]
    pub confirm: Option<bool>,
    /// Timestamp of this push
    #[serde(rename = "timestamp")]
    #[serde(default)]
    pub timestamp: Option<i64>,
}

/// Typed wrapper for kline stream data.
///
/// Bybit sends an array with a single kline per message.
pub struct KlineStream;

impl KlineStream {
    /// Parse raw WS data into a vector of klines (usually single-element).
    pub fn parse(data: &serde_json::Value) -> serde_json::Result<Vec<KlineData>> {
        serde_json::from_value(data.clone())
    }

    /// Parse and return the first kline (most common case).
    pub fn parse_single(data: &serde_json::Value) -> serde_json::Result<KlineData> {
        let mut klines: Vec<KlineData> = serde_json::from_value(data.clone())?;
        klines
            .pop()
            .ok_or_else(|| serde::de::Error::custom("empty kline array"))
    }

    /// Check if the given topic matches a kline channel.
    pub fn matches_topic(topic: &str) -> bool {
        topic.starts_with("kline.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_kline() {
        let json = serde_json::json!([{
            "start": 1672828800000_i64,
            "end": 1672832400000_i64,
            "interval": "60",
            "open": "50000.00",
            "close": "50100.00",
            "high": "50200.00",
            "low": "49900.00",
            "volume": "150.5",
            "turnover": "7525000.00",
            "confirm": false,
            "timestamp": 1672832100000_i64
        }]);

        let kline = KlineStream::parse_single(&json).unwrap();
        assert_eq!(kline.open.as_deref(), Some("50000.00"));
        assert_eq!(kline.close.as_deref(), Some("50100.00"));
        assert_eq!(kline.confirm, Some(false));
    }

    #[test]
    fn test_matches_topic() {
        assert!(KlineStream::matches_topic("kline.1.BTCUSDT"));
        assert!(KlineStream::matches_topic("kline.D.ETHUSDT"));
        assert!(!KlineStream::matches_topic("tickers.BTCUSDT"));
    }
}
