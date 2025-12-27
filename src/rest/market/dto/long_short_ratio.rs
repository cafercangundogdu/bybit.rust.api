use crate::rest::client::ServerResponse;
use crate::rest::enums::category::Category;
use crate::rest::enums::interval_time::IntervalTime;
use serde::{Deserialize, Serialize};

// https://bybit-exchange.github.io/docs/v5/market/long-short-ratio#request-parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetLongShortRatioParams {
    pub category: Category,   // Product type. linear,inverse
    pub symbol: String,       // Symbol name. e.g. BTCUSD
    pub period: IntervalTime, // Data recording period. 5min, 15min, 30min, 1h, 4h, 1d
    pub limit: Option<i64>,   // Limit for data size per page. [1, 500]. Default: 50
}

// https://bybit-exchange.github.io/docs/v5/market/long-short-ratio#response-parameters
/*
{
    "retCode": 0,
    "retMsg": "OK",
    "result": {
        "list": [
            {
                "symbol": "BTCUSDT",
                "buyRatio": "0.5777",
                "sellRatio": "0.4223",
                "timestamp": "1695772800000"
            }
        ]
    },
    "retExtInfo": {},
    "time": 1695785131028
}
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LongShortRatio {
    pub symbol: String,
    #[serde(rename = "buyRatio")]
    pub buy_ratio: String,
    #[serde(rename = "sellRatio")]
    pub sell_ratio: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LongShortRatioResult {
    pub list: Vec<LongShortRatio>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LongShortRatioResponse(ServerResponse<LongShortRatioResult>);

impl LongShortRatioResponse {
    pub fn into_inner(self) -> LongShortRatioResult {
        self.0.result
    }

    pub fn into_response(self) -> ServerResponse<LongShortRatioResult> {
        self.0
    }
}
