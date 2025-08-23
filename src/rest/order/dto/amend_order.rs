use serde::{Deserialize, Serialize};
use crate::rest::enums::category::Category;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AmendOrderRequest {
    pub category: Category,
    pub symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_iv: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qty: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tpsl_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub take_profit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_loss: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_trigger_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_trigger_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_limit_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_limit_price: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AmendOrderResponse {
    pub order_id: String,
    pub order_link_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchAmendOrderResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: Vec<BatchAmendResult>,
    pub ret_ext_info: BatchAmendExtInfo,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchAmendResult {
    pub category: Category,
    pub symbol: String,
    pub order_id: String,
    pub order_link_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchAmendExtInfo {
    pub list: Vec<BatchAmendOrderExtInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchAmendOrderExtInfo {
    pub code: i32,
    pub msg: String,
}