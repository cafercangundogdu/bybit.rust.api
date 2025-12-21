use std::fmt::{Display, Formatter, Result};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy, Default)]
pub enum TriggerBy {
    #[default]
    #[serde(rename = "LastPrice")]
    LastPrice,
    #[serde(rename = "IndexPrice")]
    IndexPrice,
    #[serde(rename = "MarkPrice")]
    MarkPrice,
}

impl Display for TriggerBy {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            TriggerBy::LastPrice => write!(f, "LastPrice"),
            TriggerBy::IndexPrice => write!(f, "IndexPrice"),
            TriggerBy::MarkPrice => write!(f, "MarkPrice"),
        }
    }
}
