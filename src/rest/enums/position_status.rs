use std::fmt::{Display, Formatter, Result};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum PositionStatus {
    #[serde(rename = "Normal")]
    Normal,
    #[serde(rename = "Liq")]
    Liq, // in the liquidation progress
    #[serde(rename = "Adl")]
    Adl, // in the auto-deleverage progress
}

impl Display for PositionStatus {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            PositionStatus::Normal => write!(f, "Normal"),
            PositionStatus::Liq => write!(f, "Liq"),
            PositionStatus::Adl => write!(f, "Adl"),
        }
    }
}
