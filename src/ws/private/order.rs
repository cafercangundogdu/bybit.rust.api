//! Order stream — real-time order status updates.
//!
//! # Topics
//! - `order` (all categories)
//! - `order.linear`, `order.inverse`, `order.spot`, `order.option`

use serde::Deserialize;

/// Order update data from private WebSocket.
#[derive(Debug, Clone, Deserialize)]
pub struct OrderData {
    /// Category: "linear", "inverse", "spot", "option"
    #[serde(rename = "category")]
    #[serde(default)]
    pub category: Option<String>,
    /// Symbol
    #[serde(rename = "symbol")]
    #[serde(default)]
    pub symbol: Option<String>,
    /// Order ID
    #[serde(rename = "orderId")]
    #[serde(default)]
    pub order_id: Option<String>,
    /// Client-specified order link ID
    #[serde(rename = "orderLinkId")]
    #[serde(default)]
    pub order_link_id: Option<String>,
    /// Whether this is a block trade order
    #[serde(rename = "blockTradeId")]
    #[serde(default)]
    pub block_trade_id: Option<String>,
    /// Side: "Buy" or "Sell"
    #[serde(rename = "side")]
    #[serde(default)]
    pub side: Option<String>,
    /// Order type: "Market", "Limit"
    #[serde(rename = "orderType")]
    #[serde(default)]
    pub order_type: Option<String>,
    /// Stop order type
    #[serde(rename = "stopOrderType")]
    #[serde(default)]
    pub stop_order_type: Option<String>,
    /// Order price
    #[serde(rename = "price")]
    #[serde(default)]
    pub price: Option<String>,
    /// Order quantity
    #[serde(rename = "qty")]
    #[serde(default)]
    pub qty: Option<String>,
    /// Time in force: "GTC", "IOC", "FOK", "PostOnly"
    #[serde(rename = "timeInForce")]
    #[serde(default)]
    pub time_in_force: Option<String>,
    /// Order status
    #[serde(rename = "orderStatus")]
    #[serde(default)]
    pub order_status: Option<String>,
    /// Leaves quantity (remaining)
    #[serde(rename = "leavesQty")]
    #[serde(default)]
    pub leaves_qty: Option<String>,
    /// Cumulative executed quantity
    #[serde(rename = "cumExecQty")]
    #[serde(default)]
    pub cum_exec_qty: Option<String>,
    /// Cumulative executed value
    #[serde(rename = "cumExecValue")]
    #[serde(default)]
    pub cum_exec_value: Option<String>,
    /// Cumulative executed fee
    #[serde(rename = "cumExecFee")]
    #[serde(default)]
    pub cum_exec_fee: Option<String>,
    /// Average fill price
    #[serde(rename = "avgPrice")]
    #[serde(default)]
    pub avg_price: Option<String>,
    /// Reject reason (if rejected)
    #[serde(rename = "rejectReason")]
    #[serde(default)]
    pub reject_reason: Option<String>,
    /// Cancel type: "CancelByUser", "CancelByReduceOnly", etc.
    #[serde(rename = "cancelType")]
    #[serde(default)]
    pub cancel_type: Option<String>,
    /// Create type: "CreateByUser", "CreateByClosing"
    #[serde(rename = "createType")]
    #[serde(default)]
    pub create_type: Option<String>,
    /// Is leverage token?
    #[serde(rename = "isLeverage")]
    #[serde(default)]
    pub is_leverage: Option<String>,
    /// Position index
    #[serde(rename = "positionIdx")]
    #[serde(default)]
    pub position_idx: Option<i32>,
    /// Take profit price
    #[serde(rename = "takeProfit")]
    #[serde(default)]
    pub take_profit: Option<String>,
    /// Stop loss price
    #[serde(rename = "stopLoss")]
    #[serde(default)]
    pub stop_loss: Option<String>,
    /// Trigger price (for conditional orders)
    #[serde(rename = "triggerPrice")]
    #[serde(default)]
    pub trigger_price: Option<String>,
    /// Trigger direction: 1=rise, 2=fall
    #[serde(rename = "triggerDirection")]
    #[serde(default)]
    pub trigger_direction: Option<i32>,
    /// Trigger by: "LastPrice", "IndexPrice", "MarkPrice"
    #[serde(rename = "triggerBy")]
    #[serde(default)]
    pub trigger_by: Option<String>,
    /// Whether to reduce-only
    #[serde(rename = "reduceOnly")]
    #[serde(default)]
    pub reduce_only: Option<bool>,
    /// Whether to close on trigger
    #[serde(rename = "closeOnTrigger")]
    #[serde(default)]
    pub close_on_trigger: Option<bool>,
    /// SMP type
    #[serde(rename = "smpType")]
    #[serde(default)]
    pub smp_type: Option<String>,
    /// SMP group
    #[serde(rename = "smpGroup")]
    #[serde(default)]
    pub smp_group: Option<i32>,
    /// Created timestamp
    #[serde(rename = "createdTime")]
    #[serde(default)]
    pub created_time: Option<String>,
    /// Updated timestamp
    #[serde(rename = "updatedTime")]
    #[serde(default)]
    pub updated_time: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_order() {
        let json = serde_json::json!({
            "category": "linear",
            "symbol": "BTCUSDT",
            "orderId": "order-789",
            "side": "Buy",
            "orderType": "Limit",
            "price": "50000.00",
            "qty": "0.01",
            "orderStatus": "New",
            "leavesQty": "0.01",
            "cumExecQty": "0"
        });

        let order: OrderData = serde_json::from_value(json).unwrap();
        assert_eq!(order.symbol.as_deref(), Some("BTCUSDT"));
        assert_eq!(order.order_status.as_deref(), Some("New"));
        assert_eq!(order.qty.as_deref(), Some("0.01"));
    }
}
