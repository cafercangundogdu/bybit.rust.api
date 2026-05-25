//! WebSocket Trade (Order Entry) — place, amend, cancel orders via WebSocket.
//!
//! Bybit V5 supports order entry through WebSocket for lower latency
//! compared to REST API. All operations require authentication.
//!
//! # Topics / Operations
//! - `order.create` — Place a new order
//! - `order.amend`  — Amend an existing order
//! - `order.cancel` — Cancel an order
//!
//! # Usage
//!
//! ```ignore
//! client.place_order_via_ws(PlaceOrderRequest { ... }).await?;
//! ```

use crate::dto::{AmendOrderRequest, CancelOrderRequest, PlaceOrderRequest};
use serde::{Deserialize, Serialize};
use serde_json::Value;

// ── Request Types ──────────────────────────────────────────────

/// Order operation types for WebSocket.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TradeOp {
    OrderCreate,
    OrderAmend,
    OrderCancel,
}

/// Request header for WS trade operations.
#[derive(Debug, Clone, Serialize)]
pub struct WsTradeRequest {
    /// Request ID (unique per request, echoed in response)
    #[serde(rename = "reqId")]
    pub req_id: String,
    /// Header with operation type
    pub header: TradeHeader,
    /// Request parameters (order details)
    pub args: Vec<Value>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TradeHeader {
    /// X-BAPI-TIMESTAMP
    #[serde(rename = "X-BAPI-TIMESTAMP")]
    pub timestamp: String,
    /// X-BAPI-RECV-WINDOW
    #[serde(rename = "X-BAPI-RECV-WINDOW")]
    pub recv_window: String,
    /// X-BAPI-API-KEY
    #[serde(rename = "X-BAPI-API-KEY")]
    pub api_key: String,
    /// X-BAPI-SIGN
    #[serde(rename = "X-BAPI-SIGN")]
    pub signature: String,
}

// ── Response Types ─────────────────────────────────────────────

/// Response for WS order operations.
#[derive(Debug, Clone, Deserialize)]
pub struct WsTradeResponse {
    /// Request ID (matches the request)
    #[serde(rename = "reqId")]
    #[serde(default)]
    pub req_id: Option<String>,
    /// Operation type echoed back
    #[serde(rename = "op")]
    #[serde(default)]
    pub op: Option<String>,
    /// Return code (0 = success)
    #[serde(rename = "retCode")]
    #[serde(default)]
    pub ret_code: Option<i32>,
    /// Return message
    #[serde(rename = "retMsg")]
    #[serde(default)]
    pub ret_msg: Option<String>,
    /// Whether the operation succeeded
    #[serde(default)]
    pub success: Option<bool>,
    /// Connection ID
    #[serde(rename = "connId")]
    #[serde(default)]
    pub conn_id: Option<String>,
    /// The result data
    #[serde(default)]
    pub data: Option<TradeResultData>,
}

/// Result data from a WS trade operation.
#[derive(Debug, Clone, Deserialize)]
pub struct TradeResultData {
    /// Order ID
    #[serde(rename = "orderId")]
    #[serde(default)]
    pub order_id: Option<String>,
    /// Client-specified order link ID
    #[serde(rename = "orderLinkId")]
    #[serde(default)]
    pub order_link_id: Option<String>,
    /// Order status
    #[serde(rename = "orderStatus")]
    #[serde(default)]
    pub order_status: Option<String>,
    /// Category
    #[serde(default)]
    pub category: Option<String>,
    /// Symbol
    #[serde(default)]
    pub symbol: Option<String>,
}

// ── Builder / Constructors ─────────────────────────────────────

impl WsTradeRequest {
    /// Generate a unique request ID based on timestamp.
    pub fn new_req_id() -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_micros();
        format!("ws-{}", ts)
    }

    /// Build a place-order WS request with signature.
    pub fn create_order(
        order: PlaceOrderRequest,
        api_key: &str,
        api_secret: &str,
        recv_window: u64,
    ) -> Self {
        let req_id = Self::new_req_id();
        let timestamp = crate::utils::millis().to_string();
        let body = serde_json::to_value(&order).unwrap();

        // Signature: HMAC-SHA256(timestamp + api_key + recv_window + body_json)
        let signature_input = format!(
            "{}{}{}{}",
            timestamp,
            api_key,
            recv_window,
            serde_json::to_string(&body).unwrap()
        );
        let signature = crate::utils::sign(api_secret, &signature_input);

        WsTradeRequest {
            req_id,
            header: TradeHeader {
                timestamp,
                recv_window: recv_window.to_string(),
                api_key: api_key.to_string(),
                signature,
            },
            args: vec![body],
        }
    }

    /// Build an amend-order WS request with signature.
    pub fn amend_order(
        amend: AmendOrderRequest,
        api_key: &str,
        api_secret: &str,
        recv_window: u64,
    ) -> Self {
        let req_id = Self::new_req_id();
        let timestamp = crate::utils::millis().to_string();
        let body = serde_json::to_value(&amend).unwrap();
        let signature_input = format!(
            "{}{}{}{}",
            timestamp,
            api_key,
            recv_window,
            serde_json::to_string(&body).unwrap()
        );
        let signature = crate::utils::sign(api_secret, &signature_input);

        WsTradeRequest {
            req_id,
            header: TradeHeader {
                timestamp,
                recv_window: recv_window.to_string(),
                api_key: api_key.to_string(),
                signature,
            },
            args: vec![body],
        }
    }

    /// Build a cancel-order WS request with signature.
    pub fn cancel_order(
        cancel: CancelOrderRequest,
        api_key: &str,
        api_secret: &str,
        recv_window: u64,
    ) -> Self {
        let req_id = Self::new_req_id();
        let timestamp = crate::utils::millis().to_string();
        let body = serde_json::to_value(&cancel).unwrap();
        let signature_input = format!(
            "{}{}{}{}",
            timestamp,
            api_key,
            recv_window,
            serde_json::to_string(&body).unwrap()
        );
        let signature = crate::utils::sign(api_secret, &signature_input);

        WsTradeRequest {
            req_id,
            header: TradeHeader {
                timestamp,
                recv_window: recv_window.to_string(),
                api_key: api_key.to_string(),
                signature,
            },
            args: vec![body],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dto::PlaceOrderRequest;
    use crate::rest::enums::{Category, OrderType, Side, TimeInForce};

    #[test]
    fn test_create_order_request() {
        let order = PlaceOrderRequest {
            category: Category::Spot,
            symbol: "BTCUSDT".to_string(),
            side: Side::Buy,
            order_type: OrderType::Limit,
            qty: "0.001".to_string(),
            price: Some("40000".to_string()),
            time_in_force: Some(TimeInForce::GTC),
            ..Default::default()
        };

        let req = WsTradeRequest::create_order(order, "key", "secret", 5000);
        assert!(req.req_id.starts_with("ws-"));
        assert_eq!(req.args.len(), 1);
    }

    #[test]
    fn test_req_id_unique() {
        let id1 = WsTradeRequest::new_req_id();
        std::thread::sleep(std::time::Duration::from_micros(10));
        let id2 = WsTradeRequest::new_req_id();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_deserialize_trade_response() {
        let json = serde_json::json!({
            "reqId": "ws-123",
            "op": "order.create",
            "retCode": 0,
            "retMsg": "OK",
            "success": true,
            "data": {
                "orderId": "order-456",
                "orderLinkId": "link-789",
                "orderStatus": "New",
                "category": "spot",
                "symbol": "BTCUSDT"
            }
        });

        let resp: WsTradeResponse = serde_json::from_value(json).unwrap();
        assert_eq!(resp.ret_code, Some(0));
        assert_eq!(resp.success, Some(true));
        assert_eq!(
            resp.data.as_ref().unwrap().order_id.as_deref(),
            Some("order-456")
        );
    }
}
