use std::fmt::{Display, Formatter, Result};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum AccountType {
    // Check the value of unifiedMarginStatus to see if you're on the Unified Trading Account or the classic account.

    #[serde(rename = "UNIFIED")]
    UNIFIED, // Unified Trading Account
    #[serde(rename = "CONTRACT")]
    CONTRACT, // Contract/Derivatives Account
    #[serde(rename = "SPOT")]
    SPOT, // Spot Account
    #[serde(rename = "OPTION")]
    OPTION, // USDC Derivatives
    #[serde(rename = "FUND")]
    FUND, // Funding Account
}

impl Display for AccountType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            AccountType::UNIFIED => write!(f, "UNIFIED"),
            AccountType::CONTRACT => write!(f, "CONTRACT"),
            AccountType::SPOT => write!(f, "SPOT"),
            AccountType::OPTION => write!(f, "OPTION"),
            AccountType::FUND => write!(f, "FUND"),
        }
    }
}
