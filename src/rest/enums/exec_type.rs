use std::fmt::{Display, Formatter, Result};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum ExecType {
    // https://bybit-exchange.github.io/docs/v5/enum#exectype
    #[serde(rename = "Trade")]
    Trade,
    #[serde(rename = "AdlTrade")]
    AdlTrade, // Auto-Deleveraging
    #[serde(rename = "Funding")]
    Funding, // Funding fee
    #[serde(rename = "BustTrade")]
    BustTrade, // Liquidation
    #[serde(rename = "Delivery")]
    Delivery, // USDC futures delivery
    #[serde(rename = "Settle")]
    Settle, // Inverse futures settlement; Position closed due to delisting
    #[serde(rename = "BlockTrade")]
    BlockTrade,
    #[serde(rename = "MovePosition")]
    MovePosition,
    #[serde(rename = "UNKNOWN")]
    UNKNOWN, // May be returned by a classic account. Cannot query by this type
}

impl Display for ExecType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            ExecType::Trade => write!(f, "Trade"),
            ExecType::AdlTrade => write!(f, "AdlTrade"),
            ExecType::Funding => write!(f, "Funding"),
            ExecType::BustTrade => write!(f, "BustTrade"),
            ExecType::Delivery => write!(f, "Delivery"),
            ExecType::Settle => write!(f, "Settle"),
            ExecType::BlockTrade => write!(f, "BlockTrade"),
            ExecType::MovePosition => write!(f, "MovePosition"),
            ExecType::UNKNOWN => write!(f, "UNKNOWN"),
        }
    }
}
