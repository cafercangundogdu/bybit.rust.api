use std::fmt::{Display, Formatter, Result};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy, Default)]
pub enum TimeInForce {
    // https://bybit-exchange.github.io/docs/v5/enum#timeinforce
    #[default]
    #[serde(rename = "GTC")]
    GTC, // GoodTillCancel
    #[serde(rename = "IOC")]
    IOC, // ImmediateOrCancel
    #[serde(rename = "FOK")]
    FOK, // FillOrKill
}

impl Display for TimeInForce {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            TimeInForce::GTC => write!(f, "GTC"),
            TimeInForce::IOC => write!(f, "IOC"),
            TimeInForce::FOK => write!(f, "FOK"),
        }
    }
}
