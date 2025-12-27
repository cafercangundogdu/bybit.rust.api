use crate::rest::client::ServerResponse;
use crate::rest::enums::category::Category;
use serde::{Deserialize, Serialize};

// https://bybit-exchange.github.io/docs/v5/market/risk-limit#request-parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRiskLimitParams {
    pub category: Category, // Product type. linear,inverse
    pub symbol: String,     // Symbol name. e.g. BTCUSD
}

// https://bybit-exchange.github.io/docs/v5/market/risk-limit#response-parameters
/*
{
    "retCode": 0,
    "retMsg": "OK",
    "result": {
        "category": "inverse",
        "list": [
            {
                "id": 1,
                "symbol": "BTCUSD",
                "riskLimitValue": "150",
                "maintenanceMargin": "0.5",
                "initialMargin": "1",
                "isLowestRisk": 1,
                "maxLeverage": "100.00"
            },
        ]
    },
    "retExtInfo": {},
    "time": 1672054488010
}
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiskLimit {
    pub id: i64,
    pub symbol: String,
    pub risk_limit_value: String,
    pub maintenance_margin: String,
    pub initial_margin: String,
    pub is_lowest_risk: i64,
    pub max_leverage: String,
    pub mm_deduction: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskLimitResult {
    pub category: String,
    pub list: Vec<RiskLimit>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskLimitResponse(ServerResponse<RiskLimitResult>);

impl RiskLimitResponse {
    pub fn into_inner(self) -> RiskLimitResult {
        self.0.result
    }

    pub fn into_response(self) -> ServerResponse<RiskLimitResult> {
        self.0
    }
}
