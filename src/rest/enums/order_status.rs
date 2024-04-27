use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum OrderStatus {
    // https://bybit-exchange.github.io/docs/v5/enum#orderstatus
    #[serde(rename = "New")]
    New, // order has been placed successfully
    #[serde(rename = "PartiallyFilled")]
    PartiallyFilled,
    #[serde(rename = "Untriggered")]
    Untriggered, // Conditional orders are created
}

impl Display for OrderStatus {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            OrderStatus::New => write!(f, "New"),
            OrderStatus::PartiallyFilled => write!(f, "PartiallyFilled"),
            OrderStatus::Untriggered => write!(f, "Untriggered"),
        }
    }
}
