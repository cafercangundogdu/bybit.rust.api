use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};

// https://bybit-exchange.github.io/docs/v5/enum#optionperiod
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum OptionPeriod {
    #[serde(rename = "7")] // valid for BTC, ETH, SOL
    SevenDay, // 7 days
    #[serde(rename = "14")] // valid for BTC, ETH, SOL
    FourteenDay, // 14 days
    #[serde(rename = "30")] // valid for BTC, ETH, SOL
    ThirtyDay, // 30 days
    #[serde(rename = "60")] // valid for BTC, ETH, SOL
    SixtyDay, // 60 days
    #[serde(rename = "90")] // valid for BTC, ETH, SOL
    NinetyDay, // 90 days
    #[serde(rename = "180")] // valid for BTC, ETH
    OneHundredEightyDay, // 180 days
    #[serde(rename = "270")] // valid for BTC, ETH
    TwoHundredSeventyDay, // 270 days
}

impl Display for OptionPeriod {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            OptionPeriod::SevenDay => write!(f, "7"),
            OptionPeriod::FourteenDay => write!(f, "14"),
            OptionPeriod::ThirtyDay => write!(f, "30"),
            OptionPeriod::SixtyDay => write!(f, "60"),
            OptionPeriod::NinetyDay => write!(f, "90"),
            OptionPeriod::OneHundredEightyDay => write!(f, "180"),
            OptionPeriod::TwoHundredSeventyDay => write!(f, "270"),
        }
    }
}
