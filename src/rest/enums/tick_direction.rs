use std::fmt::{Display, Formatter, Result};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum TickDirection {
    // https://bybit-exchange.github.io/docs/v5/enum#tickdirection
    #[serde(rename = "PlusTick")]
    PlusTick, // price rise
    #[serde(rename = "ZeroPlusTick")]
    ZeroPlusTick, // trade occurs at the same price as the previous trade, which occurred at a price higher than that for the trade preceding it
    #[serde(rename = "MinusTick")]
    MinusTick, // price drop
    #[serde(rename = "ZeroMinusTick")]
    ZeroMinusTick, // trade occurs at the same price as the previous trade, which occurred at a price lower than that for the trade preceding it
}

impl Display for TickDirection {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            TickDirection::PlusTick => write!(f, "PlusTick"),
            TickDirection::ZeroPlusTick => write!(f, "ZeroPlusTick"),
            TickDirection::MinusTick => write!(f, "MinusTick"),
            TickDirection::ZeroMinusTick => write!(f, "ZeroMinusTick"),
        }
    }
}
