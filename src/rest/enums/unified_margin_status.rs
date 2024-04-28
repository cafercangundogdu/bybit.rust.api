use std::fmt::{Display, Formatter, Result};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum UnifiedMarginStatus {
    #[serde(rename = "1")]
    RegularAccount,
    #[serde(rename = "2")]
    PleaseIgnore,
    #[serde(rename = "3")]
    UnifiedTradeAccount,
    #[serde(rename = "4")]
    UtaPro,
}

impl Display for UnifiedMarginStatus {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            UnifiedMarginStatus::RegularAccount => write!(f, "1"),
            UnifiedMarginStatus::PleaseIgnore => write!(f, "2"),
            UnifiedMarginStatus::UnifiedTradeAccount => write!(f, "3"),
            UnifiedMarginStatus::UtaPro => write!(f, "4"),
        }
    }
}
