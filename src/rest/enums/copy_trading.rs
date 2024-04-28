use std::fmt::{Display, Formatter, Result};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum CopyTrading {
    #[serde(rename = "none")]
    None, // Regardless of normal account or UTA account, this trading pair does not support copy trading
    #[serde(rename = "both")]
    Both, // For both normal account and UTA account, this trading pair supports copy trading
    #[serde(rename = "utaOnly")]
    UtaOnly, // Only for UTA account,this trading pair supports copy trading
    #[serde(rename = "normalSpotOnly")]
    NormalSpotOnly, // Only for normal account, this trading pair supports copy trading
}

impl Display for CopyTrading {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            CopyTrading::None => write!(f, "none"),
            CopyTrading::Both => write!(f, "both"),
            CopyTrading::UtaOnly => write!(f, "utaOnly"),
            CopyTrading::NormalSpotOnly => write!(f, "normalSpotOnly"),
        }
    }
}
