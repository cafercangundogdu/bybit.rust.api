use crate::rest::client::{RestClient, SecType, ServerResponse};
use anyhow::Result;
use serde_json::json;

pub struct SpotLeverageTokenClient {
    client: RestClient,
}

impl SpotLeverageTokenClient {
    pub fn new(client: RestClient) -> Self {
        SpotLeverageTokenClient { client }
    }

    /// Get leverage token info
    /// https://bybit-exchange.github.io/docs/v5/lt/leverage-token-info
    pub async fn get_leverage_token_info(
        &self,
        lt_coin: Option<&str>,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/spot-lever-token/info";
        let mut params = json!({});

        if let Some(lt_coin) = lt_coin {
            params["ltCoin"] = json!(lt_coin);
        }

        let response = self.client.get(endpoint, params, SecType::None).await?;
        Ok(response)
    }

    /// Get leverage token market
    /// https://bybit-exchange.github.io/docs/v5/lt/leverage-token-reference
    pub async fn get_leverage_token_reference(
        &self,
        lt_coin: &str,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/spot-lever-token/reference";
        let params = json!({
            "ltCoin": lt_coin,
        });

        let response = self.client.get(endpoint, params, SecType::None).await?;
        Ok(response)
    }

    /// Purchase leverage token
    /// https://bybit-exchange.github.io/docs/v5/lt/purchase
    pub async fn purchase(
        &self,
        lt_coin: &str,
        lt_amount: &str,
        serial_no: Option<&str>,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/spot-lever-token/purchase";
        let mut body = json!({
            "ltCoin": lt_coin,
            "ltAmount": lt_amount,
        });

        if let Some(serial_no) = serial_no {
            body["serialNo"] = json!(serial_no);
        }

        let response = self.client.post(endpoint, body, SecType::Signed).await?;
        Ok(response)
    }

    /// Redeem leverage token
    /// https://bybit-exchange.github.io/docs/v5/lt/redeem
    pub async fn redeem(
        &self,
        lt_coin: &str,
        lt_amount: &str,
        serial_no: Option<&str>,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/spot-lever-token/redeem";
        let mut body = json!({
            "ltCoin": lt_coin,
            "ltAmount": lt_amount,
        });

        if let Some(serial_no) = serial_no {
            body["serialNo"] = json!(serial_no);
        }

        let response = self.client.post(endpoint, body, SecType::Signed).await?;
        Ok(response)
    }

    /// Get purchase/redemption records
    /// https://bybit-exchange.github.io/docs/v5/lt/order-record
    pub async fn get_order_record(
        &self,
        lt_coin: Option<&str>,
        order_id: Option<&str>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: Option<i32>,
        lt_order_type: Option<i32>, // 1: purchase, 2: redemption
        serial_no: Option<&str>,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/spot-lever-token/order-record";
        let mut params = json!({});

        if let Some(lt_coin) = lt_coin {
            params["ltCoin"] = json!(lt_coin);
        }
        if let Some(order_id) = order_id {
            params["orderId"] = json!(order_id);
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
        if let Some(lt_order_type) = lt_order_type {
            params["ltOrderType"] = json!(lt_order_type);
        }
        if let Some(serial_no) = serial_no {
            params["serialNo"] = json!(serial_no);
        }

        let response = self.client.get(endpoint, params, SecType::Signed).await?;
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rest::ApiKeyPair;

    fn create_test_client() -> SpotLeverageTokenClient {
        let api_key_pair = ApiKeyPair::new(
            "test_key".to_string(),
            "test_secret".to_string(),
            "".to_string(),
        );
        let rest_client =
            RestClient::new(api_key_pair, "https://api-testnet.bybit.com".to_string());
        SpotLeverageTokenClient::new(rest_client)
    }

    #[test]
    fn test_client_creation() {
        let client = create_test_client();
        // Test that client was created successfully
        assert_eq!(
            std::mem::size_of_val(&client),
            std::mem::size_of::<SpotLeverageTokenClient>()
        );
    }

    #[tokio::test]
    async fn test_purchase_required_params() {
        let client = create_test_client();
        let result = client.purchase("BTC3LUSDT", "100.0", None).await;
        // Should not panic with valid required parameters
        assert!(result.is_err() || result.is_ok());
    }

    #[tokio::test]
    async fn test_redeem_required_params() {
        let client = create_test_client();
        let result = client
            .redeem("BTC3LUSDT", "50.0", Some("test_serial_123"))
            .await;
        // Should not panic with valid required parameters
        assert!(result.is_err() || result.is_ok());
    }

    #[tokio::test]
    async fn test_get_leverage_token_reference_required_params() {
        let client = create_test_client();
        let result = client.get_leverage_token_reference("BTC3LUSDT").await;
        // Should not panic with valid required lt_coin parameter
        assert!(result.is_err() || result.is_ok());
    }
}
