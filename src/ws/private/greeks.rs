//! Greeks stream — real-time option Greeks data.
//!
//! # Topic
//! `greeks` (all) or `greeks.{symbol}`

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct GreeksData {
    #[serde(rename = "symbol")]
    #[serde(default)]
    pub symbol: Option<String>,
    /// Delta
    #[serde(rename = "delta")]
    #[serde(default)]
    pub delta: Option<String>,
    /// Gamma
    #[serde(rename = "gamma")]
    #[serde(default)]
    pub gamma: Option<String>,
    /// Vega
    #[serde(rename = "vega")]
    #[serde(default)]
    pub vega: Option<String>,
    /// Theta
    #[serde(rename = "theta")]
    #[serde(default)]
    pub theta: Option<String>,
    /// Implied volatility
    #[serde(rename = "iv")]
    #[serde(default)]
    pub iv: Option<String>,
    /// Mark price
    #[serde(rename = "markPrice")]
    #[serde(default)]
    pub mark_price: Option<String>,
    /// Underlying price
    #[serde(rename = "underlyingPrice")]
    #[serde(default)]
    pub underlying_price: Option<String>,
}
