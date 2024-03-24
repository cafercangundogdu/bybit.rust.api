use crate::rest::account::account_type::AccountType;
use crate::rest::client::ServerResponse;
use serde::{Deserialize, Serialize};

// https://bybit-exchange.github.io/docs/v5/account/wallet-balance#request-parameters
#[derive(Debug, Serialize, Deserialize)]
pub struct GetWalletBalanceParams {
    #[serde(rename = "accountType")]
    pub account_type: AccountType,
    // coin: String, // BTC
}

// https://bybit-exchange.github.io/docs/v5/account/wallet-balance#response-parameters
/*
{
    "retCode": 0,
    "retMsg": "OK",
    "result": {
        "list": [
            {
                "totalEquity": "3.31216591",
                "accountIMRate": "0",
                "totalMarginBalance": "3.00326056",
                "totalInitialMargin": "0",
                "accountType": "UNIFIED",
                "totalAvailableBalance": "3.00326056",
                "accountMMRate": "0",
                "totalPerpUPL": "0",
                "totalWalletBalance": "3.00326056",
                "accountLTV": "0",
                "totalMaintenanceMargin": "0",
                "coin": [
                    {
                        "availableToBorrow": "3",
                        "bonus": "0",
                        "accruedInterest": "0",
                        "availableToWithdraw": "0",
                        "totalOrderIM": "0",
                        "equity": "0",
                        "totalPositionMM": "0",
                        "usdValue": "0",
                        "spotHedgingQty": "0.01592413",
                        "unrealisedPnl": "0",
                        "collateralSwitch": true,
                        "borrowAmount": "0.0",
                        "totalPositionIM": "0",
                        "walletBalance": "0",
                        "cumRealisedPnl": "0",
                        "locked": "0",
                        "marginCollateral": true,
                        "coin": "BTC"
                    }
                ]
            }
        ]
    },
    "retExtInfo": {},
    "time": 1690872862481
}
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct CoinWalletBalanceDetails {
    #[serde(rename = "availableToBorrow")]
    pub available_to_borrow: String,
    pub bonus: String,
    #[serde(rename = "accruedInterest")]
    pub accrued_interest: String,
    #[serde(rename = "availableToWithdraw")]
    pub available_to_withdraw: String,
    #[serde(rename = "totalOrderIM")]
    pub total_order_im: String,
    pub equity: String,
    #[serde(rename = "totalPositionMM")]
    pub total_position_mm: String,
    #[serde(rename = "usdValue")]
    pub usd_value: String,
    #[serde(rename = "spotHedgingQty")]
    pub spot_hedging_qty: String,
    #[serde(rename = "unrealisedPnl")]
    pub unrealised_pnl: String,
    #[serde(rename = "collateralSwitch")]
    pub collateral_switch: bool,
    #[serde(rename = "borrowAmount")]
    pub borrow_amount: String,
    #[serde(rename = "totalPositionIM")]
    pub total_position_im: String,
    #[serde(rename = "walletBalance")]
    pub wallet_balance: String,
    #[serde(rename = "cumRealisedPnl")]
    pub cum_realised_pnl: String,
    pub locked: String,
    #[serde(rename = "marginCollateral")]
    pub margin_collateral: bool,
    pub coin: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WalletBalanceDetails {
    #[serde(rename = "totalEquity")]
    pub total_equity: String,
    #[serde(rename = "accountIMRate")]
    pub account_imrate: String,
    #[serde(rename = "totalMarginBalance")]
    pub total_margin_balance: String,
    #[serde(rename = "totalInitialMargin")]
    pub total_initial_margin: String,
    #[serde(rename = "accountType")]
    pub account_type: String,
    #[serde(rename = "totalAvailableBalance")]
    pub total_available_balance: String,
    #[serde(rename = "accountMMRate")]
    pub account_mmrate: String,
    #[serde(rename = "totalPerpUPL")]
    pub total_perp_upl: String,
    #[serde(rename = "totalWalletBalance")]
    pub total_wallet_balance: String,
    #[serde(rename = "accountLTV")]
    pub account_ltv: String,
    #[serde(rename = "totalMaintenanceMargin")]
    pub total_maintenance_margin: String,
    pub coin: Vec<CoinWalletBalanceDetails>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WalletBalanceResult {
    // optional
    #[serde(rename = "list")]
    pub list: Option<Vec<WalletBalanceDetails>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetWalletBalanceResponse(ServerResponse<WalletBalanceResult>);

impl GetWalletBalanceResponse {
    pub fn into_inner(self) -> WalletBalanceResult {
        self.0.result
    }

    pub fn into_response(self) -> ServerResponse<WalletBalanceResult> {
        self.0
    }
}
