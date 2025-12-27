use crate::rest::client::ServerResponse;
use crate::rest::enums::category::Category;
use crate::rest::enums::interval_time::IntervalTime;
use serde::{Deserialize, Serialize};

// https://bybit-exchange.github.io/docs/v5/market/open-interest#http-request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOpenInterestParams {
    pub category: Category, // Product type. spot, linear, inverse, option
    pub symbol: String,     // Symbol name. e.g. BTCUSDT
    #[serde(rename = "intervalTime")]
    pub interval_time: IntervalTime, // Interval time. 5min, 15min, 30min, 1h, 4h, 1d
    #[serde(rename = "startTime")]
    pub start_time: Option<u64>, // The start timestamp (ms)
    #[serde(rename = "endTime")]
    pub end_time: Option<u64>, // The end timestamp (ms)
    pub limit: Option<u32>, // Limit for data size per page. [1, 200]. Default: 50
    pub cursor: Option<String>, // Cursor. Used to paginate
}

// https://bybit-exchange.github.io/docs/v5/market/open-interest#response-parameters
/*
{
    "retCode": 0,
    "retMsg": "OK",
    "result": {
        "symbol": "BTCUSD",
        "category": "inverse",
        "list": [
            {
                "openInterest": "461134384.00000000",
                "timestamp": "1669571400000"
            },
            {
                "openInterest": "461134292.00000000",
                "timestamp": "1669571100000"
            }
        ],
        "nextPageCursor": ""
    },
    "retExtInfo": {},
    "time": 1672053548579
}
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenInterest {
    #[serde(rename = "openInterest")]
    pub open_interest: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenInterestResult {
    pub symbol: String,
    pub category: String,
    pub list: Vec<OpenInterest>,
    #[serde(rename = "nextPageCursor")]
    pub next_page_cursor: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenInterestResponse(ServerResponse<OpenInterestResult>);

impl OpenInterestResponse {
    pub fn into_inner(self) -> OpenInterestResult {
        self.0.result
    }

    pub fn into_response(self) -> ServerResponse<OpenInterestResult> {
        self.0
    }
}
