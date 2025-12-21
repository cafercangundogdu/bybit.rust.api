use crate::rest::client::ServerResponse;
use serde::{Deserialize, Serialize};

// https://bybit-exchange.github.io/docs/v5/market/instrument#response-parameters
/*
{
    "retCode": 0,
    "retMsg": "OK",
    "result": {
        "category": "option",
        "nextPageCursor": "",
        "list": [
            {
                "symbol": "ETH-3JAN23-1250-P",
                "status": "Trading",
                "baseCoin": "ETH",
                "quoteCoin": "USD",
                "settleCoin": "USDC",
                "optionsType": "Put",
                "launchTime": "1672560000000",
                "deliveryTime": "1672732800000",
                "deliveryFeeRate": "0.00015",
                "priceFilter": {
                    "minPrice": "0.1",
                    "maxPrice": "10000000",
                    "tickSize": "0.1"
                },
                "lotSizeFilter": {
                    "maxOrderQty": "1500",
                    "minOrderQty": "0.1",
                    "qtyStep": "0.1"
                }
            }
        ]
    },
    "retExtInfo": {},
    "time": 1672712537130
}
*/

#[derive(Debug, Serialize, Deserialize)]
pub struct LotSizeFilter {
    #[serde(rename = "maxOrderQty")]
    pub max_order_qty: String,
    #[serde(rename = "minOrderQty")]
    pub min_order_qty: String,
    #[serde(rename = "qtyStep")]
    pub qty_step: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceFilter {
    #[serde(rename = "minPrice")]
    pub min_price: String,
    #[serde(rename = "maxPrice")]
    pub max_price: String,
    #[serde(rename = "tickSize")]
    pub tick_size: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InstrumentsInfoOption {
    pub symbol: String,
    pub status: String,
    #[serde(rename = "baseCoin")]
    pub base_coin: String,
    #[serde(rename = "quoteCoin")]
    pub quote_coin: String,
    #[serde(rename = "settleCoin")]
    pub settle_coin: String,
    #[serde(rename = "optionsType")]
    pub options_type: String,
    #[serde(rename = "launchTime")]
    pub launch_time: String,
    #[serde(rename = "deliveryTime")]
    pub delivery_time: String,
    #[serde(rename = "deliveryFeeRate")]
    pub delivery_fee_rate: String,
    #[serde(rename = "priceFilter")]
    pub price_filter: PriceFilter,
    #[serde(rename = "lotSizeFilter")]
    pub lot_size_filter: LotSizeFilter,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InstrumentsInfoOptionResult {
    pub category: String,
    #[serde(rename = "nextPageCursor")]
    pub next_page_cursor: String,
    pub list: Vec<InstrumentsInfoOption>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InstrumentsInfoOptionResponse(ServerResponse<InstrumentsInfoOptionResult>);

impl InstrumentsInfoOptionResponse {
    pub fn into_inner(self) -> InstrumentsInfoOptionResult {
        self.0.result
    }

    pub fn into_response(self) -> ServerResponse<InstrumentsInfoOptionResult> {
        self.0
    }
}
