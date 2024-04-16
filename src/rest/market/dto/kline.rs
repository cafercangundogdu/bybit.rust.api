use crate::rest::client::ServerResponse;
use crate::rest::enums::category::Category;
use serde::{Deserialize, Serialize};

// https://bybit-exchange.github.io/docs/v5/market/kline#request-parameters
#[derive(Debug, Serialize, Deserialize)]
pub struct GetKlineParams {
    pub category: Option<Category>, // Product type. spot,linear,inverse. When category is not passed, use linear by default
    pub symbol: String,             // Symbol name. e.g. BTCUSD
    pub interval: String,           // Kline interval. 1,3,5,15,30,60,120,240,360,720,D,M,W
    pub start: Option<i64>,         // The start timestamp (ms)
    pub end: Option<i64>,           // The start timestamp (ms)
    pub limit: Option<i32>,         // The start timestamp (ms)
}

// https://bybit-exchange.github.io/docs/v5/market/kline#response-parameters
/*
{
    "retCode": 0,
    "retMsg": "OK",
    "result": {
        "symbol": "BTCUSD",
        "category": "inverse",
        "list": [
            [
                "1670608800000",
                "17071",
                "17073",
                "17027",
                "17055.5",
                "268611",
                "15.74462667"
            ],
            [
                "1670605200000",
                "17071.5",
                "17071.5",
                "17061",
                "17071",
                "4177",
                "0.24469757"
            ],
            [
                "1670601600000",
                "17086.5",
                "17088",
                "16978",
                "17071.5",
                "6356",
                "0.37288112"
            ]
        ]
    },
    "retExtInfo": {},
    "time": 1672025956592
}
*/

#[derive(Debug, Serialize, Deserialize)]
struct KlineResult {
    pub symbol: String,
    pub category: String,
    pub list: Vec<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KlineResponse(ServerResponse<KlineResult>);

impl KlineResponse {
    pub fn into_inner(self) -> KlineResult {
        self.0.result
    }

    pub fn into_response(self) -> ServerResponse<KlineResult> {
        self.0
    }
}
