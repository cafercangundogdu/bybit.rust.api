use crate::rest::client::ServerResponse;
use crate::rest::enums::category::Category;
use serde::{Deserialize, Serialize};

// https://bybit-exchange.github.io/docs/v5/market/orderbook#request-parameters
#[derive(Debug, Serialize, Deserialize)]
pub struct GetFundingRateHistoryParams {
    pub category: Category, // Product type. spot, linear, inverse, option
    pub symbol: String,     // Symbol name. e.g. BTCUSDT
    #[serde(rename = "startTime")]
    pub start_time: Option<i64>, // The start timestamp (ms)
    #[serde(rename = "endTime")]
    pub end_time: Option<i64>, // The end timestamp (ms)
    pub limit: Option<i32>, // The start timestamp (ms)
}

// https://bybit-exchange.github.io/docs/v5/market/orderbook#response-parameters
/*
{
    "retCode": 0,
    "retMsg": "OK",
    "result": {
        "category": "linear",
        "list": [
            {
                "symbol": "ETHPERP",
                "fundingRate": "0.0001",
                "fundingRateTimestamp": "1672041600000"
            }
        ]
    },
    "retExtInfo": {},
    "time": 1672051897447
}
*/

#[derive(Debug, Serialize, Deserialize)]
pub struct Funding {
    pub symbol: String,
    #[serde(rename = "fundingRate")]
    pub funding_rate: String,
    #[serde(rename = "fundingRateTimestamp")]
    pub funding_rate_timestamp: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FundingRateHistoryResult {
    pub category: String,
    pub list: Vec<Funding>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FundingRateHistoryResponse(ServerResponse<FundingRateHistoryResult>);

impl FundingRateHistoryResponse {
    pub fn into_inner(self) -> FundingRateHistoryResult {
        self.0.result
    }

    pub fn into_response(self) -> ServerResponse<FundingRateHistoryResult> {
        self.0
    }
}
