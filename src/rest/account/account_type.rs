use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum AccountType {
    // UTA: Unified Trading Account
    // https://bybit-exchange.github.io/docs/v5/enum#unified-trading-account-uta
    #[serde(rename = "UNIFIED")]
    UTAUnified, // Unified Trading Account
    #[serde(rename = "CONTRACT")]
    UTAContract, // Inverse Derivatives Account
    #[serde(rename = "FUND")]
    UTAFund, // Funding Account

    // Classic Account
    // https://bybit-exchange.github.io/docs/v5/enum#classic-account
    #[serde(rename = "SPOT")]
    CLSSpot, // Spot Account
    #[serde(rename = "CONTRACT")]
    CLSContract, // Derivatives Account
    #[serde(rename = "OPTION")]
    CLSOption, // USDC Derivatives
    #[serde(rename = "FUND")]
    CLSFund, // Funding Account
}

impl Display for AccountType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            AccountType::UTAUnified => write!(f, "UNIFIED"),
            AccountType::UTAContract => write!(f, "CONTRACT"),
            AccountType::UTAFund => write!(f, "FUND"),
            AccountType::CLSSpot => write!(f, "SPOT"),
            AccountType::CLSContract => write!(f, "CONTRACT"),
            AccountType::CLSOption => write!(f, "OPTION"),
            AccountType::CLSFund => write!(f, "FUND"),
        }
    }
}
