//! Position stream — real-time position updates.
//!
//! # Topics
//! - `position` (all categories)
//! - `position.linear`, `position.inverse`, `position.option`

use serde::Deserialize;

/// Position data from private WebSocket.
#[derive(Debug, Clone, Deserialize)]
pub struct PositionData {
    /// Position index (0 = one-way, 1/2 = hedge mode)
    #[serde(rename = "positionIdx")]
    #[serde(default)]
    pub position_idx: Option<i32>,
    /// Risk ID
    #[serde(rename = "riskId")]
    #[serde(default)]
    pub risk_id: Option<i32>,
    /// Symbol
    #[serde(rename = "symbol")]
    #[serde(default)]
    pub symbol: Option<String>,
    /// Side: "Buy" or "Sell" (empty for spot)
    #[serde(rename = "side")]
    #[serde(default)]
    pub side: Option<String>,
    /// Position size
    #[serde(rename = "size")]
    #[serde(default)]
    pub size: Option<String>,
    /// Position value
    #[serde(rename = "positionValue")]
    #[serde(default)]
    pub position_value: Option<String>,
    /// Entry price
    #[serde(rename = "entryPrice")]
    #[serde(default)]
    pub entry_price: Option<String>,
    /// Trade mode: 0=cross, 1=isolated
    #[serde(rename = "tradeMode")]
    #[serde(default)]
    pub trade_mode: Option<i32>,
    /// Auto add margin: 0=off, 1=on
    #[serde(rename = "autoAddMargin")]
    #[serde(default)]
    pub auto_add_margin: Option<i32>,
    /// Leverage
    #[serde(rename = "leverage")]
    #[serde(default)]
    pub leverage: Option<String>,
    /// Position status: "Normal", "Liq", "Adl"
    #[serde(rename = "positionStatus")]
    #[serde(default)]
    pub position_status: Option<String>,
    /// Mark price
    #[serde(rename = "markPrice")]
    #[serde(default)]
    pub mark_price: Option<String>,
    /// Liquidation price
    #[serde(rename = "liqPrice")]
    #[serde(default)]
    pub liq_price: Option<String>,
    /// Bankruptcy price
    #[serde(rename = "bustPrice")]
    #[serde(default)]
    pub bust_price: Option<String>,
    /// Unrealised PnL
    #[serde(rename = "unrealisedPnl")]
    #[serde(default)]
    pub unrealised_pnl: Option<String>,
    /// Cumulative realised PnL
    #[serde(rename = "cumRealisedPnl")]
    #[serde(default)]
    pub cum_realised_pnl: Option<String>,
    /// Take profit price
    #[serde(rename = "takeProfit")]
    #[serde(default)]
    pub take_profit: Option<String>,
    /// Stop loss price
    #[serde(rename = "stopLoss")]
    #[serde(default)]
    pub stop_loss: Option<String>,
    /// Trailing stop
    #[serde(rename = "trailingStop")]
    #[serde(default)]
    pub trailing_stop: Option<String>,
    /// Position IM (initial margin)
    #[serde(rename = "positionIM")]
    #[serde(default)]
    pub position_im: Option<String>,
    /// Position MM (maintenance margin)
    #[serde(rename = "positionMM")]
    #[serde(default)]
    pub position_mm: Option<String>,
    /// Created timestamp
    #[serde(rename = "createdTime")]
    #[serde(default)]
    pub created_time: Option<String>,
    /// Updated timestamp
    #[serde(rename = "updatedTime")]
    #[serde(default)]
    pub updated_time: Option<String>,
    /// Category
    #[serde(rename = "category")]
    #[serde(default)]
    pub category: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_position() {
        let json = serde_json::json!({
            "positionIdx": 0,
            "symbol": "BTCUSDT",
            "side": "Buy",
            "size": "0.5",
            "entryPrice": "50000.00",
            "markPrice": "50100.00",
            "unrealisedPnl": "50.00",
            "leverage": "10",
            "category": "linear"
        });

        let pos: PositionData = serde_json::from_value(json).unwrap();
        assert_eq!(pos.symbol.as_deref(), Some("BTCUSDT"));
        assert_eq!(pos.side.as_deref(), Some("Buy"));
        assert_eq!(pos.size.as_deref(), Some("0.5"));
    }
}
