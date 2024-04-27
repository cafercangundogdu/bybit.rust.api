use std::fmt::{Display, Formatter, Result};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum OrderStatus {
    // https://bybit-exchange.github.io/docs/v5/enum#orderstatus

    // Open status
    #[serde(rename = "New")]
    New, // order has been placed successfully
    #[serde(rename = "PartiallyFilled")]
    PartiallyFilled,
    #[serde(rename = "Untriggered")]
    Untriggered, // Conditional orders are created

    // Closed status
    #[serde(rename = "PartiallyFilledCanceled")]
    PartiallyFilledCanceled, // Only spot has this order status
    #[serde(rename = "Filled")]
    Filled,
    #[serde(rename = "Cancelled")]
    Cancelled, // In derivatives, orders with this status may have an executed qty
    #[serde(rename = "Triggered")]
    Triggered, // instantaneous state for conditional orders from Untriggered to New
    #[serde(rename = "Deactivated")]
    Deactivated, // UTA: Spot tp/sl order, conditional order, OCO order are cancelled before they are triggered
}

impl Display for OrderStatus {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            OrderStatus::New => write!(f, "New"),
            OrderStatus::PartiallyFilled => write!(f, "PartiallyFilled"),
            OrderStatus::Untriggered => write!(f, "Untriggered"),
            OrderStatus::PartiallyFilledCanceled => write!(f, "PartiallyFilledCanceled"),
            OrderStatus::Filled => write!(f, "Filled"),
            OrderStatus::Cancelled => write!(f, "Cancelled"),
            OrderStatus::Triggered => write!(f, "Triggered"),
            OrderStatus::Deactivated => write!(f, "Deactivated"),
        }
    }
}

impl OrderStatus {
    pub fn is_open(&self) -> bool {
        match self {
            OrderStatus::New => true,
            OrderStatus::PartiallyFilled => true,
            OrderStatus::Untriggered => true,
            OrderStatus::PartiallyFilledCanceled => true,
            OrderStatus::Filled => false,
            OrderStatus::Cancelled => false,
            OrderStatus::Triggered => false,
            OrderStatus::Deactivated => false,
        }
    }

    pub fn is_closed(&self) -> bool {
        !self.is_open()
    }
}
