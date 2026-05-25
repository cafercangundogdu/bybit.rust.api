//! Execution stream — real-time trade execution updates.
//!
//! # Topics
//! - `execution` (all categories)
//! - `execution.linear`, `execution.inverse`, `execution.option`
//! - `execution.fast` (fast execution, lower latency)

use serde::Deserialize;

/// Execution/trade fill data from private WebSocket.
#[derive(Debug, Clone, Deserialize)]
pub struct ExecutionData {
    /// Category: "linear", "inverse", "spot", "option"
    #[serde(rename = "category")]
    #[serde(default)]
    pub category: Option<String>,
    /// Symbol
    #[serde(rename = "symbol")]
    #[serde(default)]
    pub symbol: Option<String>,
    /// Execution fee rate
    #[serde(rename = "execFee")]
    #[serde(default)]
    pub exec_fee: Option<String>,
    /// Execution ID
    #[serde(rename = "execId")]
    #[serde(default)]
    pub exec_id: Option<String>,
    /// Execution price
    #[serde(rename = "execPrice")]
    #[serde(default)]
    pub exec_price: Option<String>,
    /// Execution quantity
    #[serde(rename = "execQty")]
    #[serde(default)]
    pub exec_qty: Option<String>,
    /// Execution type: "Trade", "AdlTrade", "Funding", "BustTrade"
    #[serde(rename = "execType")]
    #[serde(default)]
    pub exec_type: Option<String>,
    /// Execution value
    #[serde(rename = "execValue")]
    #[serde(default)]
    pub exec_value: Option<String>,
    /// Fee currency
    #[serde(rename = "feeCurrency")]
    #[serde(default)]
    pub fee_currency: Option<String>,
    /// Whether the trade is maker (true) or taker (false)
    #[serde(rename = "isMaker")]
    #[serde(default)]
    pub is_maker: Option<bool>,
    /// Fee rate
    #[serde(rename = "feeRate")]
    #[serde(default)]
    pub fee_rate: Option<String>,
    /// Trade ID for this execution
    #[serde(rename = "tradeIv")]
    #[serde(default)]
    pub trade_iv: Option<String>,
    /// Mark price at execution time
    #[serde(rename = "markIv")]
    #[serde(default)]
    pub mark_iv: Option<String>,
    /// Index price at execution time
    #[serde(rename = "indexIv")]
    #[serde(default)]
    pub index_iv: Option<String>,
    /// Block trade ID
    #[serde(rename = "blockTradeId")]
    #[serde(default)]
    pub block_trade_id: Option<String>,
    /// Mark price
    #[serde(rename = "markPrice")]
    #[serde(default)]
    pub mark_price: Option<String>,
    /// Index price
    #[serde(rename = "indexPrice")]
    #[serde(default)]
    pub index_price: Option<String>,
    /// Underlying price (options)
    #[serde(rename = "underlyingPrice")]
    #[serde(default)]
    pub underlying_price: Option<String>,
    /// Order ID
    #[serde(rename = "orderId")]
    #[serde(default)]
    pub order_id: Option<String>,
    /// Order link ID
    #[serde(rename = "orderLinkId")]
    #[serde(default)]
    pub order_link_id: Option<String>,
    /// Order price
    #[serde(rename = "orderPrice")]
    #[serde(default)]
    pub order_price: Option<String>,
    /// Order quantity
    #[serde(rename = "orderQty")]
    #[serde(default)]
    pub order_qty: Option<String>,
    /// Order type: "Market", "Limit"
    #[serde(rename = "orderType")]
    #[serde(default)]
    pub order_type: Option<String>,
    /// Stop order type
    #[serde(rename = "stopOrderType")]
    #[serde(default)]
    pub stop_order_type: Option<String>,
    /// Side: "Buy" or "Sell"
    #[serde(rename = "side")]
    #[serde(default)]
    pub side: Option<String>,
    /// Execution timestamp
    #[serde(rename = "execTime")]
    #[serde(default)]
    pub exec_time: Option<String>,
    /// Is leverage token?
    #[serde(rename = "isLeverage")]
    #[serde(default)]
    pub is_leverage: Option<String>,
    /// Closed size (for reduce-only orders)
    #[serde(rename = "closedSize")]
    #[serde(default)]
    pub closed_size: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_execution() {
        let json = serde_json::json!({
            "category": "linear",
            "symbol": "BTCUSDT",
            "execId": "abc-123",
            "execPrice": "50000.00",
            "execQty": "0.01",
            "execType": "Trade",
            "side": "Buy",
            "orderId": "order-456",
            "isMaker": false
        });

        let exec: ExecutionData = serde_json::from_value(json).unwrap();
        assert_eq!(exec.symbol.as_deref(), Some("BTCUSDT"));
        assert_eq!(exec.side.as_deref(), Some("Buy"));
        assert_eq!(exec.exec_qty.as_deref(), Some("0.01"));
    }
}
