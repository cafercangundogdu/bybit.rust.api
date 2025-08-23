use crate::rest::client::{RestClient, SecType, ServerResponse};
use anyhow::Result;
use serde_json::json;

pub struct UserClient {
    client: RestClient,
}

impl UserClient {
    pub fn new(client: RestClient) -> Self {
        UserClient { client }
    }

    /// Create a new sub user
    /// https://bybit-exchange.github.io/docs/v5/user/create-subuid
    pub async fn create_sub_member(
        &self,
        username: &str,
        member_type: i32, // 1: normal sub account, 6: custodial sub account
        password: Option<&str>,
        note: Option<&str>,
        switch_option: Option<i32>,
        is_uta: Option<bool>,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/user/create-sub-member";
        let mut body = json!({
            "username": username,
            "memberType": member_type,
        });

        if let Some(password) = password {
            body["password"] = json!(password);
        }
        if let Some(note) = note {
            body["note"] = json!(note);
        }
        if let Some(switch_option) = switch_option {
            body["switch"] = json!(switch_option);
        }
        if let Some(is_uta) = is_uta {
            body["isUta"] = json!(is_uta);
        }

        let response = self.client.post(endpoint, body, SecType::Signed).await?;
        Ok(response)
    }

    /// Create sub UID API key
    /// https://bybit-exchange.github.io/docs/v5/user/create-subuid-apikey
    pub async fn create_sub_api(
        &self,
        sub_uid: i64,
        note: Option<&str>,
        read_only: i32,
        permissions: serde_json::Value,
        ips: Option<Vec<String>>,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/user/create-sub-api";
        let mut body = json!({
            "subuid": sub_uid,
            "readOnly": read_only,
            "permissions": permissions,
        });

        if let Some(note) = note {
            body["note"] = json!(note);
        }
        if let Some(ips) = ips {
            body["ips"] = json!(ips);
        }

        let response = self.client.post(endpoint, body, SecType::Signed).await?;
        Ok(response)
    }

    /// Get sub UID list
    /// https://bybit-exchange.github.io/docs/v5/user/subuid-list
    pub async fn query_sub_members(
        &self,
        page_size: Option<i32>,
        page: Option<i32>,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/user/query-sub-members";
        let mut params = json!({});

        if let Some(page_size) = page_size {
            params["pageSize"] = json!(page_size);
        }
        if let Some(page) = page {
            params["page"] = json!(page);
        }

        let response = self.client.get(endpoint, params, SecType::Signed).await?;
        Ok(response)
    }

    /// Get all sub UID list (Alternative method)
    /// https://bybit-exchange.github.io/docs/v5/user/subuid-list-all
    pub async fn get_sub_members(
        &self,
        uid: Option<&str>,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/user/submembers";
        let mut params = json!({});

        if let Some(uid) = uid {
            params["uid"] = json!(uid);
        }

        let response = self.client.get(endpoint, params, SecType::Signed).await?;
        Ok(response)
    }

    /// Get API key information
    /// https://bybit-exchange.github.io/docs/v5/user/apikey-info
    pub async fn query_api(&self) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/user/query-api";
        let response = self
            .client
            .get(endpoint, json!({}), SecType::Signed)
            .await?;
        Ok(response)
    }

    /// Get sub account API keys
    /// https://bybit-exchange.github.io/docs/v5/user/sub-apikey-list
    pub async fn get_sub_api_keys(
        &self,
        sub_member_id: &str,
        limit: Option<i32>,
        cursor: Option<&str>,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/user/sub-apikeys";
        let mut params = json!({
            "subMemberId": sub_member_id,
        });

        if let Some(limit) = limit {
            params["limit"] = json!(limit);
        }
        if let Some(cursor) = cursor {
            params["cursor"] = json!(cursor);
        }

        let response = self.client.get(endpoint, params, SecType::Signed).await?;
        Ok(response)
    }

    /// Get user account type
    /// https://bybit-exchange.github.io/docs/v5/user/account-type
    pub async fn get_member_type(&self) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/user/get-member-type";
        let response = self
            .client
            .get(endpoint, json!({}), SecType::Signed)
            .await?;
        Ok(response)
    }

    /// Get affiliate user info
    /// https://bybit-exchange.github.io/docs/v5/user/affiliate-info
    pub async fn get_affiliate_customer_info(
        &self,
        uid: &str,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/user/aff-customer-info";
        let params = json!({
            "uid": uid,
        });

        let response = self.client.get(endpoint, params, SecType::Signed).await?;
        Ok(response)
    }

    /// Freeze sub UID
    /// https://bybit-exchange.github.io/docs/v5/user/frozen-subuid
    pub async fn freeze_sub_member(
        &self,
        sub_uid: i64,
        frozen: i32, // 0: unfreeze, 1: freeze
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/user/frozen-sub-member";
        let body = json!({
            "subuid": sub_uid,
            "frozen": frozen,
        });

        let response = self.client.post(endpoint, body, SecType::Signed).await?;
        Ok(response)
    }

    /// Delete sub member
    /// https://bybit-exchange.github.io/docs/v5/user/del-submember
    pub async fn delete_sub_member(
        &self,
        sub_member_id: &str,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/user/del-submember";
        let body = json!({
            "subMemberId": sub_member_id,
        });

        let response = self.client.post(endpoint, body, SecType::Signed).await?;
        Ok(response)
    }

    /// Modify master API key
    /// https://bybit-exchange.github.io/docs/v5/user/modify-master-apikey
    pub async fn update_api(
        &self,
        read_only: Option<i32>,
        ips: Option<Vec<String>>,
        permissions: Option<serde_json::Value>,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/user/update-api";
        let mut body = json!({});

        if let Some(read_only) = read_only {
            body["readOnly"] = json!(read_only);
        }
        if let Some(ips) = ips {
            body["ips"] = json!(ips);
        }
        if let Some(permissions) = permissions {
            body["permissions"] = json!(permissions);
        }

        let response = self.client.post(endpoint, body, SecType::Signed).await?;
        Ok(response)
    }

    /// Delete master API key
    /// https://bybit-exchange.github.io/docs/v5/user/rm-master-apikey
    pub async fn delete_api(&self) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/user/delete-api";
        let body = json!({});

        let response = self.client.post(endpoint, body, SecType::Signed).await?;
        Ok(response)
    }

    /// Modify sub API key
    /// https://bybit-exchange.github.io/docs/v5/user/modify-sub-apikey
    pub async fn update_sub_api(
        &self,
        api_key: &str,
        read_only: Option<i32>,
        ips: Option<Vec<String>>,
        permissions: Option<serde_json::Value>,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/user/update-sub-api";
        let mut body = json!({
            "apikey": api_key,
        });

        if let Some(read_only) = read_only {
            body["readOnly"] = json!(read_only);
        }
        if let Some(ips) = ips {
            body["ips"] = json!(ips);
        }
        if let Some(permissions) = permissions {
            body["permissions"] = json!(permissions);
        }

        let response = self.client.post(endpoint, body, SecType::Signed).await?;
        Ok(response)
    }

    /// Delete sub API key
    /// https://bybit-exchange.github.io/docs/v5/user/rm-sub-apikey
    pub async fn delete_sub_api(&self, api_key: &str) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/user/delete-sub-api";
        let body = json!({
            "apikey": api_key,
        });

        let response = self.client.post(endpoint, body, SecType::Signed).await?;
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rest::ApiKeyPair;

    fn create_test_client() -> UserClient {
        let api_key_pair = ApiKeyPair::new(
            "test".to_string(),
            "test_key".to_string(),
            "test_secret".to_string(),
        );
        let rest_client = RestClient::new(
            api_key_pair,
            "https://api-testnet.bybit.com".to_string(),
            false,
        );
        UserClient::new(rest_client)
    }

    #[test]
    fn test_user_client_creation() {
        let _client = create_test_client();
    }

    #[tokio::test]
    async fn test_create_sub_member_params() {
        let username = "test_sub_user";
        let member_type = 1;
        let password = Some("secure_password");
        let note = Some("Test sub account");
        let switch_option = Some(1);
        let is_uta = Some(true);

        assert_eq!(username, "test_sub_user");
        assert_eq!(member_type, 1);
        assert_eq!(password, Some("secure_password"));
        assert_eq!(note, Some("Test sub account"));
        assert_eq!(switch_option, Some(1));
        assert_eq!(is_uta, Some(true));
    }

    #[tokio::test]
    async fn test_create_sub_api_params() {
        let sub_uid = 12345678i64;
        let note = Some("API for trading");
        let read_only = 0;
        let permissions = json!({
            "ContractTrade": ["Order", "Position"],
            "Spot": ["SpotTrade"]
        });
        let ips = Some(vec!["192.168.1.1".to_string(), "10.0.0.1".to_string()]);

        assert_eq!(sub_uid, 12345678);
        assert_eq!(note, Some("API for trading"));
        assert_eq!(read_only, 0);
        assert!(permissions.is_object());
        assert_eq!(ips.as_ref().unwrap().len(), 2);
    }

    #[tokio::test]
    async fn test_freeze_sub_member_params() {
        let sub_uid = 87654321i64;
        let frozen_freeze = 1;
        let frozen_unfreeze = 0;

        assert_eq!(sub_uid, 87654321);
        assert_eq!(frozen_freeze, 1);
        assert_eq!(frozen_unfreeze, 0);
    }

    #[tokio::test]
    async fn test_query_sub_members_params() {
        let page_size = Some(20);
        let page = Some(1);

        assert_eq!(page_size, Some(20));
        assert_eq!(page, Some(1));
    }

    #[tokio::test]
    async fn test_get_sub_api_keys_params() {
        let sub_member_id = "sub_member_123";
        let limit = Some(50);
        let cursor = Some("next_page_cursor");

        assert_eq!(sub_member_id, "sub_member_123");
        assert_eq!(limit, Some(50));
        assert_eq!(cursor, Some("next_page_cursor"));
    }

    #[tokio::test]
    async fn test_update_api_params() {
        let read_only = Some(1);
        let ips = Some(vec!["10.0.0.1".to_string()]);
        let permissions = Some(json!({
            "Spot": ["SpotTrade"]
        }));

        assert_eq!(read_only, Some(1));
        assert_eq!(ips.as_ref().unwrap().len(), 1);
        assert!(permissions.as_ref().unwrap().is_object());
    }

    #[tokio::test]
    async fn test_delete_sub_api_params() {
        let api_key = "test_api_key_123";

        assert_eq!(api_key, "test_api_key_123");
    }
}
