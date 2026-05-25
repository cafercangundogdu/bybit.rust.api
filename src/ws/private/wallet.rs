//! Wallet stream — real-time wallet balance updates.
//!
//! # Topics
//! - `wallet` (all categories)
//! - `wallet.linear`, `wallet.inverse`, `wallet.spot`, `wallet.option`

use serde::Deserialize;

/// Coin balance within the wallet.
#[derive(Debug, Clone, Deserialize)]
pub struct CoinBalance {
    /// Coin name (e.g. "USDT", "BTC")
    #[serde(rename = "coin")]
    #[serde(default)]
    pub coin: Option<String>,
    /// Equity (total value in this coin)
    #[serde(rename = "equity")]
    #[serde(default)]
    pub equity: Option<String>,
    /// Wallet balance
    #[serde(rename = "walletBalance")]
    #[serde(default)]
    pub wallet_balance: Option<String>,
    /// Position margin
    #[serde(rename = "positionMargin")]
    #[serde(default)]
    pub position_margin: Option<String>,
    /// Available balance
    #[serde(rename = "availableToWithdraw")]
    #[serde(default)]
    pub available_to_withdraw: Option<String>,
    /// Available to trade (order margin)
    #[serde(rename = "availableToTrade")]
    #[serde(default)]
    pub available_to_trade: Option<String>,
    /// Unrealised PnL
    #[serde(rename = "unrealisedPnl")]
    #[serde(default)]
    pub unrealised_pnl: Option<String>,
    /// Cumulative realised PnL
    #[serde(rename = "cumRealisedPnl")]
    #[serde(default)]
    pub cum_realised_pnl: Option<String>,
    /// Bonus
    #[serde(rename = "bonus")]
    #[serde(default)]
    pub bonus: Option<String>,
    /// Whether this is a borrow coin (margin)
    #[serde(rename = "borrowAmount")]
    #[serde(default)]
    pub borrow_amount: Option<String>,
}

/// Wallet data from private WebSocket.
#[derive(Debug, Clone, Deserialize)]
pub struct WalletData {
    /// Account type
    #[serde(rename = "accountType")]
    #[serde(default)]
    pub account_type: Option<String>,
    /// Account LTV (Loan-to-Value)
    #[serde(rename = "accountLTV")]
    #[serde(default)]
    pub account_ltv: Option<String>,
    /// Account IM rate
    #[serde(rename = "accountIMRate")]
    #[serde(default)]
    pub account_im_rate: Option<String>,
    /// Account MM rate
    #[serde(rename = "accountMMRate")]
    #[serde(default)]
    pub account_mm_rate: Option<String>,
    /// Total equity (USD value)
    #[serde(rename = "totalEquity")]
    #[serde(default)]
    pub total_equity: Option<String>,
    /// Total wallet balance
    #[serde(rename = "totalWalletBalance")]
    #[serde(default)]
    pub total_wallet_balance: Option<String>,
    /// Total margin balance
    #[serde(rename = "totalMarginBalance")]
    #[serde(default)]
    pub total_margin_balance: Option<String>,
    /// Total available balance
    #[serde(rename = "totalAvailableBalance")]
    #[serde(default)]
    pub total_available_balance: Option<String>,
    /// Total perpetual PnL (Unified account)
    #[serde(rename = "totalPerpUPL")]
    #[serde(default)]
    pub total_perp_upl: Option<String>,
    /// Total initial margin
    #[serde(rename = "totalInitialMargin")]
    #[serde(default)]
    pub total_initial_margin: Option<String>,
    /// Total maintenance margin
    #[serde(rename = "totalMaintenanceMargin")]
    #[serde(default)]
    pub total_maintenance_margin: Option<String>,
    /// Coin balances per asset
    #[serde(rename = "coin")]
    #[serde(default)]
    pub coins: Vec<CoinBalance>,
    /// Timestamp
    #[serde(rename = "time")]
    #[serde(default)]
    pub time: Option<String>,
    /// Category
    #[serde(rename = "category")]
    #[serde(default)]
    pub category: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_wallet() {
        let json = serde_json::json!({
            "accountType": "UNIFIED",
            "totalEquity": "10000.00",
            "totalWalletBalance": "10000.00",
            "coin": [
                {
                    "coin": "USDT",
                    "equity": "10000.00",
                    "walletBalance": "10000.00",
                    "availableToWithdraw": "8000.00",
                    "availableToTrade": "9000.00"
                }
            ],
            "category": "linear"
        });

        let wallet: WalletData = serde_json::from_value(json).unwrap();
        assert_eq!(wallet.account_type.as_deref(), Some("UNIFIED"));
        assert_eq!(wallet.coins.len(), 1);
        assert_eq!(wallet.coins[0].coin.as_deref(), Some("USDT"));
    }
}
