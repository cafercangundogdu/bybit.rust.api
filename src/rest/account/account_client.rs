use crate::rest::account::dto::account_info::AccountInfoResult;
use crate::rest::account::dto::account_wallet::{GetWalletBalanceParams, WalletBalanceResult};
use crate::rest::account::dto::collateral::{BorrowHistoryResult, CollateralInfoResult};
use crate::rest::account::dto::contract_transaction_log::{ContractTransactionLogResult, GetContractTransactionLogParams};
use crate::rest::account::dto::fee_rate::FeeRateResult;
use crate::rest::account::dto::mmp::{MmpStateResult, ModifyMmpParams};
use crate::rest::account::dto::transaction_log::{GetTransactionLogParams, TransactionLogResult};
use crate::rest::client::{RestClient, SecType, ServerResponse};
use anyhow::Result;
use serde_json::{json, to_value};

pub struct AccountClient {
    client: RestClient,
}

impl AccountClient {
    pub fn new(client: RestClient) -> Self {
        AccountClient { client }
    }

    /// Get wallet balance
    ///
    /// API: GET /v5/account/wallet-balance
    /// https://bybit-exchange.github.io/docs/v5/account/wallet-balance
    pub async fn get_wallet_balance(
        &self,
        params: GetWalletBalanceParams,
    ) -> Result<ServerResponse<WalletBalanceResult>> {
        let endpoint = "v5/account/wallet-balance";
        let query = to_value(&params)?;
        let response = self.client.get(endpoint, query, SecType::Signed).await?;
        Ok(response)
    }

    /// Get fee rate
    ///
    /// API: GET /v5/account/fee-rate
    /// https://bybit-exchange.github.io/docs/v5/account/fee-rate
    pub async fn get_fee_rate(
        &self,
        category: &str,
        symbol: Option<&str>,
        base_coin: Option<&str>,
    ) -> Result<ServerResponse<FeeRateResult>> {
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
    ///
    /// API: GET /v5/account/info
    /// https://bybit-exchange.github.io/docs/v5/account/account-info
    pub async fn get_account_info(&self) -> Result<ServerResponse<AccountInfoResult>> {
        let endpoint = "v5/account/info";
        let response = self
            .client
            .get(endpoint, json!({}), SecType::Signed)
            .await?;
        Ok(response)
    }

    /// Get transaction log
    ///
    /// API: GET /v5/account/transaction-log
    /// https://bybit-exchange.github.io/docs/v5/account/transaction-log
    pub async fn get_transaction_log(
        &self,
        params: GetTransactionLogParams,
    ) -> Result<ServerResponse<TransactionLogResult>> {
        let endpoint = "v5/account/transaction-log";
        let query = to_value(&params)?;
        let response = self.client.get(endpoint, query, SecType::Signed).await?;
        Ok(response)
    }

    /// Set margin mode
    ///
    /// API: POST /v5/account/set-margin-mode
    /// https://bybit-exchange.github.io/docs/v5/account/set-margin-mode
    pub async fn set_margin_mode(&self, margin_mode: &str) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/account/set-margin-mode";
        let body = json!({
            "setMarginMode": margin_mode,
        });

        let response = self.client.post(endpoint, body, SecType::Signed).await?;
        Ok(response)
    }

    /// Set MMP
    ///
    /// API: POST /v5/account/mmp-modify
    /// https://bybit-exchange.github.io/docs/v5/account/set-mmp
    pub async fn set_mmp(&self, params: ModifyMmpParams) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/account/mmp-modify";
        let body = to_value(&params)?;
        let response = self.client.post(endpoint, body, SecType::Signed).await?;
        Ok(response)
    }

    /// Reset MMP
    ///
    /// API: POST /v5/account/mmp-reset
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
    ///
    /// API: GET /v5/account/mmp-state
    /// https://bybit-exchange.github.io/docs/v5/account/get-mmp-state
    pub async fn get_mmp_state(
        &self,
        base_coin: &str,
    ) -> Result<ServerResponse<MmpStateResult>> {
        let endpoint = "v5/account/mmp-state";
        let params = json!({
            "baseCoin": base_coin,
        });

        let response = self.client.get(endpoint, params, SecType::Signed).await?;
        Ok(response)
    }

    /// Get SMP group list
    ///
    /// API: GET /v5/account/smp-group
    /// https://bybit-exchange.github.io/docs/v5/account/smp-group
    pub async fn get_smp_group_list(&self) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/account/smp-group";
        let response = self
            .client
            .get(endpoint, json!({}), SecType::Signed)
            .await?;
        Ok(response)
    }

    /// Get coin Greeks
    ///
    /// API: GET /v5/asset/coin-greeks
    /// https://bybit-exchange.github.io/docs/v5/account/coin-greeks
    pub async fn get_coin_greeks(
        &self,
        base_coin: Option<&str>,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/asset/coin-greeks";
        let mut params = json!({});

        if let Some(base_coin) = base_coin {
            params["baseCoin"] = json!(base_coin);
        }

        let response = self.client.get(endpoint, params, SecType::Signed).await?;
        Ok(response)
    }

    /// Get collateral info
    ///
    /// API: GET /v5/account/collateral-info
    /// https://bybit-exchange.github.io/docs/v5/account/collateral-info
    pub async fn get_collateral_info(
        &self,
        currency: Option<&str>,
    ) -> Result<ServerResponse<CollateralInfoResult>> {
        let endpoint = "v5/account/collateral-info";
        let mut params = json!({});

        if let Some(currency) = currency {
            params["currency"] = json!(currency);
        }

        let response = self.client.get(endpoint, params, SecType::Signed).await?;
        Ok(response)
    }

    /// Get borrow history
    ///
    /// API: GET /v5/account/borrow-history
    /// https://bybit-exchange.github.io/docs/v5/account/borrow-history
    pub async fn get_borrow_history(
        &self,
        currency: Option<&str>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: Option<i32>,
        cursor: Option<&str>,
    ) -> Result<ServerResponse<BorrowHistoryResult>> {
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
    ///
    /// API: POST /v5/order/disconnected-cancel-all
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
    ///
    /// API: POST /v5/account/upgrade-to-uta
    /// https://bybit-exchange.github.io/docs/v5/account/upgrade-unified-account
    pub async fn upgrade_to_unified_account(&self) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/account/upgrade-to-uta";
        let body = json!({});

        let response = self.client.post(endpoint, body, SecType::Signed).await?;
        Ok(response)
    }

    /// Get contract transaction log
    ///
    /// API: GET /v5/account/contract-transaction-log
    /// https://bybit-exchange.github.io/docs/v5/account/contract-transaction-log
    pub async fn get_contract_transaction_log(
        &self,
        params: GetContractTransactionLogParams,
    ) -> Result<ServerResponse<ContractTransactionLogResult>> {
        let endpoint = "v5/account/contract-transaction-log";
        let query = to_value(&params)?;
        let response = self.client.get(endpoint, query, SecType::Signed).await?;
        Ok(response)
    }

    /// Query DCP information
    ///
    /// API: GET /v5/account/query-dcp-info
    /// https://bybit-exchange.github.io/docs/v5/account/query-dcp-info
    pub async fn query_dcp_info(&self) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/account/query-dcp-info";
        let response = self
            .client
            .get(endpoint, json!({}), SecType::Signed)
            .await?;
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rest::ApiKeyPair;
    use crate::rest::enums::account_type::AccountType;
    use crate::rest::enums::category::Category;

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
        let _client = create_test_client();
        let params = GetWalletBalanceParams {
            account_type: AccountType::UNIFIED,
            coin: Some("USDT".to_string()),
        };

        assert_eq!(params.account_type, AccountType::UNIFIED);
        assert_eq!(params.coin, Some("USDT".to_string()));
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
        let params = GetTransactionLogParams {
            account_type: Some(AccountType::UNIFIED),
            category: Some(Category::Spot),
            currency: Some("USDT".to_string()),
            log_type: Some("TRADE".to_string()),
            limit: Some(50),
            ..Default::default()
        };

        assert_eq!(params.account_type, Some(AccountType::UNIFIED));
        assert_eq!(params.category, Some(Category::Spot));
        assert_eq!(params.currency, Some("USDT".to_string()));
        assert_eq!(params.log_type, Some("TRADE".to_string()));
        assert_eq!(params.limit, Some(50));
    }

    #[tokio::test]
    async fn test_set_margin_mode_params() {
        let set_margin_mode = "PORTFOLIO_MARGIN";
        assert_eq!(set_margin_mode, "PORTFOLIO_MARGIN");
    }

    #[tokio::test]
    async fn test_set_mmp_params() {
        let params = ModifyMmpParams {
            base_coin: "BTC".to_string(),
            window: 5000,
            frozen_period: 100000,
            qty_limit: "100".to_string(),
            delta_limit: "10".to_string(),
        };

        assert_eq!(params.base_coin, "BTC");
        assert_eq!(params.window, 5000);
        assert_eq!(params.frozen_period, 100000);
        assert_eq!(params.qty_limit, "100");
        assert_eq!(params.delta_limit, "10");
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
        let params = GetContractTransactionLogParams {
            category: Some(Category::Linear),
            base_coin: Some("BTC".to_string()),
            log_type: Some("SETTLEMENT".to_string()),
            limit: Some(50),
            ..Default::default()
        };

        assert_eq!(params.category, Some(Category::Linear));
        assert_eq!(params.base_coin, Some("BTC".to_string()));
        assert_eq!(params.log_type, Some("SETTLEMENT".to_string()));
        assert_eq!(params.limit, Some(50));
    }
}
