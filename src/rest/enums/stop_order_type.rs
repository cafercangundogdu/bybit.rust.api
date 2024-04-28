use std::fmt::{Display, Formatter, Result};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum StopOrderType {
    // https://bybit-exchange.github.io/docs/v5/enum#stopordertype
    #[serde(rename = "TakeProfit")]
    TakeProfit,
    #[serde(rename = "StopLoss")]
    StopLoss,
    #[serde(rename = "TrailingStop")]
    TrailingStop,
    #[serde(rename = "Stop")]
    Stop,
    #[serde(rename = "PartialTakeProfit")]
    PartialTakeProfit,
    #[serde(rename = "PartialStopLoss")]
    PartialStopLoss,
    #[serde(rename = "tpslOrder")]
    TpslOrder, // spot TP/SL order
    #[serde(rename = "OcoOrder")]
    OcoOrder, // spot Oco order
    #[serde(rename = "MmRateClose")]
    MmRateClose, // On web or app can set MMR to close position
    #[serde(rename = "BidirectionalTpslOrder")]
    BidirectionalTpslOrder, // Spot bidirectional tpsl order
}

impl Display for StopOrderType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            StopOrderType::TakeProfit => write!(f, "TakeProfit"),
            StopOrderType::StopLoss => write!(f, "StopLoss"),
            StopOrderType::TrailingStop => write!(f, "TrailingStop"),
            StopOrderType::Stop => write!(f, "Stop"),
            StopOrderType::PartialTakeProfit => write!(f, "PartialTakeProfit"),
            StopOrderType::PartialStopLoss => write!(f, "PartialStopLoss"),
            StopOrderType::TpslOrder => write!(f, "tpslOrder"),
            StopOrderType::OcoOrder => write!(f, "OcoOrder"),
            StopOrderType::MmRateClose => write!(f, "MmRateClose"),
            StopOrderType::BidirectionalTpslOrder => write!(f, "BidirectionalTpslOrder"),
        }
    }
}
