use std::fmt::{Display, Formatter, Result};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum PositionIdx {
    #[serde(rename = "0")]
    OneWayMode, // one-way mode position
    #[serde(rename = "1")]
    BuySideHedge, // Buy side of hedge-mode position
    #[serde(rename = "2")]
    SellSideHedge, // Sell side of hedge-mode position
}

impl Display for PositionIdx {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            PositionIdx::OneWayMode => write!(f, "0"),
            PositionIdx::BuySideHedge => write!(f, "1"),
            PositionIdx::SellSideHedge => write!(f, "2"),
        }
    }
}
