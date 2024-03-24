use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Category {
    // https://bybit-exchange.github.io/docs/v5/enum#category

    // Unified Account
    #[serde(rename = "spot")]
    UTASpot, // UTA Spot Account
    #[serde(rename = "linear")]
    UTALinear, // USDT perpetual, and USDC contract, including USDC perp, USDC futures
    #[serde(rename = "inverse")]
    UTAInverse, // Inverse contract, including Inverse perp, Inverse futures
    #[serde(rename = "option")]
    UTAOption, // UTA Option Account

    // Classic Account
    #[serde(rename = "linear")]
    CLSLinear, // USDT perp
    #[serde(rename = "inverse")]
    CLSInverse, // Inverse contract, including Inverse perp, Inverse futures
    #[serde(rename = "spot")]
    CLSSpot, // Classic Spot Account
}

impl Display for Category {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Category::UTASpot => write!(f, "spot"),
            Category::UTALinear => write!(f, "linear"),
            Category::UTAInverse => write!(f, "inverse"),
            Category::UTAOption => write!(f, "option"),
            Category::CLSLinear => write!(f, "linear"),
            Category::CLSInverse => write!(f, "inverse"),
            Category::CLSSpot => write!(f, "spot"),
        }
    }
}
