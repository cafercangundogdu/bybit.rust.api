use std::fmt::{Display, Formatter, Result};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Status {
    #[serde(rename = "PreLaunch")]
    PreLaunch,
    #[serde(rename = "Trading")]
    Trading,
    #[serde(rename = "Settling")]
    Settling, // The unique status for USDC Perpetual 8-hour settlement
    #[serde(rename = "Delivering")]
    Delivering,
    #[serde(rename = "Closed")]
    Closed,
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Status::PreLaunch => write!(f, "PreLaunch"),
            Status::Trading => write!(f, "Trading"),
            Status::Settling => write!(f, "Settling"),
            Status::Delivering => write!(f, "Delivering"),
            Status::Closed => write!(f, "Closed"),
        }
    }
}
