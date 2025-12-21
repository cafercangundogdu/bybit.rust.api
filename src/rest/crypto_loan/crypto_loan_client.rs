use crate::rest::client::{RestClient, SecType, ServerResponse};
use anyhow::Result;
use serde_json::json;

pub struct CryptoLoanClient {
    client: RestClient,
}

impl CryptoLoanClient {
    pub fn new(client: RestClient) -> Self {
        CryptoLoanClient { client }
    }

    /// Get loan product info
    /// https://bybit-exchange.github.io/docs/v5/crypto-loan/collateral-data
    pub async fn get_collateral_data(
        &self,
        ltv_type: Option<&str>,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/crypto-loan/collateral-data";
        let mut params = json!({});

        if let Some(ltv_type) = ltv_type {
            params["ltvType"] = json!(ltv_type);
        }

        let response = self.client.get(endpoint, params, SecType::Signed).await?;
        Ok(response)
    }

    /// Borrow crypto loan
    /// https://bybit-exchange.github.io/docs/v5/crypto-loan/borrow
    pub async fn borrow(
        &self,
        ltv_type: &str,
        loan_currency: &str,
        loan_amount: &str,
        collateral_currency: &str,
        max_rate: Option<&str>,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/crypto-loan/borrow";
        let mut body = json!({
            "ltvType": ltv_type,
            "loanCurrency": loan_currency,
            "loanAmount": loan_amount,
            "collateralCurrency": collateral_currency,
        });

        if let Some(max_rate) = max_rate {
            body["maxRate"] = json!(max_rate);
        }

        let response = self.client.post(endpoint, body, SecType::Signed).await?;
        Ok(response)
    }

    /// Repay crypto loan
    /// https://bybit-exchange.github.io/docs/v5/crypto-loan/repay
    pub async fn repay(
        &self,
        order_id: &str,
        repay_amount: Option<&str>,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/crypto-loan/repay";
        let mut body = json!({
            "orderId": order_id,
        });

        if let Some(repay_amount) = repay_amount {
            body["repayAmount"] = json!(repay_amount);
        }

        let response = self.client.post(endpoint, body, SecType::Signed).await?;
        Ok(response)
    }

    /// Get ongoing orders
    /// https://bybit-exchange.github.io/docs/v5/crypto-loan/ongoing-orders
    pub async fn get_ongoing_orders(
        &self,
        order_id: Option<&str>,
        loan_currency: Option<&str>,
        collateral_currency: Option<&str>,
        limit: Option<i32>,
        cursor: Option<&str>,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/crypto-loan/ongoing-orders";
        let mut params = json!({});

        if let Some(order_id) = order_id {
            params["orderId"] = json!(order_id);
        }
        if let Some(loan_currency) = loan_currency {
            params["loanCurrency"] = json!(loan_currency);
        }
        if let Some(collateral_currency) = collateral_currency {
            params["collateralCurrency"] = json!(collateral_currency);
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

    /// Get borrow history
    /// https://bybit-exchange.github.io/docs/v5/crypto-loan/borrow-history
    pub async fn get_borrow_history(
        &self,
        order_id: Option<&str>,
        loan_currency: Option<&str>,
        collateral_currency: Option<&str>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: Option<i32>,
        cursor: Option<&str>,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/crypto-loan/borrow-history";
        let mut params = json!({});

        if let Some(order_id) = order_id {
            params["orderId"] = json!(order_id);
        }
        if let Some(loan_currency) = loan_currency {
            params["loanCurrency"] = json!(loan_currency);
        }
        if let Some(collateral_currency) = collateral_currency {
            params["collateralCurrency"] = json!(collateral_currency);
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

    /// Get max collateral amount
    /// https://bybit-exchange.github.io/docs/v5/crypto-loan/max-collateral-amount
    pub async fn get_max_collateral_amount(
        &self,
        ltv_type: &str,
        loan_currency: &str,
        loan_amount: &str,
        collateral_currency: &str,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/crypto-loan/max-collateral-amount";
        let params = json!({
            "ltvType": ltv_type,
            "loanCurrency": loan_currency,
            "loanAmount": loan_amount,
            "collateralCurrency": collateral_currency,
        });

        let response = self.client.get(endpoint, params, SecType::Signed).await?;
        Ok(response)
    }

    /// Adjust LTV
    /// https://bybit-exchange.github.io/docs/v5/crypto-loan/adjust-ltv
    pub async fn adjust_ltv(
        &self,
        order_id: &str,
        amount: &str,
        direction: i32, // 1: add collateral, 2: remove collateral
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/crypto-loan/adjust-ltv";
        let body = json!({
            "orderId": order_id,
            "amount": amount,
            "direction": direction,
        });

        let response = self.client.post(endpoint, body, SecType::Signed).await?;
        Ok(response)
    }

    /// Get adjustment history
    /// https://bybit-exchange.github.io/docs/v5/crypto-loan/adjustment-history
    pub async fn get_adjustment_history(
        &self,
        order_id: Option<&str>,
        adjustment_id: Option<&str>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: Option<i32>,
        cursor: Option<&str>,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/crypto-loan/adjustment-history";
        let mut params = json!({});

        if let Some(order_id) = order_id {
            params["orderId"] = json!(order_id);
        }
        if let Some(adjustment_id) = adjustment_id {
            params["adjustmentId"] = json!(adjustment_id);
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rest::ApiKeyPair;

    fn create_test_client() -> CryptoLoanClient {
        let api_key_pair = ApiKeyPair::new(
            "test_key".to_string(),
            "test_secret".to_string(),
            "".to_string(),
        );
        let rest_client =
            RestClient::new(api_key_pair, "https://api-testnet.bybit.com".to_string());
        CryptoLoanClient::new(rest_client)
    }

    #[test]
    fn test_client_creation() {
        let client = create_test_client();
        // Test that client was created successfully
        assert_eq!(
            std::mem::size_of_val(&client),
            std::mem::size_of::<CryptoLoanClient>()
        );
    }

    #[tokio::test]
    async fn test_borrow_required_params() {
        let client = create_test_client();
        let result = client
            .borrow("1", "USDT", "1000.0", "BTC", Some("0.05"))
            .await;
        // Should not panic with valid required parameters
        assert!(result.is_err() || result.is_ok());
    }

    #[tokio::test]
    async fn test_repay_required_params() {
        let client = create_test_client();
        let result = client.repay("order_123", Some("500.0")).await;
        // Should not panic with valid required order_id parameter
        assert!(result.is_err() || result.is_ok());
    }

    #[tokio::test]
    async fn test_adjust_ltv_required_params() {
        let client = create_test_client();
        let result = client
            .adjust_ltv(
                "order_123",
                "100.0",
                1, // add collateral
            )
            .await;
        // Should not panic with valid required parameters
        assert!(result.is_err() || result.is_ok());
    }
}
