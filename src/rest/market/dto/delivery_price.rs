use crate::rest::client::ServerResponse;
use crate::rest::enums::category::Category;
use serde::{Deserialize, Serialize};

// https://bybit-exchange.github.io/docs/v5/market/delivery-price#request-parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDeliveryPriceParams {
    pub category: Category, // Product type. linear,inverse
    pub symbol: String,     // Symbol name. e.g. BTCUSD
}

// https://bybit-exchange.github.io/docs/v5/market/delivery-price#response-parameters
/*
{
    "retCode": 0,
    "retMsg": "success",
    "result": {
        "category": "option",
        "nextPageCursor": "",
        "list": [
            {
                "symbol": "ETH-26DEC22-1400-C",
                "deliveryPrice": "1220.728594450",
                "deliveryTime": "1672041600000"
            }
        ]
    },
    "retExtInfo": {},
    "time": 1672055336993
}
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryPrice {
    pub symbol: String,
    #[serde(rename = "deliveryPrice")]
    pub delivery_price: String,
    #[serde(rename = "deliveryTime")]
    pub delivery_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryPriceResult {
    pub category: String,
    #[serde(rename = "nextPageCursor")]
    pub next_page_cursor: String,
    pub list: Vec<DeliveryPrice>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryPriceResponse(ServerResponse<DeliveryPriceResult>);

impl DeliveryPriceResponse {
    pub fn into_inner(self) -> DeliveryPriceResult {
        self.0.result
    }

    pub fn into_response(self) -> ServerResponse<DeliveryPriceResult> {
        self.0
    }
}
