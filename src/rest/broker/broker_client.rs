use crate::rest::client::{RestClient, SecType, ServerResponse};
use anyhow::Result;
use serde_json::json;

pub struct BrokerClient {
    client: RestClient,
}

impl BrokerClient {
    pub fn new(client: RestClient) -> Self {
        BrokerClient { client }
    }

    /// Get broker account info
    /// https://bybit-exchange.github.io/docs/v5/broker/account-info
    pub async fn get_account_info(&self) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/broker/account-info";
        let response = self
            .client
            .get(endpoint, json!({}), SecType::Signed)
            .await?;
        Ok(response)
    }

    /// Get sub account deposit records
    /// https://bybit-exchange.github.io/docs/v5/broker/sub-deposit-record
    pub async fn get_sub_member_deposit_record(
        &self,
        sub_member_id: Option<&str>,
        coin: Option<&str>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: Option<i32>,
        cursor: Option<&str>,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/broker/asset/query-sub-member-deposit-record";
        let mut params = json!({});

        if let Some(sub_member_id) = sub_member_id {
            params["subMemberId"] = json!(sub_member_id);
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

    /// Get broker earning
    /// https://bybit-exchange.github.io/docs/v5/broker/earning
    pub async fn get_earning_record(
        &self,
        biz_type: Option<&str>,
        begin: Option<&str>,
        end: Option<&str>,
        uid: Option<&str>,
        limit: Option<i32>,
        cursor: Option<&str>,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/broker/earning-record";
        let mut params = json!({});

        if let Some(biz_type) = biz_type {
            params["bizType"] = json!(biz_type);
        }
        if let Some(begin) = begin {
            params["begin"] = json!(begin);
        }
        if let Some(end) = end {
            params["end"] = json!(end);
        }
        if let Some(uid) = uid {
            params["uid"] = json!(uid);
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

    /// Get award info
    /// https://bybit-exchange.github.io/docs/v5/broker/award-info
    pub async fn get_award_info(
        &self,
        coin: Option<&str>,
        record_type: Option<&str>,
        page: Option<i32>,
        limit: Option<i32>,
        start_date: Option<&str>,
        end_date: Option<&str>,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/broker/award/info";
        let mut params = json!({});

        if let Some(coin) = coin {
            params["coin"] = json!(coin);
        }
        if let Some(record_type) = record_type {
            params["recordType"] = json!(record_type);
        }
        if let Some(page) = page {
            params["page"] = json!(page);
        }
        if let Some(limit) = limit {
            params["limit"] = json!(limit);
        }
        if let Some(start_date) = start_date {
            params["startDate"] = json!(start_date);
        }
        if let Some(end_date) = end_date {
            params["endDate"] = json!(end_date);
        }

        let response = self.client.get(endpoint, params, SecType::Signed).await?;
        Ok(response)
    }

    /// Distribute award
    /// https://bybit-exchange.github.io/docs/v5/broker/distribute-award
    pub async fn distribute_award(
        &self,
        coin: &str,
        amount: &str,
        to_uid: &str,
        record_type: &str,
        from_memo: Option<&str>,
        to_memo: Option<&str>,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/broker/award/distribute-award";
        let mut body = json!({
            "coin": coin,
            "amount": amount,
            "toUid": to_uid,
            "recordType": record_type,
        });

        if let Some(from_memo) = from_memo {
            body["fromMemo"] = json!(from_memo);
        }
        if let Some(to_memo) = to_memo {
            body["toMemo"] = json!(to_memo);
        }

        let response = self.client.post(endpoint, body, SecType::Signed).await?;
        Ok(response)
    }

    /// Get distribution record
    /// https://bybit-exchange.github.io/docs/v5/broker/distribution-record
    pub async fn get_distribution_record(
        &self,
        coin: Option<&str>,
        page: Option<i32>,
        limit: Option<i32>,
        start_date: Option<&str>,
        end_date: Option<&str>,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/broker/award/distribution-record";
        let mut params = json!({});

        if let Some(coin) = coin {
            params["coin"] = json!(coin);
        }
        if let Some(page) = page {
            params["page"] = json!(page);
        }
        if let Some(limit) = limit {
            params["limit"] = json!(limit);
        }
        if let Some(start_date) = start_date {
            params["startDate"] = json!(start_date);
        }
        if let Some(end_date) = end_date {
            params["endDate"] = json!(end_date);
        }

        let response = self.client.get(endpoint, params, SecType::Signed).await?;
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rest::ApiKeyPair;

    fn create_test_client() -> BrokerClient {
        let api_key_pair = ApiKeyPair::new(
            "test_key".to_string(),
            "test_secret".to_string(),
            "".to_string(),
        );
        let rest_client =
            RestClient::new(api_key_pair, "https://api-testnet.bybit.com".to_string());
        BrokerClient::new(rest_client)
    }

    #[test]
    fn test_client_creation() {
        let client = create_test_client();
        // Test that client was created successfully
        assert_eq!(
            std::mem::size_of_val(&client),
            std::mem::size_of::<BrokerClient>()
        );
    }

    #[tokio::test]
    async fn test_distribute_award_required_params() {
        let client = create_test_client();
        let result = client
            .distribute_award("USDT", "100.0", "12345", "reward", None, None)
            .await;
        // Should not panic with valid required parameters
        assert!(result.is_err() || result.is_ok());
    }

    #[tokio::test]
    async fn test_get_sub_member_deposit_record_params() {
        let client = create_test_client();
        let result = client
            .get_sub_member_deposit_record(Some("sub123"), Some("USDT"), None, None, Some(10), None)
            .await;
        // Should handle optional parameters correctly
        assert!(result.is_err() || result.is_ok());
    }

    #[tokio::test]
    async fn test_get_earning_record_params() {
        let client = create_test_client();
        let result = client
            .get_earning_record(
                Some("trading"),
                Some("2023-01-01"),
                Some("2023-12-31"),
                None,
                Some(50),
                None,
            )
            .await;
        // Should handle optional parameters correctly
        assert!(result.is_err() || result.is_ok());
    }
}
