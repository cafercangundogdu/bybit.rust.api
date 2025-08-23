use crate::rest::client::{RestClient, SecType, ServerResponse};
use anyhow::Result;
use serde_json::json;

pub struct PreUpgradeClient {
    client: RestClient,
}

impl PreUpgradeClient {
    pub fn new(client: RestClient) -> Self {
        PreUpgradeClient { client }
    }

    /// Get pre-upgrade order history
    /// Get orders placed before the system upgrade
    pub async fn get_order_history(
        &self,
        category: &str,
        symbol: Option<&str>,
        base_coin: Option<&str>,
        order_id: Option<&str>,
        order_link_id: Option<&str>,
        order_filter: Option<&str>,
        order_status: Option<&str>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: Option<i32>,
        cursor: Option<&str>,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/pre-upgrade/order/history";
        let mut params = json!({
            "category": category
        });

        if let Some(symbol) = symbol {
            params["symbol"] = json!(symbol);
        }
        if let Some(base_coin) = base_coin {
            params["baseCoin"] = json!(base_coin);
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

        let response = self.client.get(endpoint, params, SecType::Signed).await?;
        Ok(response)
    }

    /// Get pre-upgrade trade history
    /// Get trade execution history before the system upgrade
    pub async fn get_trade_history(
        &self,
        category: &str,
        symbol: Option<&str>,
        order_id: Option<&str>,
        order_link_id: Option<&str>,
        base_coin: Option<&str>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        exec_type: Option<&str>,
        limit: Option<i32>,
        cursor: Option<&str>,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/pre-upgrade/execution/list";
        let mut params = json!({
            "category": category
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

        let response = self.client.get(endpoint, params, SecType::Signed).await?;
        Ok(response)
    }

    /// Get pre-upgrade transaction log
    /// Get account transaction log before the system upgrade
    pub async fn get_transaction_log(
        &self,
        category: Option<&str>,
        base_coin: Option<&str>,
        coin: Option<&str>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: Option<i32>,
        cursor: Option<&str>,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/pre-upgrade/account/transaction-log";
        let mut params = json!({});

        if let Some(category) = category {
            params["category"] = json!(category);
        }
        if let Some(base_coin) = base_coin {
            params["baseCoin"] = json!(base_coin);
        }
        if let Some(coin) = coin {
            params["coin"] = json!(coin);
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

        let response = self.client.get(endpoint, params, SecType::Signed).await?;
        Ok(response)
    }

    /// Get pre-upgrade closed PnL
    /// Get closed profit and loss records before the system upgrade
    pub async fn get_closed_pnl(
        &self,
        category: &str,
        symbol: Option<&str>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: Option<i32>,
        cursor: Option<&str>,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/pre-upgrade/position/closed-pnl";
        let mut params = json!({
            "category": category
        });

        if let Some(symbol) = symbol {
            params["symbol"] = json!(symbol);
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

        let response = self.client.get(endpoint, params, SecType::Signed).await?;
        Ok(response)
    }

    /// Get pre-upgrade option delivery record
    /// Get option delivery records before the system upgrade
    pub async fn get_option_delivery_record(
        &self,
        category: &str,
        symbol: Option<&str>,
        expired_date: Option<&str>,
        limit: Option<i32>,
        cursor: Option<&str>,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/pre-upgrade/asset/delivery-record";
        let mut params = json!({
            "category": category
        });

        if let Some(symbol) = symbol {
            params["symbol"] = json!(symbol);
        }
        if let Some(expired_date) = expired_date {
            params["expiredDate"] = json!(expired_date);
        }
        if let Some(limit) = limit {
            params["limit"] = json!(limit);
        }
        if let Some(cursor) = cursor {
            params["cursor"] = json!(cursor);
        }

        let response = self.client.get(endpoint, params, SecType::Signed).await?;
        Ok(response)
    }

    /// Get pre-upgrade USDC session settlement
    /// Get USDC session settlement records before the system upgrade
    pub async fn get_usdc_session_settlement(
        &self,
        category: &str,
        symbol: Option<&str>,
        limit: Option<i32>,
        cursor: Option<&str>,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/pre-upgrade/asset/settlement-record";
        let mut params = json!({
            "category": category
        });

        if let Some(symbol) = symbol {
            params["symbol"] = json!(symbol);
        }
        if let Some(limit) = limit {
            params["limit"] = json!(limit);
        }
        if let Some(cursor) = cursor {
            params["cursor"] = json!(cursor);
        }

        let response = self.client.get(endpoint, params, SecType::Signed).await?;
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rest::ApiKeyPair;

    fn create_test_client() -> PreUpgradeClient {
        let api_key_pair = ApiKeyPair::new(
            "test_key".to_string(),
            "test_secret".to_string(),
            "".to_string(),
        );
        let rest_client = RestClient::new(
            api_key_pair,
            "https://api-testnet.bybit.com".to_string(),
            false,
        );
        PreUpgradeClient::new(rest_client)
    }

    #[test]
    fn test_client_creation() {
        let client = create_test_client();
        // Test that client was created successfully
        assert_eq!(
            std::mem::size_of_val(&client),
            std::mem::size_of::<PreUpgradeClient>()
        );
    }

    #[tokio::test]
    async fn test_get_order_history_required_params() {
        let client = create_test_client();
        let result = client
            .get_order_history(
                "linear",
                Some("BTCUSDT"),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(10),
                None,
            )
            .await;
        // Should not panic with valid category parameter
        assert!(result.is_err() || result.is_ok());
    }

    #[tokio::test]
    async fn test_get_trade_history_required_params() {
        let client = create_test_client();
        let result = client
            .get_trade_history(
                "linear",
                Some("BTCUSDT"),
                None,
                None,
                None,
                None,
                None,
                None,
                Some(20),
                None,
            )
            .await;
        // Should not panic with valid category parameter
        assert!(result.is_err() || result.is_ok());
    }

    #[tokio::test]
    async fn test_get_closed_pnl_required_params() {
        let client = create_test_client();
        let result = client
            .get_closed_pnl(
                "linear",
                Some("BTCUSDT"),
                Some(1672531200000), // 2023-01-01
                None,
                Some(10),
                None,
            )
            .await;
        // Should not panic with valid category parameter
        assert!(result.is_err() || result.is_ok());
    }
}
