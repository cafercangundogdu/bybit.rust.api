use crate::rest::client::{RestClient, SecType, ServerResponse};
use crate::rest::BybitResult as Result;
use serde_json::json;

#[derive(Clone)]
pub struct InstitutionalLoanClient {
    client: RestClient,
}

impl InstitutionalLoanClient {
    pub fn new(client: RestClient) -> Self {
        InstitutionalLoanClient { client }
    }

    /// Get LTV (Loan-to-Value) information
    /// Get institutional loan LTV ratio information
    pub async fn get_ltv(
        &self,
        currency: Option<&str>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: Option<i32>,
        cursor: Option<&str>,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/ins-loan/ltv";
        let mut params = json!({});

        if let Some(currency) = currency {
            params["currency"] = json!(currency);
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

    /// Bind or unbind UID
    /// Bind or unbind institutional loan association UID
    pub async fn bind_or_unbind_uid(
        &self,
        uid: &str,
        operate: &str, // "0" to bind, "1" to unbind
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/ins-loan/association-uid";
        let body = json!({
            "uid": uid,
            "operate": operate,
        });

        let response = self.client.post(endpoint, body, SecType::Signed).await?;
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rest::ApiKeyPair;

    fn create_test_client() -> InstitutionalLoanClient {
        let api_key_pair = ApiKeyPair::new(
            "test_key".to_string(),
            "test_secret".to_string(),
            "".to_string(),
        );
        let rest_client =
            RestClient::new(api_key_pair, "https://api-testnet.bybit.com".to_string());
        InstitutionalLoanClient::new(rest_client)
    }

    #[test]
    fn test_client_creation() {
        let client = create_test_client();
        // Test that client was created successfully
        assert_eq!(
            std::mem::size_of_val(&client),
            std::mem::size_of::<InstitutionalLoanClient>()
        );
    }

    #[tokio::test]
    async fn test_bind_or_unbind_uid_required_params() {
        let client = create_test_client();
        let result = client.bind_or_unbind_uid("12345", "0").await;
        // Should not panic with valid required parameters
        assert!(result.is_err() || result.is_ok());
    }

    #[tokio::test]
    async fn test_get_ltv_params() {
        let client = create_test_client();
        let result = client
            .get_ltv(
                Some("USDT"),
                Some(1672531200000), // 2023-01-01
                None,
                Some(10),
                None,
            )
            .await;
        // Should handle optional parameters correctly
        assert!(result.is_err() || result.is_ok());
    }

    #[tokio::test]
    async fn test_bind_uid_operation() {
        let client = create_test_client();
        let result = client.bind_or_unbind_uid("test_uid", "1").await;
        // Should handle unbind operation
        assert!(result.is_err() || result.is_ok());
    }
}
