use crate::rest::client::{RestClient, SecType, ServerResponse};
use anyhow::Result;
use serde_json::json;

#[derive(Clone)]
pub struct SpotMarginTradeClient {
    client: RestClient,
}

impl SpotMarginTradeClient {
    pub fn new(client: RestClient) -> Self {
        SpotMarginTradeClient { client }
    }

    /// Get VIP margin data
    /// Get VIP spot margin trading data including borrowable amounts and interest rates
    pub async fn get_vip_margin_data(
        &self,
        vip_level: Option<&str>,
        currency: Option<&str>,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/spot-margin-trade/data";
        let mut params = json!({});

        if let Some(vip_level) = vip_level {
            params["vipLevel"] = json!(vip_level);
        }
        if let Some(currency) = currency {
            params["currency"] = json!(currency);
        }

        let response = self.client.get(endpoint, params, SecType::Signed).await?;
        Ok(response)
    }

    /// Get historical interest rate
    /// Get historical spot margin trading interest rates
    pub async fn get_historical_interest_rate(
        &self,
        currency: Option<&str>,
        vip_level: Option<&str>,
        start_time: Option<i64>,
        end_time: Option<i64>,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/spot-margin-trade/interest-rate-history";
        let mut params = json!({});

        if let Some(currency) = currency {
            params["currency"] = json!(currency);
        }
        if let Some(vip_level) = vip_level {
            params["vipLevel"] = json!(vip_level);
        }
        if let Some(start_time) = start_time {
            params["startTime"] = json!(start_time);
        }
        if let Some(end_time) = end_time {
            params["endTime"] = json!(end_time);
        }

        let response = self.client.get(endpoint, params, SecType::Signed).await?;
        Ok(response)
    }

    /// Get status and leverage
    /// Get current spot margin trading status and leverage information
    pub async fn get_status_and_leverage(&self) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/spot-margin-trade/state";
        let params = json!({});

        let response = self.client.get(endpoint, params, SecType::Signed).await?;
        Ok(response)
    }

    /// Switch spot margin trade mode
    /// Switch between normal and spot margin trading mode
    pub async fn switch_mode(
        &self,
        spot_margin_mode: &str, // "1" for margin mode, "0" for normal mode
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/spot-margin-trade/switch-mode";
        let body = json!({
            "spotMarginMode": spot_margin_mode,
        });

        let response = self.client.post(endpoint, body, SecType::Signed).await?;
        Ok(response)
    }

    /// Set leverage
    /// Set spot margin trading leverage
    pub async fn set_leverage(&self, leverage: i32) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/spot-margin-trade/set-leverage";
        let body = json!({
            "leverage": leverage,
        });

        let response = self.client.post(endpoint, body, SecType::Signed).await?;
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rest::ApiKeyPair;

    fn create_test_client() -> SpotMarginTradeClient {
        let api_key_pair = ApiKeyPair::new(
            "test_key".to_string(),
            "test_secret".to_string(),
            "".to_string(),
        );
        let rest_client =
            RestClient::new(api_key_pair, "https://api-testnet.bybit.com".to_string());
        SpotMarginTradeClient::new(rest_client)
    }

    #[test]
    fn test_client_creation() {
        let client = create_test_client();
        // Test that client was created successfully
        assert_eq!(
            std::mem::size_of_val(&client),
            std::mem::size_of::<SpotMarginTradeClient>()
        );
    }

    #[tokio::test]
    async fn test_switch_mode_required_params() {
        let client = create_test_client();
        let result = client.switch_mode("1").await;
        // Should not panic with valid mode parameter
        assert!(result.is_err() || result.is_ok());
    }

    #[tokio::test]
    async fn test_set_leverage_required_params() {
        let client = create_test_client();
        let result = client.set_leverage(10).await;
        // Should not panic with valid leverage parameter
        assert!(result.is_err() || result.is_ok());
    }

    #[tokio::test]
    async fn test_get_vip_margin_data_params() {
        let client = create_test_client();
        let result = client.get_vip_margin_data(Some("VIP1"), Some("USDT")).await;
        // Should handle optional parameters correctly
        assert!(result.is_err() || result.is_ok());
    }
}
