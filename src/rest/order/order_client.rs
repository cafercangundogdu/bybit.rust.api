use crate::rest::client::{RestClient, SecType, ServerResponse};
use crate::rest::enums::category::Category;
use crate::rest::order::dto::*;
use anyhow::Result;
use serde_json::json;

#[derive(Clone)]
pub struct OrderClient {
    client: RestClient,
}

impl OrderClient {
    pub fn new(client: RestClient) -> Self {
        OrderClient { client }
    }

    /// Place an order
    ///
    /// API: POST /v5/order/create
    /// https://bybit-exchange.github.io/docs/v5/order/create-order
    pub async fn place_order(
        &self,
        order: PlaceOrderRequest,
    ) -> Result<ServerResponse<PlaceOrderResponse>> {
        let endpoint = "v5/order/create";
        let body = serde_json::to_value(&order)?;

        let response: ServerResponse<PlaceOrderResponse> =
            self.client.post(endpoint, body, SecType::Signed).await?;
        Ok(response)
    }

    /// Batch place orders (Option only)
    ///
    /// API: POST /v5/order/create-batch
    /// https://bybit-exchange.github.io/docs/v5/order/batch-place
    pub async fn batch_place_orders(
        &self,
        category: Category,
        orders: Vec<PlaceOrderRequest>,
    ) -> Result<ServerResponse<BatchPlaceOrderResponse>> {
        let endpoint = "v5/order/create-batch";
        let body = json!({
            "category": category,
            "request": orders
        });

        let response: ServerResponse<BatchPlaceOrderResponse> =
            self.client.post(endpoint, body, SecType::Signed).await?;
        Ok(response)
    }

    /// Amend order
    ///
    /// API: POST /v5/order/amend
    /// https://bybit-exchange.github.io/docs/v5/order/amend-order
    pub async fn amend_order(
        &self,
        amend_request: AmendOrderRequest,
    ) -> Result<ServerResponse<AmendOrderResponse>> {
        let endpoint = "v5/order/amend";
        let body = serde_json::to_value(&amend_request)?;

        let response: ServerResponse<AmendOrderResponse> =
            self.client.post(endpoint, body, SecType::Signed).await?;
        Ok(response)
    }

    /// Batch amend orders (Option only)
    ///
    /// API: POST /v5/order/amend-batch
    /// https://bybit-exchange.github.io/docs/v5/order/batch-amend
    pub async fn batch_amend_orders(
        &self,
        category: Category,
        amendments: Vec<AmendOrderRequest>,
    ) -> Result<ServerResponse<BatchAmendOrderResponse>> {
        let endpoint = "v5/order/amend-batch";
        let body = json!({
            "category": category,
            "request": amendments
        });

        let response: ServerResponse<BatchAmendOrderResponse> =
            self.client.post(endpoint, body, SecType::Signed).await?;
        Ok(response)
    }

    /// Cancel order
    ///
    /// API: POST /v5/order/cancel
    /// https://bybit-exchange.github.io/docs/v5/order/cancel-order
    pub async fn cancel_order(
        &self,
        cancel_request: CancelOrderRequest,
    ) -> Result<ServerResponse<CancelOrderResponse>> {
        let endpoint = "v5/order/cancel";
        let body = serde_json::to_value(&cancel_request)?;

        let response: ServerResponse<CancelOrderResponse> =
            self.client.post(endpoint, body, SecType::Signed).await?;
        Ok(response)
    }

    /// Batch cancel orders (Option only)
    ///
    /// API: POST /v5/order/cancel-batch
    /// https://bybit-exchange.github.io/docs/v5/order/batch-cancel
    pub async fn batch_cancel_orders(
        &self,
        category: Category,
        cancellations: Vec<CancelOrderRequest>,
    ) -> Result<ServerResponse<BatchCancelOrderResponse>> {
        let endpoint = "v5/order/cancel-batch";
        let body = json!({
            "category": category,
            "request": cancellations
        });

        let response: ServerResponse<BatchCancelOrderResponse> =
            self.client.post(endpoint, body, SecType::Signed).await?;
        Ok(response)
    }

    /// Cancel all orders
    ///
    /// API: POST /v5/order/cancel-all
    /// https://bybit-exchange.github.io/docs/v5/order/cancel-all
    pub async fn cancel_all_orders(
        &self,
        category: Category,
        symbol: Option<&str>,
        base_coin: Option<&str>,
        settle_coin: Option<&str>,
        order_filter: Option<&str>,
    ) -> Result<ServerResponse<CancelAllOrdersResponse>> {
        let endpoint = "v5/order/cancel-all";
        let mut body = json!({
            "category": category,
        });

        if let Some(symbol) = symbol {
            body["symbol"] = json!(symbol);
        }
        if let Some(base_coin) = base_coin {
            body["baseCoin"] = json!(base_coin);
        }
        if let Some(settle_coin) = settle_coin {
            body["settleCoin"] = json!(settle_coin);
        }
        if let Some(order_filter) = order_filter {
            body["orderFilter"] = json!(order_filter);
        }

        let response: ServerResponse<CancelAllOrdersResponse> =
            self.client.post(endpoint, body, SecType::Signed).await?;
        Ok(response)
    }

    /// Get open orders
    ///
    /// API: GET /v5/order/realtime
    /// https://bybit-exchange.github.io/docs/v5/order/open-order
    pub async fn get_open_orders(
        &self,
        category: Category,
        symbol: Option<&str>,
        base_coin: Option<&str>,
        settle_coin: Option<&str>,
        order_id: Option<&str>,
        order_link_id: Option<&str>,
        open_only: Option<i32>,
        order_filter: Option<&str>,
        limit: Option<i32>,
        cursor: Option<&str>,
    ) -> Result<ServerResponse<GetOrdersResponse>> {
        let endpoint = "v5/order/realtime";
        let mut params = json!({
            "category": category,
        });

        if let Some(symbol) = symbol {
            params["symbol"] = json!(symbol);
        }
        if let Some(base_coin) = base_coin {
            params["baseCoin"] = json!(base_coin);
        }
        if let Some(settle_coin) = settle_coin {
            params["settleCoin"] = json!(settle_coin);
        }
        if let Some(order_id) = order_id {
            params["orderId"] = json!(order_id);
        }
        if let Some(order_link_id) = order_link_id {
            params["orderLinkId"] = json!(order_link_id);
        }
        if let Some(open_only) = open_only {
            params["openOnly"] = json!(open_only);
        }
        if let Some(order_filter) = order_filter {
            params["orderFilter"] = json!(order_filter);
        }
        if let Some(limit) = limit {
            params["limit"] = json!(limit);
        }
        if let Some(cursor) = cursor {
            params["cursor"] = json!(cursor);
        }

        let response: ServerResponse<GetOrdersResponse> =
            self.client.get(endpoint, params, SecType::Signed).await?;
        Ok(response)
    }

    /// Get order history
    ///
    /// API: GET /v5/order/history
    /// https://bybit-exchange.github.io/docs/v5/order/order-list
    pub async fn get_order_history(
        &self,
        category: Category,
        symbol: Option<&str>,
        base_coin: Option<&str>,
        settle_coin: Option<&str>,
        order_id: Option<&str>,
        order_link_id: Option<&str>,
        order_filter: Option<&str>,
        order_status: Option<&str>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: Option<i32>,
        cursor: Option<&str>,
    ) -> Result<ServerResponse<GetOrdersResponse>> {
        let endpoint = "v5/order/history";
        let mut params = json!({
            "category": category,
        });

        if let Some(symbol) = symbol {
            params["symbol"] = json!(symbol);
        }
        if let Some(base_coin) = base_coin {
            params["baseCoin"] = json!(base_coin);
        }
        if let Some(settle_coin) = settle_coin {
            params["settleCoin"] = json!(settle_coin);
        }
        if let Some(order_id) = order_id {
            params["orderId"] = json!(order_id);
        }
        if let Some(order_link_id) = order_link_id {
            params["orderLinkId"] = json!(order_link_id);
        }
        if let Some(order_filter) = order_filter {
            params["orderFilter"] = json!(order_filter);
        }
        if let Some(order_status) = order_status {
            params["orderStatus"] = json!(order_status);
        }
        if let Some(start_time) = start_time {
            params["startTime"] = json!(start_time);
        }
        if let Some(end_time) = end_time {
            params["endTime"] = json!(end_time);
        }
        if let Some(limit) = limit {
            params["limit"] = json!(limit);
        }
        if let Some(cursor) = cursor {
            params["cursor"] = json!(cursor);
        }

        let response: ServerResponse<GetOrdersResponse> =
            self.client.get(endpoint, params, SecType::Signed).await?;
        Ok(response)
    }

    /// Get trade history
    ///
    /// API: GET /v5/execution/list
    /// https://bybit-exchange.github.io/docs/v5/order/execution
    pub async fn get_trade_history(
        &self,
        category: Category,
        symbol: Option<&str>,
        order_id: Option<&str>,
        order_link_id: Option<&str>,
        base_coin: Option<&str>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        exec_type: Option<&str>,
        limit: Option<i32>,
        cursor: Option<&str>,
    ) -> Result<ServerResponse<GetTradeHistoryResponse>> {
        let endpoint = "v5/execution/list";
        let mut params = json!({
            "category": category,
        });

        if let Some(symbol) = symbol {
            params["symbol"] = json!(symbol);
        }
        if let Some(order_id) = order_id {
            params["orderId"] = json!(order_id);
        }
        if let Some(order_link_id) = order_link_id {
            params["orderLinkId"] = json!(order_link_id);
        }
        if let Some(base_coin) = base_coin {
            params["baseCoin"] = json!(base_coin);
        }
        if let Some(start_time) = start_time {
            params["startTime"] = json!(start_time);
        }
        if let Some(end_time) = end_time {
            params["endTime"] = json!(end_time);
        }
        if let Some(exec_type) = exec_type {
            params["execType"] = json!(exec_type);
        }
        if let Some(limit) = limit {
            params["limit"] = json!(limit);
        }
        if let Some(cursor) = cursor {
            params["cursor"] = json!(cursor);
        }

        let response: ServerResponse<GetTradeHistoryResponse> =
            self.client.get(endpoint, params, SecType::Signed).await?;
        Ok(response)
    }

    /// Check spot borrow quota
    ///
    /// API: GET /v5/order/spot-borrow-check
    /// https://bybit-exchange.github.io/docs/v5/order/spot-borrow-quota
    pub async fn spot_borrow_check(
        &self,
        category: &str,
        symbol: &str,
        side: &str,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/order/spot-borrow-check";
        let params = json!({
            "category": category,
            "symbol": symbol,
            "side": side,
        });

        let response = self.client.get(endpoint, params, SecType::Signed).await?;
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rest::enums::order_type::OrderType;
    use crate::rest::enums::side::Side;
    use crate::rest::enums::time_in_force::TimeInForce;
    use crate::rest::ApiKeyPair;

    fn create_test_client() -> OrderClient {
        let api_key_pair = ApiKeyPair::new(
            "test".to_string(),
            "test_key".to_string(),
            "test_secret".to_string(),
        );
        let rest_client =
            RestClient::new(api_key_pair, "https://api-testnet.bybit.com".to_string());
        OrderClient::new(rest_client)
    }

    #[test]
    fn test_order_client_creation() {
        let _client = create_test_client();
    }

    #[test]
    fn test_place_order_request_creation() {
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

        assert_eq!(order.symbol, "BTCUSDT");
        assert_eq!(order.qty, "0.001");
    }

    #[test]
    fn test_cancel_order_request_creation() {
        let cancel_request = CancelOrderRequest {
            category: Category::Spot,
            symbol: "BTCUSDT".to_string(),
            order_id: Some("123456".to_string()),
            ..Default::default()
        };

        assert_eq!(cancel_request.symbol, "BTCUSDT");
        assert_eq!(cancel_request.order_id, Some("123456".to_string()));
    }

    #[test]
    fn test_batch_request_creation() {
        let orders = vec![
            PlaceOrderRequest {
                category: Category::Spot,
                symbol: "BTCUSDT".to_string(),
                side: Side::Buy,
                order_type: OrderType::Limit,
                qty: "0.001".to_string(),
                ..Default::default()
            },
            PlaceOrderRequest {
                category: Category::Spot,
                symbol: "ETHUSDT".to_string(),
                side: Side::Buy,
                order_type: OrderType::Limit,
                qty: "0.01".to_string(),
                ..Default::default()
            },
        ];

        assert_eq!(orders.len(), 2);
        assert_eq!(orders[0].symbol, "BTCUSDT");
        assert_eq!(orders[1].symbol, "ETHUSDT");
    }
}
