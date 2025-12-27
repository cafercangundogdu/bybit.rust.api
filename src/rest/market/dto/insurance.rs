use crate::rest::client::ServerResponse;
use serde::{Deserialize, Serialize};

// https://bybit-exchange.github.io/docs/v5/market/insurance#request-parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetInsuranceParams {
    pub coin: Option<String>, // coin. Default: return all insurance coins
}

// https://bybit-exchange.github.io/docs/v5/market/insurance#response-parameters
/*
{
    "retCode": 0,
    "retMsg": "OK",
    "result": {
        "updatedTime": "1672012800000",
        "list": [
            {
                "coin": "ETH",
                "balance": "0.00187332",
                "value": "0"
            }
        ]
},
    "retExtInfo": {},
    "time": 1672053931991
}
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Insurance {
    pub coin: String,
    pub balance: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsuranceResult {
    #[serde(rename = "updatedTime")]
    pub updated_time: String,
    pub list: Vec<Insurance>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsuranceResponse(ServerResponse<InsuranceResult>);

impl InsuranceResponse {
    pub fn into_inner(self) -> InsuranceResult {
        self.0.result
    }

    pub fn into_response(self) -> ServerResponse<InsuranceResult> {
        self.0
    }
}
