use std::fmt::{Display, Formatter, Result};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum MarginTrading {
    #[serde(rename = "none")]
    None, // Regardless of normal account or UTA account, this trading pair does not support margin trading
    #[serde(rename = "both")]
    Both, // For both normal account and UTA account, this trading pair supports margin trading
    #[serde(rename = "utaOnly")]
    UtaOnly, // Only for UTA account,this trading pair supports margin trading
    #[serde(rename = "normalSpotOnly")]
    NormalSpotOnly, // Only for normal account, this trading pair supports margin trading
}

impl Display for MarginTrading {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            MarginTrading::None => write!(f, "none"),
            MarginTrading::Both => write!(f, "both"),
            MarginTrading::UtaOnly => write!(f, "utaOnly"),
            MarginTrading::NormalSpotOnly => write!(f, "normalSpotOnly"),
        }
    }
}
