use std::fmt::{Display, Formatter, Result};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum AdlRankIndicator {
    #[serde(rename = "0")]
    Adl0,
    #[serde(rename = "1")]
    Adl1,
    #[serde(rename = "2")]
    Adl2,
    #[serde(rename = "3")]
    Adl3,
    #[serde(rename = "4")]
    Adl4,
    #[serde(rename = "5")]
    Adl5,
}

impl Display for AdlRankIndicator {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            AdlRankIndicator::Adl0 => write!(f, "0"),
            AdlRankIndicator::Adl1 => write!(f, "1"),
            AdlRankIndicator::Adl2 => write!(f, "2"),
            AdlRankIndicator::Adl3 => write!(f, "3"),
            AdlRankIndicator::Adl4 => write!(f, "4"),
            AdlRankIndicator::Adl5 => write!(f, "5"),
        }
    }
}
