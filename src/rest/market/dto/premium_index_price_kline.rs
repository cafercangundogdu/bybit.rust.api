use crate::rest::client::ServerResponse;
use crate::rest::enums::category::Category;
use serde::{Deserialize, Serialize};

// https://bybit-exchange.github.io/docs/v5/market/preimum-index-kline#http-request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPremiumIndexPriceKlineParams {
    pub category: Option<Category>, // Product type. spot,linear,inverse. When category is not passed, use linear by default
    pub symbol: String,             // Symbol name. e.g. BTCUSD
    pub interval: String,           // Kline interval. 1,3,5,15,30,60,120,240,360,720,D,M,W
    pub start: Option<i64>,         // The start timestamp (ms)
    pub end: Option<i64>,           // The start timestamp (ms)
    pub limit: Option<i32>,         // The start timestamp (ms)
}

// https://bybit-exchange.github.io/docs/v5/market/preimum-index-kline#response-parameters
/*
{
    "retCode": 0,
    "retMsg": "OK",
    "result": {
        "symbol": "BTCUSDT",
        "category": "linear",
        "list": [
            [
                "1652486400000",
                "-0.000587",
                "-0.000344",
                "-0.000480",
                "-0.000344"
            ],
            [
                "1652400000000",
                "-0.000989",
                "-0.000561",
                "-0.000587",
                "-0.000587"
            ]
        ]
    },
    "retExtInfo": {},
    "time": 1672765216291
}
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PremiumIndexPriceKlineResult {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PremiumIndexPriceKlineResponse(ServerResponse<PremiumIndexPriceKlineResult>);

impl PremiumIndexPriceKlineResponse {
    pub fn into_inner(self) -> PremiumIndexPriceKlineResult {
        self.0.result
    }

    pub fn into_response(self) -> ServerResponse<PremiumIndexPriceKlineResult> {
        self.0
    }
}
