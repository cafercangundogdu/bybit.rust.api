use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy, Default)]
pub enum Side {
    #[default]
    #[serde(rename = "Buy")]
    Buy, // Buy side
    #[serde(rename = "Sell")]
    Sell, // Sell side
}

impl Display for Side {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Side::Buy => write!(f, "Buy"),
            Side::Sell => write!(f, "Sell"),
        }
    }
}
