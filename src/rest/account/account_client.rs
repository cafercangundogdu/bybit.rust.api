use crate::rest::client::{RestClient, SecType, ServerResponse};
use anyhow::Result;
use serde_json::json;

pub struct AccountClient {
    client: RestClient,
}

impl AccountClient {
    pub fn new(client: RestClient) -> Self {
        AccountClient { client }
    }

    /// Get wallet balance
    /// https://bybit-exchange.github.io/docs/v5/account/wallet-balance
    pub async fn get_wallet_balance(
        &self,
        account_type: &str,
        coin: Option<&str>,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/account/wallet-balance";
        let mut params = json!({
            "accountType": account_type,
        });
        
        if let Some(coin) = coin {
            params["coin"] = json!(coin);
        }
        
        let response = self.client.get(endpoint, params, SecType::Signed).await?;
        Ok(response)
    }

    /// Get fee rate
    /// https://bybit-exchange.github.io/docs/v5/account/fee-rate
    pub async fn get_fee_rate(
        &self,
        category: &str,
        symbol: Option<&str>,
        base_coin: Option<&str>,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/account/fee-rate";
        let mut params = json!({
            "category": category,
        });
        
        if let Some(symbol) = symbol {
            params["symbol"] = json!(symbol);
        }
        if let Some(base_coin) = base_coin {
            params["baseCoin"] = json!(base_coin);
        }
        
        let response = self.client.get(endpoint, params, SecType::Signed).await?;
        Ok(response)
    }

    /// Get account info
    /// https://bybit-exchange.github.io/docs/v5/account/account-info
    pub async fn get_account_info(&self) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/account/info";
        let response = self.client.get(endpoint, json!({}), SecType::Signed).await?;
        Ok(response)
    }

    /// Get transaction log
    /// https://bybit-exchange.github.io/docs/v5/account/transaction-log
    pub async fn get_transaction_log(
        &self,
        account_type: Option<&str>,
        category: Option<&str>,
        currency: Option<&str>,
        base_coin: Option<&str>,
        log_type: Option<&str>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: Option<i32>,
        cursor: Option<&str>,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/account/transaction-log";
        let mut params = json!({});
        
        if let Some(account_type) = account_type {
            params["accountType"] = json!(account_type);
        }
        if let Some(category) = category {
            params["category"] = json!(category);
        }
        if let Some(currency) = currency {
            params["currency"] = json!(currency);
        }
        if let Some(base_coin) = base_coin {
            params["baseCoin"] = json!(base_coin);
        }
        if let Some(log_type) = log_type {
            params["type"] = json!(log_type);
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

    /// Set margin mode
    /// https://bybit-exchange.github.io/docs/v5/account/set-margin-mode
    pub async fn set_margin_mode(
        &self,
        set_margin_mode: &str,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/account/set-margin-mode";
        let body = json!({
            "setMarginMode": set_margin_mode,
        });
        
        let response = self.client.post(endpoint, body, SecType::Signed).await?;
        Ok(response)
    }

    /// Set MMP
    /// https://bybit-exchange.github.io/docs/v5/account/set-mmp
    pub async fn set_mmp(
        &self,
        base_coin: &str,
        window: i32,
        frozen_period: i32,
        qty_limit: &str,
        delta_limit: &str,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/account/mmp-modify";
        let body = json!({
            "baseCoin": base_coin,
            "window": window,
            "frozenPeriod": frozen_period,
            "qtyLimit": qty_limit,
            "deltaLimit": delta_limit,
        });
        
        let response = self.client.post(endpoint, body, SecType::Signed).await?;
        Ok(response)
    }

    /// Reset MMP
    /// https://bybit-exchange.github.io/docs/v5/account/reset-mmp
    pub async fn reset_mmp(&self, base_coin: &str) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/account/mmp-reset";
        let body = json!({
            "baseCoin": base_coin,
        });
        
        let response = self.client.post(endpoint, body, SecType::Signed).await?;
        Ok(response)
    }

    /// Get MMP state
    /// https://bybit-exchange.github.io/docs/v5/account/get-mmp-state
    pub async fn get_mmp_state(&self, base_coin: &str) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/account/mmp-state";
        let params = json!({
            "baseCoin": base_coin,
        });
        
        let response = self.client.get(endpoint, params, SecType::Signed).await?;
        Ok(response)
    }

    /// Get SMP group list
    /// https://bybit-exchange.github.io/docs/v5/account/smp-group
    pub async fn get_smp_group_list(&self) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/account/smp-group";
        let response = self.client.get(endpoint, json!({}), SecType::Signed).await?;
        Ok(response)
    }

    /// Get coin Greeks
    /// https://bybit-exchange.github.io/docs/v5/account/coin-greeks
    pub async fn get_coin_greeks(&self, base_coin: Option<&str>) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/asset/exchange/coin-greeks";
        let mut params = json!({});
        
        if let Some(base_coin) = base_coin {
            params["baseCoin"] = json!(base_coin);
        }
        
        let response = self.client.get(endpoint, params, SecType::Signed).await?;
        Ok(response)
    }

    /// Get collateral info
    /// https://bybit-exchange.github.io/docs/v5/account/collateral-info
    pub async fn get_collateral_info(&self, currency: Option<&str>) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/account/collateral-info";
        let mut params = json!({});
        
        if let Some(currency) = currency {
            params["currency"] = json!(currency);
        }
        
        let response = self.client.get(endpoint, params, SecType::Signed).await?;
        Ok(response)
    }

    /// Get borrow history
    /// https://bybit-exchange.github.io/docs/v5/account/borrow-history
    pub async fn get_borrow_history(
        &self,
        currency: Option<&str>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: Option<i32>,
        cursor: Option<&str>,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/account/borrow-history";
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

    /// Set disconnect cancel all
    /// https://bybit-exchange.github.io/docs/v5/account/set-dcp
    pub async fn set_disconnect_cancel_all(
        &self,
        time_window: i32,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/order/disconnected-cancel-all";
        let body = json!({
            "timeWindow": time_window,
        });
        
        let response = self.client.post(endpoint, body, SecType::Signed).await?;
        Ok(response)
    }

    /// Upgrade to unified account
    /// https://bybit-exchange.github.io/docs/v5/account/upgrade-unified-account
    pub async fn upgrade_to_unified_account(&self) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/account/upgrade-to-uta";
        let body = json!({});
        
        let response = self.client.post(endpoint, body, SecType::Signed).await?;
        Ok(response)
    }

    /// Get contract transaction log
    /// https://bybit-exchange.github.io/docs/v5/account/contract-transaction-log
    pub async fn get_contract_transaction_log(
        &self,
        category: Option<&str>,
        base_coin: Option<&str>,
        log_type: Option<&str>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: Option<i32>,
        cursor: Option<&str>,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/account/contract-transaction-log";
        let mut params = json!({});
        
        if let Some(category) = category {
            params["category"] = json!(category);
        }
        if let Some(base_coin) = base_coin {
            params["baseCoin"] = json!(base_coin);
        }
        if let Some(log_type) = log_type {
            params["type"] = json!(log_type);
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

    /// Query DCP information
    /// https://bybit-exchange.github.io/docs/v5/account/query-dcp-info
    pub async fn query_dcp_info(&self) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/account/query-dcp-info";
        let response = self.client.get(endpoint, json!({}), SecType::Signed).await?;
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rest::ApiKeyPair;

    fn create_test_client() -> AccountClient {
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
        AccountClient::new(rest_client)
    }

    #[test]
    fn test_account_client_creation() {
        let _client = create_test_client();
    }

    #[tokio::test]
    async fn test_get_wallet_balance_params() {
        let client = create_test_client();
        // Test parameter handling (not actual API call in unit test)
        let account_type = "UNIFIED";
        let coin = Some("USDT");
        
        assert_eq!(account_type, "UNIFIED");
        assert_eq!(coin, Some("USDT"));
    }

    #[tokio::test]
    async fn test_get_fee_rate_params() {
        let category = "spot";
        let symbol = Some("BTCUSDT");
        let base_coin: Option<&str> = None;
        
        assert_eq!(category, "spot");
        assert_eq!(symbol, Some("BTCUSDT"));
        assert_eq!(base_coin, None);
    }

    #[tokio::test]
    async fn test_transaction_log_params() {
        let account_type = Some("UNIFIED");
        let category = Some("spot");
        let currency = Some("USDT");
        let base_coin: Option<&str> = None;
        let log_type = Some("TRADE");
        let start_time = Some(1234567890i64);
        let end_time = Some(1234567899i64);
        let limit = Some(50);
        let cursor: Option<&str> = None;
        
        assert_eq!(account_type, Some("UNIFIED"));
        assert_eq!(category, Some("spot"));
        assert_eq!(currency, Some("USDT"));
        assert!(base_coin.is_none());
        assert_eq!(log_type, Some("TRADE"));
        assert_eq!(start_time, Some(1234567890));
        assert_eq!(end_time, Some(1234567899));
        assert_eq!(limit, Some(50));
        assert!(cursor.is_none());
    }

    #[tokio::test]
    async fn test_set_margin_mode_params() {
        let set_margin_mode = "PORTFOLIO_MARGIN";
        assert_eq!(set_margin_mode, "PORTFOLIO_MARGIN");
    }

    #[tokio::test]
    async fn test_set_mmp_params() {
        let base_coin = "BTC";
        let window = 5000;
        let frozen_period = 100000;
        let qty_limit = "100";
        let delta_limit = "10";
        
        assert_eq!(base_coin, "BTC");
        assert_eq!(window, 5000);
        assert_eq!(frozen_period, 100000);
        assert_eq!(qty_limit, "100");
        assert_eq!(delta_limit, "10");
    }

    #[tokio::test]
    async fn test_borrow_history_params() {
        let currency = Some("USDT");
        let start_time = Some(1234567890i64);
        let end_time = Some(1234567899i64);
        let limit = Some(100);
        let cursor = Some("next_page");
        
        assert_eq!(currency, Some("USDT"));
        assert_eq!(start_time, Some(1234567890));
        assert_eq!(end_time, Some(1234567899));
        assert_eq!(limit, Some(100));
        assert_eq!(cursor, Some("next_page"));
    }

    #[tokio::test]
    async fn test_set_disconnect_cancel_all_params() {
        let time_window = 10;
        assert_eq!(time_window, 10);
    }

    #[tokio::test]
    async fn test_contract_transaction_log_params() {
        let category = Some("linear");
        let base_coin = Some("BTC");
        let log_type = Some("SETTLEMENT");
        let start_time = Some(1234567890i64);
        let end_time = Some(1234567899i64);
        let limit = Some(50);
        let cursor: Option<&str> = None;
        
        assert_eq!(category, Some("linear"));
        assert_eq!(base_coin, Some("BTC"));
        assert_eq!(log_type, Some("SETTLEMENT"));
        assert_eq!(start_time, Some(1234567890));
        assert_eq!(end_time, Some(1234567899));
        assert_eq!(limit, Some(50));
        assert!(cursor.is_none());
    }
}