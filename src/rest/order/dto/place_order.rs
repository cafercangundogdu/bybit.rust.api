use serde::{Deserialize, Serialize};

use crate::rest::client::ServerResponse;
use crate::rest::enums::{category::Category, order_type::OrderType, side::Side};

// Place Order endpoint according to the given YAML configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateOrderParams {
    pub category: Category,       // Product type
    pub symbol: String,           // Symbol name, e.g., ETHUSDT
    pub is_leverage: Option<i32>, // Whether to loan, only for spot
    pub side: Side,               // Order side, Buy or Sell
    pub order_type: OrderType,    // Order type, Market or Limit
    pub qty: String,              // Order quantity
    pub price: Option<String>,    // Price, ignore if Market order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_price: Option<String>, // Trigger price for StopOrder
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_direction: Option<i32>, // Trigger direction for StopOrder
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_by: Option<String>, // Trigger by LastPrice, MarkPrice, or IndexPrice
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_filter: Option<String>, // TP/SL, normal order, or conditional order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_iv: Option<String>, // Implied volatility for option orders
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<String>, // Time in force (GTC, IOC, FOK, PostOnly)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_idx: Option<i32>, // Position index for hedge mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>, // Link ID for option orders
    #[serde(skip_serializing_if = "Option::is_none")]
    pub take_profit: Option<String>, // Take profit price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_loss: Option<String>, // Stop loss price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_trigger_by: Option<String>, // TP trigger by price type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_trigger_by: Option<String>, // SL trigger by price type
    pub reduce_only: Option<bool>, // Specify true for close position order
    pub close_on_trigger: Option<bool>, // Close on trigger
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smp_type: Option<String>, // SMP execution type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mmp: Option<bool>, // Market maker protection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tpsl_mode: Option<String>, // TP/SL mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_limit_price: Option<String>, // Limit price when TP is triggered
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_limit_price: Option<String>, // Limit price when SL is triggered
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_order_type: Option<String>, // Order type when TP is triggered
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_order_type: Option<String>, // Order type when SL is triggered
}

// Struct for wrapping a server response to the create order request
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateOrderResponse(ServerResponse<String>); // Assuming the response is just a string message

impl CreateOrderResponse {
    pub fn into_inner(self) -> String {
        self.0.result
    }

    pub fn into_response(self) -> ServerResponse<String> {
        self.0
    }
}
