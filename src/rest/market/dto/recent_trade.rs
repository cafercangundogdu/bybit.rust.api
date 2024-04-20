use crate::rest::client::ServerResponse;
use crate::rest::enums::category::Category;
use serde::{Deserialize, Serialize};

// https://bybit-exchange.github.io/docs/v5/market/tickers#http-request
#[derive(Debug, Serialize, Deserialize)]
pub struct GetRecentTradeParams {
    pub category: Category, // Product type. spot, linear, inverse, option
    pub symbol: String,     // Symbol name. e.g. BTCUSDT
    #[serde(rename = "baseCoin")]
    pub base_coin: Option<String>, // Base coin. Apply to option only, If the field is not passed, return BTC data by default
    #[serde(rename = "optionType")]
    pub option_type: Option<String>, // Option type. Call or Put. Apply to option only
    pub limit: Option<u32>, // Limit for data size per page, spot: [1,60], default: 60, others: [1,1000], default: 500
}

// https://bybit-exchange.github.io/docs/v5/market/tickers#response-parameters
/*
{
    "retCode": 0,
    "retMsg": "OK",
    "result": {
        "category": "spot",
        "list": [
            {
                "execId": "2100000000007764263",
                "symbol": "BTCUSDT",
                "price": "16618.49",
                "size": "0.00012",
                "side": "Buy",
                "time": "1672052955758",
                "isBlockTrade": false
            }
        ]
    },
    "retExtInfo": {},
    "time": 1672053054358
}
*/

#[derive(Debug, Serialize, Deserialize)]
pub struct RecentTrade {
    #[serde(rename = "execId")]
    pub exec_id: String,
    pub symbol: String,
    pub price: String,
    pub size: String,
    pub side: String,
    pub time: String,
    #[serde(rename = "isBlockTrade")]
    pub is_block_trade: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecentTradeResult {
    pub category: String, // Product type
    pub list: Vec<RecentTrade>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecentTradeResponse(ServerResponse<RecentTradeResult>);

impl RecentTradeResponse {
    pub fn into_inner(self) -> RecentTradeResult {
        self.0.result
    }

    pub fn into_response(self) -> ServerResponse<RecentTradeResult> {
        self.0
    }
}
