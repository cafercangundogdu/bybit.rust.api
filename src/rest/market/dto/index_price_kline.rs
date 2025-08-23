use crate::rest::client::ServerResponse;
use crate::rest::enums::category::Category;
use serde::{Deserialize, Serialize};

// https://bybit-exchange.github.io/docs/v5/market/index-kline#request-parameters
#[derive(Debug, Serialize, Deserialize)]
pub struct GetIndexPriceKlineParams {
    pub category: Option<Category>, // Product type. spot,linear,inverse. When category is not passed, use linear by default
    pub symbol: String,             // Symbol name. e.g. BTCUSD
    pub interval: String,           // Kline interval. 1,3,5,15,30,60,120,240,360,720,D,M,W
    pub start: Option<i64>,         // The start timestamp (ms)
    pub end: Option<i64>,           // The start timestamp (ms)
    pub limit: Option<i32>,         // The start timestamp (ms)
}

// https://bybit-exchange.github.io/docs/v5/market/index-kline#response-parameters
/*
{
    "retCode": 0,
    "retMsg": "OK",
    "result": {
        "symbol": "BTCUSDZ22",
        "category": "inverse",
        "list": [
            [
                "1670608800000",
                "17167.00",
                "17167.00",
                "17161.90",
                "17163.07"
            ],
            [
                "1670608740000",
                "17166.54",
                "17167.69",
                "17165.42",
                "17167.00"
            ]
        ]
    },
    "retExtInfo": {},
    "time": 1672026471128
}
*/

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexPriceKlineResult {
    pub symbol: String,   // Symbol name
    pub category: String, // Product type
    /**
     ** list[0]: startTime	string	Start time of the candle (ms)
     ** list[1]: openPrice	string	Open price
     ** list[2]: highPrice	string	Highest price
     ** list[3]: lowPrice	string	Lowest price
     ** list[4]: closePrice	string	Close price. Is the last traded price when the candle is not closed
     */
    pub list: Vec<Vec<String>>, // A string array of individual candle, Sort in reverse by startTime
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexPriceKlineResponse(ServerResponse<IndexPriceKlineResult>);

impl IndexPriceKlineResponse {
    pub fn into_inner(self) -> IndexPriceKlineResult {
        self.0.result
    }

    pub fn into_response(self) -> ServerResponse<IndexPriceKlineResult> {
        self.0
    }
}
