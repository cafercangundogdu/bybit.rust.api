use std::fmt::{Display, Formatter, Result};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum SmpType {
    #[serde(rename = "None")]
    None,
    #[serde(rename = "CancelMaker")]
    CancelMaker,
    #[serde(rename = "CancelTaker")]
    CancelTaker,
    #[serde(rename = "CancelBoth")]
    CancelBoth,
}

impl Display for SmpType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            SmpType::None => write!(f, "None"),
            SmpType::CancelMaker => write!(f, "CancelMaker"),
            SmpType::CancelTaker => write!(f, "CancelTaker"),
            SmpType::CancelBoth => write!(f, "CancelBoth"),
        }
    }
}
