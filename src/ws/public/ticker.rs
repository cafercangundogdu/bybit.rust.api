//! Ticker stream — 24hr rolling window statistics.
//!
//! # Topic format
//! `tickers.{symbol}` — e.g. `tickers.BTCUSDT`
//!
//! Works for all categories: linear, inverse, spot, option.

use serde::Deserialize;

/// Ticker data for a single symbol (24hr stats).
#[derive(Debug, Clone, Deserialize)]
pub struct TickerData {
    /// Symbol
    #[serde(rename = "symbol")]
    #[serde(default)]
    pub symbol: Option<String>,
    /// Last traded price
    #[serde(rename = "lastPrice")]
    #[serde(default)]
    pub last_price: Option<String>,
    /// 24hr high
    #[serde(rename = "highPrice24h")]
    #[serde(default)]
    pub high_price_24h: Option<String>,
    /// 24hr low
    #[serde(rename = "lowPrice24h")]
    #[serde(default)]
    pub low_price_24h: Option<String>,
    /// Previous 24hr price (for % change calc)
    #[serde(rename = "prevPrice24h")]
    #[serde(default)]
    pub prev_price_24h: Option<String>,
    /// 24hr price change %
    #[serde(rename = "price24hPcnt")]
    #[serde(default)]
    pub price_24h_pcnt: Option<String>,
    /// 24hr volume
    #[serde(rename = "volume24h")]
    #[serde(default)]
    pub volume_24h: Option<String>,
    /// 24hr turnover (USDT value)
    #[serde(rename = "turnover24h")]
    #[serde(default)]
    pub turnover_24h: Option<String>,
    /// Open interest (derivatives)
    #[serde(rename = "openInterest")]
    #[serde(default)]
    pub open_interest: Option<String>,
    /// Open interest value
    #[serde(rename = "openInterestValue")]
    #[serde(default)]
    pub open_interest_value: Option<String>,
    /// Index price (derivatives)
    #[serde(rename = "indexPrice")]
    #[serde(default)]
    pub index_price: Option<String>,
    /// Mark price (derivatives)
    #[serde(rename = "markPrice")]
    #[serde(default)]
    pub mark_price: Option<String>,
    /// Funding rate (derivatives)
    #[serde(rename = "fundingRate")]
    #[serde(default)]
    pub funding_rate: Option<String>,
    /// Next funding timestamp
    #[serde(rename = "nextFundingTime")]
    #[serde(default)]
    pub next_funding_time: Option<String>,
    /// Best bid price
    #[serde(rename = "bid1Price")]
    #[serde(default)]
    pub bid1_price: Option<String>,
    /// Best ask price
    #[serde(rename = "ask1Price")]
    #[serde(default)]
    pub ask1_price: Option<String>,
    /// Category: "spot", "linear", "inverse", "option"
    #[serde(rename = "category")]
    #[serde(default)]
    pub category: Option<String>,
}

/// Typed wrapper for ticker stream data.
pub struct TickerStream;

impl TickerStream {
    /// Parse raw WS data into typed TickerData.
    pub fn parse(data: &serde_json::Value) -> serde_json::Result<TickerData> {
        serde_json::from_value(data.clone())
    }

    /// Check if the given topic matches a ticker channel.
    pub fn matches_topic(topic: &str) -> bool {
        topic.starts_with("tickers.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ticker() {
        let json = serde_json::json!({
            "symbol": "BTCUSDT",
            "lastPrice": "50000.00",
            "highPrice24h": "51000.00",
            "lowPrice24h": "49000.00",
            "volume24h": "15000.5",
            "turnover24h": "750000000.00",
            "price24hPcnt": "0.025",
            "category": "linear"
        });

        let ticker = TickerStream::parse(&json).unwrap();
        assert_eq!(ticker.symbol.as_deref(), Some("BTCUSDT"));
        assert_eq!(ticker.last_price.as_deref(), Some("50000.00"));
        assert_eq!(ticker.category.as_deref(), Some("linear"));
    }

    #[test]
    fn test_matches_topic() {
        assert!(TickerStream::matches_topic("tickers.BTCUSDT"));
        assert!(!TickerStream::matches_topic("orderbook.1.BTCUSDT"));
    }
}
