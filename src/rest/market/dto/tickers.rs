use crate::rest::client::ServerResponse;
use crate::rest::enums::category::Category;
use serde::{Deserialize, Serialize};

// https://bybit-exchange.github.io/docs/v5/market/tickers#http-request
#[derive(Debug, Serialize, Deserialize)]
pub struct GetTickersParams {
    pub category: Category,     // Product type. spot, linear, inverse, option
    pub symbol: Option<String>, // Symbol name. e.g. BTCUSDT
    #[serde(rename = "baseCoin")]
    pub base_coin: Option<String>, // Base coin. Apply to option only
    #[serde(rename = "expDate")]
    pub exp_date: Option<String>, // Expiry date. e.g., 25DEC22. Apply to option only
}

// https://bybit-exchange.github.io/docs/v5/market/tickers#response-parameters
/*
{
    "retCode": 0,
    "retMsg": "OK",
    "result": {
        "category": "spot",
        "list": [
            {
                "symbol": "BTCUSDT",
                "bid1Price": "20517.96",
                "bid1Size": "2",
                "ask1Price": "20527.77",
                "ask1Size": "1.862172",
                "lastPrice": "20533.13",
                "prevPrice24h": "20393.48",
                "price24hPcnt": "0.0068",
                "highPrice24h": "21128.12",
                "lowPrice24h": "20318.89",
                "turnover24h": "243765620.65899866",
                "volume24h": "11801.27771",
                "usdIndexPrice": "20784.12009279"
            }
        ]
    },
    "retExtInfo": {},
    "time": 1673859087947
}
*/

#[derive(Debug, Serialize, Deserialize)]
pub struct Ticker {
    pub symbol: String, // Symbol name
    #[serde(rename = "bid1Price")]
    pub bid1price: String, // Best bid price
    #[serde(rename = "bid1Size")]
    pub bid1size: String, // Best bid size
    #[serde(rename = "ask1Price")]
    pub ask1price: String, // Best ask price
    #[serde(rename = "ask1Size")]
    pub ask1size: String, // Best ask size
    #[serde(rename = "lastPrice")]
    pub last_price: String, // Last price
    #[serde(rename = "prevPrice24h")]
    pub prev_price24h: String, // Market price 24 hours ago
    #[serde(rename = "price24hPcnt")]
    pub price24h_pcnt: String, // Percentage change of market price relative to 24h
    #[serde(rename = "highPrice24h")]
    pub high_price24h: String, // The highest price in the last 24 hours
    #[serde(rename = "lowPrice24h")]
    pub low_price24h: String, // The lowest price in the last 24 hours
    pub turnover24h: String, // Turnover for 24h
    pub volume24h: String, // Volume for 24h
    #[serde(rename = "usdIndexPrice")]
    pub usd_index_price: String, // USD index price, used to calculate USD value of the assets in Unified account, non-collateral margin coin returns "", Only those trading pairs like "XXX/USDT" or "XXX/USDC" have the value
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TickersResult {
    pub category: String, // Product type
    pub list: Vec<Ticker>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TickersResponse(ServerResponse<TickersResult>);

impl TickersResponse {
    pub fn into_inner(self) -> TickersResult {
        self.0.result
    }

    pub fn into_response(self) -> ServerResponse<TickersResult> {
        self.0
    }
}
