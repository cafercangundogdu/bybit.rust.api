use crate::rest::enums::category::Category;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GetContractTransactionLogParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<Category>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractTransactionLog {
    pub symbol: String,
    pub side: String,
    pub funding: String,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractTransactionLogResult {
    #[serde(rename = "nextPageCursor")]
    pub next_page_cursor: String,
    pub list: Vec<ContractTransactionLog>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rest::client::ServerResponse;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_contract_transaction_log() {
        let json_data = r#"
        {
            "retCode": 0,
            "retMsg": "OK",
            "result": {
                "nextPageCursor": "cursor_contract",
                "list": [
                    {
                        "symbol": "BTCUSDT",
                        "side": "Buy",
                        "funding": "0.0001",
                        "orderId": "order12345",
                        "fee": "0.0005",
                        "change": "0.01",
                        "cashFlow": "0.01",
                        "transactionTime": "1690872862481",
                        "type": "SETTLEMENT",
                        "feeRate": "0.0006",
                        "size": "10",
                        "qty": "0.1",
                        "cashBalance": "5000",
                        "currency": "USDT",
                        "category": "linear",
                        "tradePrice": "30000",
                        "tradeId": "trade987"
                    }
                ]
            },
            "retExtInfo": {},
            "time": 1690872862481
        }
        "#;
        let response: ServerResponse<ContractTransactionLogResult> =
            from_str(json_data).expect("Failed to deserialize ContractTransactionLogResult");
        assert_eq!(response.ret_code, 0);
        let result = response.result;
        assert_eq!(result.list.len(), 1);
        assert_eq!(result.list[0].symbol, "BTCUSDT");
        assert_eq!(result.list[0].order_id, "order12345");
    }
}
