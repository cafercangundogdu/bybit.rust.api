//! Liquidation stream — real-time forced liquidation orders.
//!
//! # Topic
//! `liquidation.{symbol}` — e.g. `liquidation.BTCUSDT`

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct LiquidationData {
    #[serde(rename = "symbol")]
    #[serde(default)]
    pub symbol: Option<String>,
    #[serde(rename = "side")]
    #[serde(default)]
    pub side: Option<String>,
    #[serde(rename = "price")]
    #[serde(default)]
    pub price: Option<String>,
    #[serde(rename = "size")]
    #[serde(default)]
    pub size: Option<String>,
    #[serde(rename = "updatedTime")]
    #[serde(default)]
    pub updated_time: Option<i64>,
}

pub struct LiquidationStream;

impl LiquidationStream {
    pub fn parse(data: &serde_json::Value) -> serde_json::Result<LiquidationData> {
        serde_json::from_value(data.clone())
    }
    pub fn matches_topic(topic: &str) -> bool {
        topic.starts_with("liquidation.")
    }
}
