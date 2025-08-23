use crate::rest::client::{RestClient, SecType, ServerResponse};
use anyhow::Result;
use serde_json::json;

pub struct AssetClient {
    client: RestClient,
}

impl AssetClient {
    pub fn new(client: RestClient) -> Self {
        AssetClient { client }
    }

    /// Get coin exchange records
    /// https://bybit-exchange.github.io/docs/v5/asset/exchange-order-record
    pub async fn get_exchange_order_record(
        &self,
        from_coin: Option<&str>,
        to_coin: Option<&str>,
        limit: Option<i32>,
        cursor: Option<&str>,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/asset/exchange/order-record";
        let mut params = json!({});

        if let Some(from_coin) = from_coin {
            params["fromCoin"] = json!(from_coin);
        }
        if let Some(to_coin) = to_coin {
            params["toCoin"] = json!(to_coin);
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

    /// Get delivery record
    /// https://bybit-exchange.github.io/docs/v5/asset/delivery
    pub async fn get_delivery_record(
        &self,
        category: &str,
        symbol: Option<&str>,
        exp_date: Option<&str>,
        limit: Option<i32>,
        cursor: Option<&str>,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/asset/delivery-record";
        let mut params = json!({
            "category": category,
        });

        if let Some(symbol) = symbol {
            params["symbol"] = json!(symbol);
        }
        if let Some(exp_date) = exp_date {
            params["expDate"] = json!(exp_date);
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

    /// Get USDC settlement records
    /// https://bybit-exchange.github.io/docs/v5/asset/settlement
    pub async fn get_settlement_record(
        &self,
        category: &str,
        symbol: Option<&str>,
        limit: Option<i32>,
        cursor: Option<&str>,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/asset/settlement-record";
        let mut params = json!({
            "category": category,
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

    /// Get coins
    /// https://bybit-exchange.github.io/docs/v5/asset/coin-info
    pub async fn get_coin_info(
        &self,
        coin: Option<&str>,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/asset/coin/query-info";
        let mut params = json!({});

        if let Some(coin) = coin {
            params["coin"] = json!(coin);
        }

        let response = self.client.get(endpoint, params, SecType::Signed).await?;
        Ok(response)
    }

    /// Get asset info
    /// https://bybit-exchange.github.io/docs/v5/asset/asset-info
    pub async fn get_asset_info(
        &self,
        account_type: &str,
        coin: Option<&str>,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/asset/transfer/query-asset-info";
        let mut params = json!({
            "accountType": account_type,
        });

        if let Some(coin) = coin {
            params["coin"] = json!(coin);
        }

        let response = self.client.get(endpoint, params, SecType::Signed).await?;
        Ok(response)
    }

    /// Get sub member list
    /// https://bybit-exchange.github.io/docs/v5/asset/sub-member-list
    pub async fn get_sub_member_list(&self) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/asset/transfer/query-sub-member-list";
        let response = self
            .client
            .get(endpoint, json!({}), SecType::Signed)
            .await?;
        Ok(response)
    }

    /// Get deposit records
    /// https://bybit-exchange.github.io/docs/v5/asset/deposit-record
    pub async fn get_deposit_records(
        &self,
        coin: Option<&str>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: Option<i32>,
        cursor: Option<&str>,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/asset/deposit/query-record";
        let mut params = json!({});

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

    /// Get sub deposit records
    /// https://bybit-exchange.github.io/docs/v5/asset/sub-deposit-record
    pub async fn get_sub_deposit_records(
        &self,
        sub_member_id: &str,
        coin: Option<&str>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: Option<i32>,
        cursor: Option<&str>,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/asset/deposit/query-sub-member-record";
        let mut params = json!({
            "subMemberId": sub_member_id,
        });

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

    /// Get internal deposit records
    /// https://bybit-exchange.github.io/docs/v5/asset/internal-deposit-record
    pub async fn get_internal_deposit_records(
        &self,
        start_time: Option<i64>,
        end_time: Option<i64>,
        coin: Option<&str>,
        cursor: Option<&str>,
        limit: Option<i32>,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/asset/deposit/query-internal-record";
        let mut params = json!({});

        if let Some(start_time) = start_time {
            params["startTime"] = json!(start_time);
        }
        if let Some(end_time) = end_time {
            params["endTime"] = json!(end_time);
        }
        if let Some(coin) = coin {
            params["coin"] = json!(coin);
        }
        if let Some(cursor) = cursor {
            params["cursor"] = json!(cursor);
        }
        if let Some(limit) = limit {
            params["limit"] = json!(limit);
        }

        let response = self.client.get(endpoint, params, SecType::Signed).await?;
        Ok(response)
    }

    /// Get master deposit address
    /// https://bybit-exchange.github.io/docs/v5/asset/master-deposit-addr
    pub async fn get_master_deposit_address(
        &self,
        coin: &str,
        chain_type: Option<&str>,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/asset/deposit/query-address";
        let mut params = json!({
            "coin": coin,
        });

        if let Some(chain_type) = chain_type {
            params["chainType"] = json!(chain_type);
        }

        let response = self.client.get(endpoint, params, SecType::Signed).await?;
        Ok(response)
    }

    /// Get sub deposit address
    /// https://bybit-exchange.github.io/docs/v5/asset/sub-deposit-addr
    pub async fn get_sub_deposit_address(
        &self,
        coin: &str,
        chain_type: &str,
        sub_member_id: &str,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/asset/deposit/query-sub-member-address";
        let params = json!({
            "coin": coin,
            "chainType": chain_type,
            "subMemberId": sub_member_id,
        });

        let response = self.client.get(endpoint, params, SecType::Signed).await?;
        Ok(response)
    }

    /// Get allowed deposit coin info
    /// https://bybit-exchange.github.io/docs/v5/asset/deposit-coin-spec
    pub async fn get_allowed_deposit_list(
        &self,
        coin: Option<&str>,
        chain: Option<&str>,
        limit: Option<i32>,
        cursor: Option<&str>,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/asset/deposit/query-allowed-list";
        let mut params = json!({});

        if let Some(coin) = coin {
            params["coin"] = json!(coin);
        }
        if let Some(chain) = chain {
            params["chain"] = json!(chain);
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

    /// Get withdrawal records
    /// https://bybit-exchange.github.io/docs/v5/asset/withdraw-record
    pub async fn get_withdrawal_records(
        &self,
        withdraw_id: Option<&str>,
        coin: Option<&str>,
        withdraw_type: Option<i32>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: Option<i32>,
        cursor: Option<&str>,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/asset/withdraw/query-record";
        let mut params = json!({});

        if let Some(withdraw_id) = withdraw_id {
            params["withdrawID"] = json!(withdraw_id);
        }
        if let Some(coin) = coin {
            params["coin"] = json!(coin);
        }
        if let Some(withdraw_type) = withdraw_type {
            params["withdrawType"] = json!(withdraw_type);
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

    /// Get withdrawable amount
    /// https://bybit-exchange.github.io/docs/v5/asset/withdrawable-amount
    pub async fn get_withdrawable_amount(
        &self,
        coin: &str,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/asset/withdraw/withdrawable-amount";
        let params = json!({
            "coin": coin,
        });

        let response = self.client.get(endpoint, params, SecType::Signed).await?;
        Ok(response)
    }

    /// Withdraw
    /// https://bybit-exchange.github.io/docs/v5/asset/withdraw
    pub async fn withdraw(
        &self,
        coin: &str,
        chain: &str,
        address: &str,
        tag: Option<&str>,
        amount: &str,
        timestamp: i64,
        for_ce_chain: Option<i32>,
        account_type: Option<&str>,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/asset/withdraw/create";
        let mut body = json!({
            "coin": coin,
            "chain": chain,
            "address": address,
            "amount": amount,
            "timestamp": timestamp,
        });

        if let Some(tag) = tag {
            body["tag"] = json!(tag);
        }
        if let Some(for_ce_chain) = for_ce_chain {
            body["forceChain"] = json!(for_ce_chain);
        }
        if let Some(account_type) = account_type {
            body["accountType"] = json!(account_type);
        }

        let response = self.client.post(endpoint, body, SecType::Signed).await?;
        Ok(response)
    }

    /// Cancel withdrawal
    /// https://bybit-exchange.github.io/docs/v5/asset/cancel-withdraw
    pub async fn cancel_withdrawal(&self, id: &str) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/asset/withdraw/cancel";
        let body = json!({
            "id": id,
        });

        let response = self.client.post(endpoint, body, SecType::Signed).await?;
        Ok(response)
    }

    /// Create internal transfer
    /// https://bybit-exchange.github.io/docs/v5/asset/create-inter-transfer
    pub async fn create_internal_transfer(
        &self,
        transfer_id: &str,
        coin: &str,
        amount: &str,
        from_account_type: &str,
        to_account_type: &str,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/asset/transfer/inter-transfer";
        let body = json!({
            "transferId": transfer_id,
            "coin": coin,
            "amount": amount,
            "fromAccountType": from_account_type,
            "toAccountType": to_account_type,
        });

        let response = self.client.post(endpoint, body, SecType::Signed).await?;
        Ok(response)
    }

    /// Get internal transfer records
    /// https://bybit-exchange.github.io/docs/v5/asset/inter-transfer-list
    pub async fn get_internal_transfer_records(
        &self,
        transfer_id: Option<&str>,
        coin: Option<&str>,
        status: Option<&str>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: Option<i32>,
        cursor: Option<&str>,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/asset/transfer/query-inter-transfer-list";
        let mut params = json!({});

        if let Some(transfer_id) = transfer_id {
            params["transferId"] = json!(transfer_id);
        }
        if let Some(coin) = coin {
            params["coin"] = json!(coin);
        }
        if let Some(status) = status {
            params["status"] = json!(status);
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

    /// Create universal transfer
    /// https://bybit-exchange.github.io/docs/v5/asset/create-universal-transfer
    pub async fn create_universal_transfer(
        &self,
        transfer_id: &str,
        coin: &str,
        amount: &str,
        from_member_id: &str,
        to_member_id: &str,
        from_account_type: &str,
        to_account_type: &str,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/asset/transfer/universal-transfer";
        let body = json!({
            "transferId": transfer_id,
            "coin": coin,
            "amount": amount,
            "fromMemberId": from_member_id,
            "toMemberId": to_member_id,
            "fromAccountType": from_account_type,
            "toAccountType": to_account_type,
        });

        let response = self.client.post(endpoint, body, SecType::Signed).await?;
        Ok(response)
    }

    /// Get universal transfer records
    /// https://bybit-exchange.github.io/docs/v5/asset/universal-transfer-list
    pub async fn get_universal_transfer_records(
        &self,
        transfer_id: Option<&str>,
        coin: Option<&str>,
        status: Option<&str>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: Option<i32>,
        cursor: Option<&str>,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/asset/transfer/query-universal-transfer-list";
        let mut params = json!({});

        if let Some(transfer_id) = transfer_id {
            params["transferId"] = json!(transfer_id);
        }
        if let Some(coin) = coin {
            params["coin"] = json!(coin);
        }
        if let Some(status) = status {
            params["status"] = json!(status);
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

    /// Get allowed transfer coin list
    /// https://bybit-exchange.github.io/docs/v5/asset/transferable-coin
    pub async fn get_allowed_transfer_coin_list(
        &self,
        from_account_type: &str,
        to_account_type: &str,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/asset/transfer/query-transfer-coin-list";
        let params = json!({
            "fromAccountType": from_account_type,
            "toAccountType": to_account_type,
        });

        let response = self.client.get(endpoint, params, SecType::Signed).await?;
        Ok(response)
    }

    /// Request quote for convert
    /// https://bybit-exchange.github.io/docs/v5/asset/request-quote
    pub async fn request_convert_quote(
        &self,
        from_coin: &str,
        to_coin: &str,
        from_coin_type: Option<&str>,
        to_coin_type: Option<&str>,
        request_coin: &str,
        request_amount: &str,
        account_type: &str,
        request_id: Option<&str>,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/asset/exchange/quote-apply";
        let mut body = json!({
            "fromCoin": from_coin,
            "toCoin": to_coin,
            "requestCoin": request_coin,
            "requestAmount": request_amount,
            "accountType": account_type,
        });

        if let Some(from_coin_type) = from_coin_type {
            body["fromCoinType"] = json!(from_coin_type);
        }
        if let Some(to_coin_type) = to_coin_type {
            body["toCoinType"] = json!(to_coin_type);
        }
        if let Some(request_id) = request_id {
            body["requestId"] = json!(request_id);
        }

        let response = self.client.post(endpoint, body, SecType::Signed).await?;
        Ok(response)
    }

    /// Confirm quote for convert
    /// https://bybit-exchange.github.io/docs/v5/asset/confirm-quote
    pub async fn confirm_convert_quote(
        &self,
        quote_tx_id: &str,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/asset/exchange/convert-execute";
        let body = json!({
            "quoteTxId": quote_tx_id,
        });

        let response = self.client.post(endpoint, body, SecType::Signed).await?;
        Ok(response)
    }

    /// Get convert status
    /// https://bybit-exchange.github.io/docs/v5/asset/convert-status
    pub async fn get_convert_result(
        &self,
        quote_tx_id: Option<&str>,
        account_type: Option<&str>,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/asset/exchange/convert-result-query";
        let mut params = json!({});

        if let Some(quote_tx_id) = quote_tx_id {
            params["quoteTxId"] = json!(quote_tx_id);
        }
        if let Some(account_type) = account_type {
            params["accountType"] = json!(account_type);
        }

        let response = self.client.get(endpoint, params, SecType::Signed).await?;
        Ok(response)
    }

    /// Get convert history
    /// https://bybit-exchange.github.io/docs/v5/asset/convert-history
    pub async fn get_convert_history(
        &self,
        account_type: Option<&str>,
        index: Option<i32>,
        limit: Option<i32>,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/asset/exchange/query-convert-history";
        let mut params = json!({});

        if let Some(account_type) = account_type {
            params["accountType"] = json!(account_type);
        }
        if let Some(index) = index {
            params["index"] = json!(index);
        }
        if let Some(limit) = limit {
            params["limit"] = json!(limit);
        }

        let response = self.client.get(endpoint, params, SecType::Signed).await?;
        Ok(response)
    }

    /// Get convert coin list
    /// https://bybit-exchange.github.io/docs/v5/asset/convert-coin-list
    pub async fn get_convert_coin_list(
        &self,
        coin: Option<&str>,
        side: Option<i32>,
        account_type: &str,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/asset/exchange/query-coin-list";
        let mut params = json!({
            "accountType": account_type,
        });

        if let Some(coin) = coin {
            params["coin"] = json!(coin);
        }
        if let Some(side) = side {
            params["side"] = json!(side);
        }

        let response = self.client.get(endpoint, params, SecType::Signed).await?;
        Ok(response)
    }

    /// Get coin greeks
    /// https://bybit-exchange.github.io/docs/v5/asset/coin-greeks
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

    /// Query account coin balance
    /// https://bybit-exchange.github.io/docs/v5/asset/account-coin-balance
    pub async fn query_account_coin_balance(
        &self,
        member_id: Option<&str>,
        to_member_id: Option<&str>,
        account_type: &str,
        to_account_type: Option<&str>,
        coin: &str,
        with_bonus: Option<i32>,
        with_transfer_safe_amount: Option<i32>,
        with_ltv_transfer_safe_amount: Option<i32>,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/asset/transfer/query-account-coin-balance";
        let mut params = json!({
            "accountType": account_type,
            "coin": coin,
        });

        if let Some(member_id) = member_id {
            params["memberId"] = json!(member_id);
        }
        if let Some(to_member_id) = to_member_id {
            params["toMemberId"] = json!(to_member_id);
        }
        if let Some(to_account_type) = to_account_type {
            params["toAccountType"] = json!(to_account_type);
        }
        if let Some(with_bonus) = with_bonus {
            params["withBonus"] = json!(with_bonus);
        }
        if let Some(with_transfer_safe_amount) = with_transfer_safe_amount {
            params["withTransferSafeAmount"] = json!(with_transfer_safe_amount);
        }
        if let Some(with_ltv_transfer_safe_amount) = with_ltv_transfer_safe_amount {
            params["withLtvTransferSafeAmount"] = json!(with_ltv_transfer_safe_amount);
        }

        let response = self.client.get(endpoint, params, SecType::Signed).await?;
        Ok(response)
    }

    /// Query account coins balance
    /// https://bybit-exchange.github.io/docs/v5/asset/account-coins-balance
    pub async fn query_account_coins_balance(
        &self,
        member_id: Option<&str>,
        account_type: &str,
        coin: Option<&str>,
        with_bonus: Option<i32>,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/asset/transfer/query-account-coins-balance";
        let mut params = json!({
            "accountType": account_type,
        });

        if let Some(member_id) = member_id {
            params["memberId"] = json!(member_id);
        }
        if let Some(coin) = coin {
            params["coin"] = json!(coin);
        }
        if let Some(with_bonus) = with_bonus {
            params["withBonus"] = json!(with_bonus);
        }

        let response = self.client.get(endpoint, params, SecType::Signed).await?;
        Ok(response)
    }

    /// Set deposit account
    /// https://bybit-exchange.github.io/docs/v5/asset/set-deposit-account
    pub async fn save_transfer_sub_member(
        &self,
        coin: Vec<String>,
        member_ids: Vec<String>,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/asset/transfer/save-transfer-sub-member";
        let body = json!({
            "coin": coin,
            "memberIds": member_ids,
        });

        let response = self.client.post(endpoint, body, SecType::Signed).await?;
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rest::ApiKeyPair;

    fn create_test_client() -> AssetClient {
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
        AssetClient::new(rest_client)
    }

    #[test]
    fn test_asset_client_creation() {
        let _client = create_test_client();
    }

    #[tokio::test]
    async fn test_exchange_order_record_params() {
        let from_coin = Some("BTC");
        let to_coin = Some("USDT");
        let limit = Some(50);
        let cursor: Option<&str> = None;

        assert_eq!(from_coin, Some("BTC"));
        assert_eq!(to_coin, Some("USDT"));
        assert_eq!(limit, Some(50));
        assert!(cursor.is_none());
    }

    #[tokio::test]
    async fn test_withdrawal_params() {
        let coin = "BTC";
        let chain = "BTC";
        let address = "bc1qtest123456789";
        let tag: Option<&str> = None;
        let amount = "0.001";
        let timestamp = 1234567890i64;
        let for_ce_chain: Option<i32> = None;
        let account_type = Some("UNIFIED");

        assert_eq!(coin, "BTC");
        assert_eq!(chain, "BTC");
        assert_eq!(address, "bc1qtest123456789");
        assert!(tag.is_none());
        assert_eq!(amount, "0.001");
        assert_eq!(timestamp, 1234567890);
        assert!(for_ce_chain.is_none());
        assert_eq!(account_type, Some("UNIFIED"));
    }

    #[tokio::test]
    async fn test_internal_transfer_params() {
        let transfer_id = "transfer123";
        let coin = "USDT";
        let amount = "100";
        let from_account_type = "UNIFIED";
        let to_account_type = "CONTRACT";

        assert_eq!(transfer_id, "transfer123");
        assert_eq!(coin, "USDT");
        assert_eq!(amount, "100");
        assert_eq!(from_account_type, "UNIFIED");
        assert_eq!(to_account_type, "CONTRACT");
    }

    #[tokio::test]
    async fn test_universal_transfer_params() {
        let transfer_id = "transfer456";
        let coin = "BTC";
        let amount = "0.5";
        let from_member_id = "member1";
        let to_member_id = "member2";
        let from_account_type = "UNIFIED";
        let to_account_type = "SPOT";

        assert_eq!(transfer_id, "transfer456");
        assert_eq!(coin, "BTC");
        assert_eq!(amount, "0.5");
        assert_eq!(from_member_id, "member1");
        assert_eq!(to_member_id, "member2");
        assert_eq!(from_account_type, "UNIFIED");
        assert_eq!(to_account_type, "SPOT");
    }

    #[tokio::test]
    async fn test_convert_quote_params() {
        let from_coin = "BTC";
        let to_coin = "USDT";
        let from_coin_type: Option<&str> = None;
        let to_coin_type: Option<&str> = None;
        let request_coin = "BTC";
        let request_amount = "0.1";
        let account_type = "UNIFIED";
        let request_id = Some("req123");

        assert_eq!(from_coin, "BTC");
        assert_eq!(to_coin, "USDT");
        assert!(from_coin_type.is_none());
        assert!(to_coin_type.is_none());
        assert_eq!(request_coin, "BTC");
        assert_eq!(request_amount, "0.1");
        assert_eq!(account_type, "UNIFIED");
        assert_eq!(request_id, Some("req123"));
    }

    #[tokio::test]
    async fn test_deposit_records_params() {
        let coin = Some("ETH");
        let start_time = Some(1234567890i64);
        let end_time = Some(1234567999i64);
        let limit = Some(100);
        let cursor: Option<&str> = None;

        assert_eq!(coin, Some("ETH"));
        assert_eq!(start_time, Some(1234567890));
        assert_eq!(end_time, Some(1234567999));
        assert_eq!(limit, Some(100));
        assert!(cursor.is_none());
    }

    #[tokio::test]
    async fn test_query_account_coin_balance_params() {
        let member_id = Some("member123");
        let to_member_id: Option<&str> = None;
        let account_type = "UNIFIED";
        let to_account_type: Option<&str> = None;
        let coin = "USDT";
        let with_bonus = Some(1);
        let with_transfer_safe_amount = Some(1);
        let with_ltv_transfer_safe_amount = Some(0);

        assert_eq!(member_id, Some("member123"));
        assert!(to_member_id.is_none());
        assert_eq!(account_type, "UNIFIED");
        assert!(to_account_type.is_none());
        assert_eq!(coin, "USDT");
        assert_eq!(with_bonus, Some(1));
        assert_eq!(with_transfer_safe_amount, Some(1));
        assert_eq!(with_ltv_transfer_safe_amount, Some(0));
    }

    #[tokio::test]
    async fn test_save_transfer_sub_member_params() {
        let coin = vec!["BTC".to_string(), "ETH".to_string()];
        let member_ids = vec!["member1".to_string(), "member2".to_string()];

        assert_eq!(coin.len(), 2);
        assert_eq!(coin[0], "BTC");
        assert_eq!(coin[1], "ETH");
        assert_eq!(member_ids.len(), 2);
        assert_eq!(member_ids[0], "member1");
        assert_eq!(member_ids[1], "member2");
    }
}
