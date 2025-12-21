use crate::rest::enums::account_type::AccountType;
use crate::rest::enums::category::Category;
use serde::{Deserialize, Serialize};

// https://bybit-exchange.github.io/docs/v5/account/transaction-log#request-parameters
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GetTransactionLogParams {
    #[serde(rename = "accountType", skip_serializing_if = "Option::is_none")]
    pub account_type: Option<AccountType>,
    #[serde(rename = "category", skip_serializing_if = "Option::is_none")]
    pub category: Option<Category>,
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "baseCoin", skip_serializing_if = "Option::is_none")]
    pub base_coin: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub log_type: Option<String>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionLog {
    pub id: String,
    pub symbol: String,
    pub side: String,
    pub funding: String,
    #[serde(rename = "orderLinkId")]
    pub order_link_id: String,
    #[serde(rename = "orderId")]
    pub order_id: String,
    pub fee: String,
    pub change: String,
    #[serde(rename = "cashFlow")]
    pub cash_flow: String,
    #[serde(rename = "transactionTime")]
    pub transaction_time: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "feeRate")]
    pub fee_rate: String,
    #[serde(rename = "bonusChange")]
    pub bonus_change: String,
    pub size: String,
    pub qty: String,
    #[serde(rename = "cashBalance")]
    pub cash_balance: String,
    pub currency: String,
    pub category: String,
    #[serde(rename = "tradePrice")]
    pub trade_price: String,
    #[serde(rename = "tradeId")]
    pub trade_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionLogResult {
    #[serde(rename = "nextPageCursor")]
    pub next_page_cursor: String,
    pub list: Vec<TransactionLog>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rest::client::ServerResponse;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_transaction_log() {
        let json_data = r#"
        {
            "retCode": 0,
            "retMsg": "OK",
            "result": {
                "nextPageCursor": "cursor123",
                "list": [
                    {
                        "id": "12345",
                        "symbol": "BTCUSDT",
                        "side": "Buy",
                        "funding": "0.0001",
                        "orderLinkId": "link123",
                        "orderId": "order123",
                        "fee": "0.0005",
                        "change": "0.1",
                        "cashFlow": "0.05",
                        "transactionTime": "1690872862481",
                        "type": "TRADE",
                        "feeRate": "0.0006",
                        "bonusChange": "0",
                        "size": "100",
                        "qty": "0.1",
                        "cashBalance": "1000",
                        "currency": "USDT",
                        "category": "spot",
                        "tradePrice": "30000",
                        "tradeId": "trade123"
                    }
                ]
            },
            "retExtInfo": {},
            "time": 1690872862481
        }
        "#;
        let response: ServerResponse<TransactionLogResult> =
            from_str(json_data).expect("Failed to deserialize TransactionLogResult");
        assert_eq!(response.ret_code, 0);
        let result = response.result;
        assert_eq!(result.list.len(), 1);
        assert_eq!(result.list[0].symbol, "BTCUSDT");
        assert_eq!(result.list[0].trade_id, "trade123");
    }
}
