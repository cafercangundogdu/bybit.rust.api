use crate::rest::enums::category::Category;
use serde::{Deserialize, Serialize};

// https://bybit-exchange.github.io/docs/v5/market/instrument#request-parameters
#[derive(Debug, Serialize, Deserialize)]
pub struct GetInstrumentsInfoParams {
    pub category: Category,     // Product type. spot,linear,inverse,option
    pub symbol: Option<String>, // Symbol name. e.g. BTCUSD
    pub status: Option<String>, // Symbol status filter,  spot/linear/inverse has Trading only
    #[serde(rename = "baseCoin")]
    pub base_coin: Option<i64>, // Base coin, Apply tolinear,inverse,option only, option: it returns BTC by default
    pub limit: Option<i64>, // Limit for data size per page. [1, 1000]. Default: 500
    pub cursor: Option<i32>, // Cursor. Use the nextPageCursor token from the response to retrieve the next page of the result set
}
