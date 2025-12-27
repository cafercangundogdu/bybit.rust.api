use crate::rest::client::ServerResponse;
use crate::rest::enums::category::Category;
use crate::rest::enums::option_period::OptionPeriod;
use serde::{Deserialize, Serialize};

// https://bybit-exchange.github.io/docs/v5/market/iv#http-request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetHistoricalVolatilityParams {
    pub category: Category, // Product type. spot, linear, inverse, option
    #[serde(rename = "baseCoin")]
    pub base_coin: Option<String>, // Base coin. Default: return BTC data
    pub period: OptionPeriod, // Period. If not specified, it will return data with a 7-day average by default
    #[serde(rename = "startTime")]
    pub start_time: Option<u64>, // The start timestamp (ms)
    #[serde(rename = "endTime")]
    pub end_time: Option<u64>, // The end timestamp (ms)
}

// https://bybit-exchange.github.io/docs/v5/market/iv#response-parameters
/*
{
    "retCode": 0,
    "retMsg": "SUCCESS",
    "category": "option",
    "result": [
        {
            "period": 30,
            "value": "0.45024716",
            "time": "1672052400000"
        }
    ]
}
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalVolatility {
    pub period: i64,
    pub value: String,
    pub time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalVolatilityResult {
    pub category: String,
    pub list: Vec<HistoricalVolatility>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalVolatilityResponse(ServerResponse<HistoricalVolatilityResult>);

impl HistoricalVolatilityResponse {
    pub fn into_inner(self) -> HistoricalVolatilityResult {
        self.0.result
    }

    pub fn into_response(self) -> ServerResponse<HistoricalVolatilityResult> {
        self.0
    }
}
