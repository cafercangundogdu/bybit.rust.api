use crate::rest::client::ServerResponse;
use crate::rest::enums::account_type::AccountType;
use crate::rest::enums::category::Category;
use serde::{Deserialize, Serialize};

// https://bybit-exchange.github.io/docs/v5/account/transaction-log#request-parameters
#[derive(Debug, Serialize, Deserialize)]
pub struct GetTransactionLogParams {
    #[serde(rename = "accountType")]
    account_type: AccountType,
    #[serde(rename = "category")]
    category: Category,
    // currency: String,
    // base_coin: String,
    // type: String,
    // start_time: i64,
    // end_time: i64,
    // limit: i32,
    // cursor: String,
}

// https://bybit-exchange.github.io/docs/v5/account/transaction-log#response-parameters
/*
{
    "retCode": 0,
    "retMsg": "OK",
    "result": {
        "nextPageCursor": "21963%3A1%2C14954%3A1",
        "list": [
            {
                "id": "592324_XRPUSDT_161440249321",
                "symbol": "XRPUSDT",
                "side": "Buy",
                "funding": "-0.003676",
                "orderLinkId": "",
                "orderId": "1672128000-8-592324-1-2",
                "fee": "0.00000000",
                "change": "-0.003676",
                "cashFlow": "0",
                "transactionTime": "1672128000000",
                "type": "SETTLEMENT",
                "feeRate": "0.0001",
                "bonusChange": "",
                "size": "100",
                "qty": "100",
                "cashBalance": "5086.55825002",
                "currency": "USDT",
                "category": "linear",
                "tradePrice": "0.3676",
                "tradeId": "534c0003-4bf7-486f-aa02-78cee36825e4"
            },
            {
                "id": "592324_XRPUSDT_161440249321",
                "symbol": "XRPUSDT",
                "side": "Buy",
                "funding": "",
                "orderLinkId": "linear-order",
                "orderId": "592b7e41-78fd-42e2-9aa3-91e1835ef3e1",
                "fee": "0.01908720",
                "change": "-0.0190872",
                "cashFlow": "0",
                "transactionTime": "1672121182224",
                "type": "TRADE",
                "feeRate": "0.0006",
                "bonusChange": "-0.1430544",
                "size": "100",
                "qty": "88",
                "cashBalance": "5086.56192602",
                "currency": "USDT",
                "category": "linear",
                "tradePrice": "0.3615",
                "tradeId": "5184f079-88ec-54c7-8774-5173cafd2b4e"
            },
            {
                "id": "592324_XRPUSDT_161407743011",
                "symbol": "XRPUSDT",
                "side": "Buy",
                "funding": "",
                "orderLinkId": "linear-order",
                "orderId": "592b7e41-78fd-42e2-9aa3-91e1835ef3e1",
                "fee": "0.00260280",
                "change": "-0.0026028",
                "cashFlow": "0",
                "transactionTime": "1672121182224",
                "type": "TRADE",
                "feeRate": "0.0006",
                "bonusChange": "",
                "size": "12",
                "qty": "12",
                "cashBalance": "5086.58101322",
                "currency": "USDT",
                "category": "linear",
                "tradePrice": "0.3615",
                "tradeId": "8569c10f-5061-5891-81c4-a54929847eb3"
            }
        ]
    },
    "retExtInfo": {},
    "time": 1672132481405
}
*/
#[derive(Debug, Serialize, Deserialize)]
struct TransactionLog {
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
struct TransactionLogResult {
    #[serde(rename = "nextPageCursor")]
    pub next_page_cursor: String,
    pub list: Vec<TransactionLog>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetTransactionLogResponse(ServerResponse<TransactionLogResult>);
