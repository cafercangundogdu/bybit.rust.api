use crate::rest::client::ServerResponse;
use serde::{Deserialize, Serialize};

// https://bybit-exchange.github.io/docs/v5/market/time
// https://bybit-exchange.github.io/docs/v5/market/time#response-example
/*
{
    "retCode": 0,
    "retMsg": "OK",
    "result": {
        "timeSecond": "1688639403",
        "timeNano": "1688639403423213947"
    },
    "retExtInfo": {},
    "time": 1688639403423
}
*/

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerTimeResult {
    #[serde(rename = "timeSecond")]
    pub time_second: String,
    #[serde(rename = "timeNano")]
    pub time_nano: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerTimeResponse(ServerResponse<ServerTimeResult>);

impl ServerTimeResponse {
    pub fn into_inner(self) -> ServerTimeResult {
        self.0.result
    }

    pub fn into_response(self) -> ServerResponse<ServerTimeResult> {
        self.0
    }
}
