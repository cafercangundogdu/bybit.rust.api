use crate::rest::client::ServerResponse;
use crate::rest::enums::category::Category;
use serde::{Deserialize, Serialize};

// https://bybit-exchange.github.io/docs/v5/market/orderbook#request-parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOrderBookParams {
    pub category: Category, // Product type. spot, linear, inverse, option
    pub symbol: String,     // Symbol name. e.g. BTCUSDT
    pub limit: Option<i32>, // The start timestamp (ms)
}

// https://bybit-exchange.github.io/docs/v5/market/orderbook#response-parameters
/*
{
    "retCode": 0,
    "retMsg": "OK",
    "result": {
        "s": "BTCUSDT",
        "b": [
            [
                "62710.2",
                "5.52"
            ]
        ],
        "a": [
            [
                "62710.3",
                "0.01"
            ]
        ],
        "ts": 1709199771584,
        "u": 243781,
        "seq": 3887177875
    },
    "retExtInfo": {},
    "time": 1709199771643
}
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBookResult {
    pub s: String, // Symbol name
    pub b: Vec<
        // Bid, buyer. Order by price desc
        // b[0]: Bid price
        // b[1]: Bid size
        [String; 2],
    >,
    pub a: Vec<
        // Ask, seller. Order by price asc
        // a[0]: Ask price
        // a[1]: Ask size
        [String; 2],
    >,
    pub ts: i64,  // Timestamp (ms) that the system generates the data
    pub u: i64, // Update ID, is always in sequence, For contract, it is corresponding to u in the wss 500-level orderbook, For spot, it is corresponding to u in the wss 200-level orderbook
    pub seq: i64, // Cross sequence, You can use this field to compare different levels orderbook data, and for the smaller seq, then it means the data is generated earlier. Option does not have this field currently
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBookResponse(ServerResponse<OrderBookResult>);

impl OrderBookResponse {
    pub fn into_inner(self) -> OrderBookResult {
        self.0.result
    }

    pub fn into_response(self) -> ServerResponse<OrderBookResult> {
        self.0
    }
}
