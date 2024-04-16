use crate::rest::client::ServerResponse;
use serde::{Deserialize, Serialize};

// https://bybit-exchange.github.io/docs/v5/market/instrument#response-parameters
/*
{
    "retCode": 0,
    "retMsg": "OK",
    "result": {
        "category": "spot",
        "list": [
            {
                "symbol": "BTCUSDT",
                "baseCoin": "BTC",
                "quoteCoin": "USDT",
                "innovation": "0",
                "status": "Trading",
                "marginTrading": "both",
                "lotSizeFilter": {
                    "basePrecision": "0.000001",
                    "quotePrecision": "0.00000001",
                    "minOrderQty": "0.000048",
                    "maxOrderQty": "71.73956243",
                    "minOrderAmt": "1",
                    "maxOrderAmt": "2000000"
                },
                "priceFilter": {
                    "tickSize": "0.01"
                },
                "riskParameters": {
                    "limitParameter": "0.05",
                    "marketParameter": "0.05"
                }
            }
        ]
    },
    "retExtInfo": {},
    "time": 1672712468011
}
*/

#[derive(Debug, Serialize, Deserialize)]
struct RiskParameters {
    #[serde(rename = "limitParameter")]
    pub limit_parameter: String,
    #[serde(rename = "marketParameter")]
    pub market_parameter: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PriceFilter {
    #[serde(rename = "tickSize")]
    pub tick_size: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct LotSizeFilter {
    #[serde(rename = "basePrecision")]
    pub base_precision: String,
    #[serde(rename = "quotePrecision")]
    pub quote_precision: String,
    #[serde(rename = "minOrderQty")]
    pub min_order_qty: String,
    #[serde(rename = "maxOrderQty")]
    pub max_order_qty: String,
    #[serde(rename = "minOrderAmt")]
    pub min_order_amt: String,
    #[serde(rename = "maxOrderAmt")]
    pub max_order_amt: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct InstrumentsInfoSpot {
    pub symbol: String,
    #[serde(rename = "baseCoin")]
    pub base_coin: String,
    #[serde(rename = "quoteCoin")]
    pub quote_coin: String,
    pub innovation: String,
    pub status: String,
    #[serde(rename = "marginTrading")]
    pub margin_trading: String,
    #[serde(rename = "lotSizeFilter")]
    pub lot_size_filter: LotSizeFilter,
    #[serde(rename = "priceFilter")]
    pub price_filter: PriceFilter,
    #[serde(rename = "riskParameters")]
    pub risk_parameters: RiskParameters,
}

#[derive(Debug, Serialize, Deserialize)]
struct InstrumentsInfoSpotResult {
    pub category: String,
    pub list: Vec<InstrumentsInfoSpot>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InstrumentsInfoSpotResponse(ServerResponse<InstrumentsInfoSpotResult>);

impl InstrumentsInfoSpotResponse {
    pub fn into_inner(self) -> InstrumentsInfoSpotResult {
        self.0.result
    }

    pub fn into_response(self) -> ServerResponse<InstrumentsInfoSpotResult> {
        self.0
    }
}
