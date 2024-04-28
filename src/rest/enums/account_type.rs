use std::fmt::{Display, Formatter, Result};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum AccountType {
    // Check the value of unifiedMarginStatus to see if you're on the Unified Trading Account or the classic account.

    // Unified Trading Account (UTA)
    #[serde(rename = "CONTRACT")]
    CONTRACT_UTA, // Inverse Derivatives Account
    #[serde(rename = "UNIFIED")]
    UNIFIED, // Unified Trading Account
    #[serde(rename = "FUND")]
    FUND_UTA, // Funding Account

    #[serde(rename = "SPOT")]
    SPOT, // Spot Account
    #[serde(rename = "OPTION")]
    OPTION, // USDC Derivatives
    #[serde(rename = "CONTRACT")]
    CONTRACT_CLASSIC, // Derivatives Account
    #[serde(rename = "FUND")]
    FUND_CLASSIC, // Funding Account
}

impl Display for AccountType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            AccountType::CONTRACT_UTA => write!(f, "CONTRACT"),
            AccountType::UNIFIED => write!(f, "UNIFIED"),
            AccountType::FUND_UTA => write!(f, "FUND"),
            AccountType::SPOT => write!(f, "SPOT"),
            AccountType::OPTION => write!(f, "OPTION"),
            AccountType::CONTRACT_CLASSIC => write!(f, "CONTRACT"),
            AccountType::FUND_CLASSIC => write!(f, "FUND"),
        }
    }
}
