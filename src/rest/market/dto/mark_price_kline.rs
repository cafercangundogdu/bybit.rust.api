use crate::rest::client::ServerResponse;
use crate::rest::enums::category::Category;
use serde::{Deserialize, Serialize};

// https://bybit-exchange.github.io/docs/v5/market/mark-kline#request-parameters
#[derive(Debug, Serialize, Deserialize)]
pub struct GetMarkPriceKlineParams {
    pub category: Option<Category>, // Product type. spot,linear,inverse. When category is not passed, use linear by default
    pub symbol: String,             // Symbol name. e.g. BTCUSD
    pub interval: String,           // Kline interval. 1,3,5,15,30,60,120,240,360,720,D,M,W
    pub start: Option<i64>,         // The start timestamp (ms)
    pub end: Option<i64>,           // The start timestamp (ms)
    pub limit: Option<i32>,         // The start timestamp (ms)
}

// https://bybit-exchange.github.io/docs/v5/market/mark-kline#response-parameters
/*
{
    "retCode": 0,
    "retMsg": "OK",
    "result": {
        "symbol": "BTCUSDT",
        "category": "linear",
        "list": [
            [
            "1670608800000",
            "17164.16",
            "17164.16",
            "17121.5",
            "17131.64"
            ]
        ]
    },
    "retExtInfo": {},
    "time": 1672026361839
}
*/

#[derive(Debug, Serialize, Deserialize)]
struct MarkPriceKlineResult {
    pub symbol: String,
    pub category: String,
    pub list: Vec<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarkPriceKlineResponse(ServerResponse<MarkPriceKlineResult>);

impl MarkPriceKlineResponse {
    pub fn into_inner(self) -> MarkPriceKlineResult {
        self.0.result
    }

    pub fn into_response(self) -> ServerResponse<MarkPriceKlineResult> {
        self.0
    }
}
