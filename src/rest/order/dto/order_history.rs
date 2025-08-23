use crate::rest::enums::{
    category::Category, exec_type::ExecType, order_status::OrderStatus, order_type::OrderType,
    side::Side, time_in_force::TimeInForce,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrdersResponse {
    pub list: Vec<OrderInfo>,
    pub next_page_cursor: String,
    pub category: Category,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderInfo {
    pub order_id: String,
    pub order_link_id: String,
    pub block_trade_id: String,
    pub symbol: String,
    pub price: String,
    pub qty: String,
    pub side: Side,
    pub is_leverage: String,
    pub position_idx: i32,
    pub order_status: OrderStatus,
    pub cancel_type: String,
    pub reject_reason: String,
    pub avg_price: String,
    pub leaves_qty: String,
    pub leaves_value: String,
    pub cum_exec_qty: String,
    pub cum_exec_value: String,
    pub cum_exec_fee: String,
    pub time_in_force: TimeInForce,
    pub order_type: OrderType,
    pub stop_order_type: String,
    pub order_iv: String,
    pub trigger_price: String,
    pub take_profit: String,
    pub stop_loss: String,
    pub tpsl_mode: String,
    pub oco_trigger_type: String,
    pub tp_limit_price: String,
    pub sl_limit_price: String,
    pub tp_trigger_by: String,
    pub sl_trigger_by: String,
    pub trigger_direction: i32,
    pub trigger_by: String,
    pub last_price_on_created: String,
    pub reduce_only: bool,
    pub close_on_trigger: bool,
    pub place_type: String,
    pub smp_type: String,
    pub smp_group: i32,
    pub smp_order_id: String,
    pub created_time: String,
    pub updated_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTradeHistoryResponse {
    pub list: Vec<TradeHistory>,
    pub next_page_cursor: String,
    pub category: Category,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeHistory {
    pub symbol: String,
    pub order_id: String,
    pub order_link_id: String,
    pub side: Side,
    pub order_price: String,
    pub order_qty: String,
    pub leaves_qty: String,
    pub order_type: OrderType,
    pub stop_order_type: String,
    pub exec_fee: String,
    pub exec_id: String,
    pub exec_price: String,
    pub exec_qty: String,
    pub exec_type: ExecType,
    pub exec_value: String,
    pub exec_time: String,
    pub is_maker: bool,
    pub fee_rate: String,
    pub trade_iv: String,
    pub mark_iv: String,
    pub mark_price: String,
    pub index_price: String,
    pub underlying_price: String,
    pub block_trade_id: String,
    pub closed_size: String,
    pub seq: i64,
}
