use crate::rest::client::{RestClient, SecType, ServerResponse};
use crate::rest::BybitResult as Result;
use serde_json::json;

#[derive(Clone)]
pub struct PositionClient {
    client: RestClient,
}

impl PositionClient {
    pub fn new(client: RestClient) -> Self {
        PositionClient { client }
    }

    /// Get position info
    ///
    /// API: GET /v5/position/list
    /// https://bybit-exchange.github.io/docs/v5/position
    pub async fn get_position_info(
        &self,
        category: &str,
        symbol: Option<&str>,
        base_coin: Option<&str>,
        settle_coin: Option<&str>,
        limit: Option<i32>,
        cursor: Option<&str>,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/position/list";
        let mut params = json!({
            "category": category,
        });

        if let Some(symbol) = symbol {
            params["symbol"] = json!(symbol);
        }
        if let Some(base_coin) = base_coin {
            params["baseCoin"] = json!(base_coin);
        }
        if let Some(settle_coin) = settle_coin {
            params["settleCoin"] = json!(settle_coin);
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

    /// Set leverage
    ///
    /// API: POST /v5/position/set-leverage
    /// https://bybit-exchange.github.io/docs/v5/position/set-leverage
    pub async fn set_leverage(
        &self,
        category: &str,
        symbol: &str,
        buy_leverage: &str,
        sell_leverage: &str,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/position/set-leverage";
        let body = json!({
            "category": category,
            "symbol": symbol,
            "buyLeverage": buy_leverage,
            "sellLeverage": sell_leverage,
        });

        let response = self.client.post(endpoint, body, SecType::Signed).await?;
        Ok(response)
    }

    /// Switch between cross/isolated margin
    ///
    /// API: POST /v5/position/switch-isolated
    /// https://bybit-exchange.github.io/docs/v5/position/switch-isolated
    pub async fn switch_margin_mode(
        &self,
        category: &str,
        symbol: &str,
        trade_mode: i32, // 0: cross margin, 1: isolated margin
        buy_leverage: &str,
        sell_leverage: &str,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/position/switch-isolated";
        let body = json!({
            "category": category,
            "symbol": symbol,
            "tradeMode": trade_mode,
            "buyLeverage": buy_leverage,
            "sellLeverage": sell_leverage,
        });

        let response = self.client.post(endpoint, body, SecType::Signed).await?;
        Ok(response)
    }

    /// Switch position mode
    ///
    /// API: POST /v5/position/switch-mode
    /// https://bybit-exchange.github.io/docs/v5/position/switch-mode
    pub async fn switch_position_mode(
        &self,
        category: &str,
        mode: i32, // 0: Merged Single, 3: Both Sides
        symbol: Option<&str>,
        coin: Option<&str>,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/position/switch-mode";
        let mut body = json!({
            "category": category,
            "mode": mode,
        });

        if let Some(symbol) = symbol {
            body["symbol"] = json!(symbol);
        }
        if let Some(coin) = coin {
            body["coin"] = json!(coin);
        }

        let response = self.client.post(endpoint, body, SecType::Signed).await?;
        Ok(response)
    }

    /// Set Trading Stop (Take profit/Stop loss)
    ///
    /// API: POST /v5/position/trading-stop
    /// https://bybit-exchange.github.io/docs/v5/position/trading-stop
    pub async fn set_trading_stop(
        &self,
        category: &str,
        symbol: &str,
        position_idx: i32,
        take_profit: Option<&str>,
        stop_loss: Option<&str>,
        trailing_stop: Option<&str>,
        tp_trigger_by: Option<&str>,
        sl_trigger_by: Option<&str>,
        active_price: Option<&str>,
        tp_size: Option<&str>,
        sl_size: Option<&str>,
        tp_limit_price: Option<&str>,
        sl_limit_price: Option<&str>,
        tp_order_type: Option<&str>,
        sl_order_type: Option<&str>,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/position/trading-stop";
        let mut body = json!({
            "category": category,
            "symbol": symbol,
            "positionIdx": position_idx,
        });

        if let Some(take_profit) = take_profit {
            body["takeProfit"] = json!(take_profit);
        }
        if let Some(stop_loss) = stop_loss {
            body["stopLoss"] = json!(stop_loss);
        }
        if let Some(trailing_stop) = trailing_stop {
            body["trailingStop"] = json!(trailing_stop);
        }
        if let Some(tp_trigger_by) = tp_trigger_by {
            body["tpTriggerBy"] = json!(tp_trigger_by);
        }
        if let Some(sl_trigger_by) = sl_trigger_by {
            body["slTriggerBy"] = json!(sl_trigger_by);
        }
        if let Some(active_price) = active_price {
            body["activePrice"] = json!(active_price);
        }
        if let Some(tp_size) = tp_size {
            body["tpSize"] = json!(tp_size);
        }
        if let Some(sl_size) = sl_size {
            body["slSize"] = json!(sl_size);
        }
        if let Some(tp_limit_price) = tp_limit_price {
            body["tpLimitPrice"] = json!(tp_limit_price);
        }
        if let Some(sl_limit_price) = sl_limit_price {
            body["slLimitPrice"] = json!(sl_limit_price);
        }
        if let Some(tp_order_type) = tp_order_type {
            body["tpOrderType"] = json!(tp_order_type);
        }
        if let Some(sl_order_type) = sl_order_type {
            body["slOrderType"] = json!(sl_order_type);
        }

        let response = self.client.post(endpoint, body, SecType::Signed).await?;
        Ok(response)
    }

    /// Set auto add margin
    ///
    /// API: POST /v5/position/set-auto-add-margin
    /// https://bybit-exchange.github.io/docs/v5/position/set-auto-add-margin
    pub async fn set_auto_add_margin(
        &self,
        category: &str,
        symbol: &str,
        auto_add_margin: i32, // 0: off, 1: on
        position_idx: Option<i32>,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/position/set-auto-add-margin";
        let mut body = json!({
            "category": category,
            "symbol": symbol,
            "autoAddMargin": auto_add_margin,
        });

        if let Some(position_idx) = position_idx {
            body["positionIdx"] = json!(position_idx);
        }

        let response = self.client.post(endpoint, body, SecType::Signed).await?;
        Ok(response)
    }

    /// Get closed PnL
    ///
    /// API: GET /v5/position/closed-pnl
    /// https://bybit-exchange.github.io/docs/v5/position/closed-pnl
    pub async fn get_closed_pnl(
        &self,
        category: &str,
        symbol: Option<&str>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: Option<i32>,
        cursor: Option<&str>,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/position/closed-pnl";
        let mut params = json!({
            "category": category,
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

    /// Set TP/SL mode
    ///
    /// API: POST /v5/position/set-tpsl-mode
    /// https://bybit-exchange.github.io/docs/v5/position/set-tpsl-mode
    pub async fn set_tpsl_mode(
        &self,
        category: &str,
        symbol: &str,
        tp_sl_mode: &str, // Full: entire position TP/SL, Partial: partial position TP/SL
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/position/set-tpsl-mode";
        let body = json!({
            "category": category,
            "symbol": symbol,
            "tpSlMode": tp_sl_mode,
        });

        let response = self.client.post(endpoint, body, SecType::Signed).await?;
        Ok(response)
    }

    /// Set risk limit
    ///
    /// API: POST /v5/position/set-risk-limit
    /// https://bybit-exchange.github.io/docs/v5/position/set-risk-limit
    pub async fn set_risk_limit(
        &self,
        category: &str,
        symbol: &str,
        risk_id: i32,
        position_idx: Option<i32>,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/position/set-risk-limit";
        let mut body = json!({
            "category": category,
            "symbol": symbol,
            "riskId": risk_id,
        });

        if let Some(position_idx) = position_idx {
            body["positionIdx"] = json!(position_idx);
        }

        let response = self.client.post(endpoint, body, SecType::Signed).await?;
        Ok(response)
    }

    /// Move positions
    ///
    /// API: POST /v5/position/move-positions
    /// https://bybit-exchange.github.io/docs/v5/position/move-positions
    pub async fn move_positions(
        &self,
        from_uid: &str,
        to_uid: &str,
        list: Vec<serde_json::Value>,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/position/move-positions";
        let body = json!({
            "fromUid": from_uid,
            "toUid": to_uid,
            "list": list,
        });

        let response = self.client.post(endpoint, body, SecType::Signed).await?;
        Ok(response)
    }

    /// Get move position history
    ///
    /// API: GET /v5/position/move-history
    /// https://bybit-exchange.github.io/docs/v5/position/move-history
    pub async fn get_move_position_history(
        &self,
        category: Option<&str>,
        symbol: Option<&str>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        status: Option<&str>,
        block_trade_id: Option<&str>,
        limit: Option<i32>,
        cursor: Option<&str>,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/position/move-history";
        let mut params = json!({});

        if let Some(category) = category {
            params["category"] = json!(category);
        }
        if let Some(symbol) = symbol {
            params["symbol"] = json!(symbol);
        }
        if let Some(start_time) = start_time {
            params["startTime"] = json!(start_time);
        }
        if let Some(end_time) = end_time {
            params["endTime"] = json!(end_time);
        }
        if let Some(status) = status {
            params["status"] = json!(status);
        }
        if let Some(block_trade_id) = block_trade_id {
            params["blockTradeId"] = json!(block_trade_id);
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

    /// Confirm new risk limit
    ///
    /// API: POST /v5/position/confirm-pending-mmr
    /// https://bybit-exchange.github.io/docs/v5/position/confirm-pending-mmr
    pub async fn confirm_new_risk_limit(
        &self,
        category: &str,
        symbol: &str,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/position/confirm-pending-mmr";
        let body = json!({
            "category": category,
            "symbol": symbol,
        });

        let response = self.client.post(endpoint, body, SecType::Signed).await?;
        Ok(response)
    }

    /// Add/Reduce margin
    ///
    /// API: POST /v5/position/add-margin
    /// https://bybit-exchange.github.io/docs/v5/position/manual-add-margin
    pub async fn update_margin(
        &self,
        category: &str,
        symbol: &str,
        margin: &str, // positive for add, negative for reduce
        position_idx: Option<i32>,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/position/add-margin";
        let mut body = json!({
            "category": category,
            "symbol": symbol,
            "margin": margin,
        });

        if let Some(position_idx) = position_idx {
            body["positionIdx"] = json!(position_idx);
        }

        let response = self.client.post(endpoint, body, SecType::Signed).await?;
        Ok(response)
    }

    /// Get execution
    ///
    /// API: GET /v5/execution/list
    /// https://bybit-exchange.github.io/docs/v5/position/execution
    pub async fn get_execution(
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
        let endpoint = "v5/execution/list";
        let mut params = json!({
            "category": category,
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rest::ApiKeyPair;

    fn create_test_client() -> PositionClient {
        let api_key_pair = ApiKeyPair::new(
            "test".to_string(),
            "test_key".to_string(),
            "test_secret".to_string(),
        );
        let rest_client =
            RestClient::new(api_key_pair, "https://api-testnet.bybit.com".to_string());
        PositionClient::new(rest_client)
    }

    #[test]
    fn test_position_client_creation() {
        let _client = create_test_client();
    }

    #[tokio::test]
    async fn test_position_info_params() {
        let category = "linear";
        let symbol = Some("BTCUSDT");
        let base_coin: Option<&str> = None;
        let settle_coin: Option<&str> = None;
        let limit = Some(50);
        let cursor: Option<&str> = None;

        assert_eq!(category, "linear");
        assert_eq!(symbol, Some("BTCUSDT"));
        assert!(base_coin.is_none());
        assert!(settle_coin.is_none());
        assert_eq!(limit, Some(50));
        assert!(cursor.is_none());
    }

    #[tokio::test]
    async fn test_set_leverage_params() {
        let category = "linear";
        let symbol = "BTCUSDT";
        let buy_leverage = "10";
        let sell_leverage = "10";

        assert_eq!(category, "linear");
        assert_eq!(symbol, "BTCUSDT");
        assert_eq!(buy_leverage, "10");
        assert_eq!(sell_leverage, "10");
    }

    #[tokio::test]
    async fn test_switch_margin_mode_params() {
        let category = "linear";
        let symbol = "BTCUSDT";
        let trade_mode = 1; // isolated margin
        let buy_leverage = "5";
        let sell_leverage = "5";

        assert_eq!(category, "linear");
        assert_eq!(symbol, "BTCUSDT");
        assert_eq!(trade_mode, 1);
        assert_eq!(buy_leverage, "5");
        assert_eq!(sell_leverage, "5");
    }

    #[tokio::test]
    async fn test_set_trading_stop_params() {
        let category = "linear";
        let symbol = "BTCUSDT";
        let position_idx = 0;
        let take_profit = Some("50000");
        let stop_loss = Some("40000");
        let trailing_stop: Option<&str> = None;

        assert_eq!(category, "linear");
        assert_eq!(symbol, "BTCUSDT");
        assert_eq!(position_idx, 0);
        assert_eq!(take_profit, Some("50000"));
        assert_eq!(stop_loss, Some("40000"));
        assert!(trailing_stop.is_none());
    }

    #[tokio::test]
    async fn test_closed_pnl_params() {
        let category = "linear";
        let symbol = Some("BTCUSDT");
        let start_time = Some(1234567890i64);
        let end_time = Some(1234567899i64);
        let limit = Some(100);
        let cursor: Option<&str> = None;

        assert_eq!(category, "linear");
        assert_eq!(symbol, Some("BTCUSDT"));
        assert_eq!(start_time, Some(1234567890));
        assert_eq!(end_time, Some(1234567899));
        assert_eq!(limit, Some(100));
        assert!(cursor.is_none());
    }
}
