use std::fmt::{Display, Formatter, Result};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum RejectReason {
    #[serde(rename = "EC_NoError")]
    NoError,
    #[serde(rename = "EC_Others")]
    Others,
    #[serde(rename = "EC_UnknownMessageType")]
    UnknownMessageType,
    #[serde(rename = "EC_MissingClOrdID")]
    MissingClOrdID,
    #[serde(rename = "EC_MissingOrigClOrdID")]
    MissingOrigClOrdID,
    #[serde(rename = "EC_ClOrdIDOrigClOrdIDAreTheSame")]
    ClOrdIDOrigClOrdIDAreTheSame,
    #[serde(rename = "EC_DuplicatedClOrdID")]
    DuplicatedClOrdID,
    #[serde(rename = "EC_OrigClOrdIDDoesNotExist")]
    OrigClOrdIDDoesNotExist,
    #[serde(rename = "EC_TooLateToCancel")]
    TooLateToCancel,
    #[serde(rename = "EC_UnknownOrderType")]
    UnknownOrderType,
    #[serde(rename = "EC_UnknownSide")]
    UnknownSide,
    #[serde(rename = "EC_UnknownTimeInForce")]
    UnknownTimeInForce,
    #[serde(rename = "EC_WronglyRouted")]
    WronglyRouted,
    #[serde(rename = "EC_MarketOrderPriceIsNotZero")]
    MarketOrderPriceIsNotZero,
    #[serde(rename = "EC_LimitOrderInvalidPrice")]
    LimitOrderInvalidPrice,
    #[serde(rename = "EC_NoEnoughQtyToFill")]
    NoEnoughQtyToFill,
    #[serde(rename = "EC_NoImmediateQtyToFill")]
    NoImmediateQtyToFill,
    #[serde(rename = "EC_PerCancelRequest")]
    PerCancelRequest,
    #[serde(rename = "EC_MarketOrderCannotBePostOnly")]
    MarketOrderCannotBePostOnly,
    #[serde(rename = "EC_PostOnlyWillTakeLiquidity")]
    PostOnlyWillTakeLiquidity,
    #[serde(rename = "EC_CancelReplaceOrder")]
    CancelReplaceOrder,
    #[serde(rename = "EC_InvalidSymbolStatus")]
    InvalidSymbolStatus,
    #[serde(rename = "EC_CancelForNoFullFill")]
    CancelForNoFullFill,
}

impl Display for RejectReason {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            RejectReason::NoError => write!(f, "EC_NoError"),
            RejectReason::Others => write!(f, "EC_Others"),
            RejectReason::UnknownMessageType => write!(f, "EC_UnknownMessageType"),
            RejectReason::MissingClOrdID => write!(f, "EC_MissingClOrdID"),
            RejectReason::MissingOrigClOrdID => write!(f, "EC_MissingOrigClOrdID"),
            RejectReason::ClOrdIDOrigClOrdIDAreTheSame => {
                write!(f, "EC_ClOrdIDOrigClOrdIDAreTheSame")
            }
            RejectReason::DuplicatedClOrdID => write!(f, "EC_DuplicatedClOrdID"),
            RejectReason::OrigClOrdIDDoesNotExist => write!(f, "EC_OrigClOrdIDDoesNotExist"),
            RejectReason::TooLateToCancel => write!(f, "EC_TooLateToCancel"),
            RejectReason::UnknownOrderType => write!(f, "EC_UnknownOrderType"),
            RejectReason::UnknownSide => write!(f, "EC_UnknownSide"),
            RejectReason::UnknownTimeInForce => write!(f, "EC_UnknownTimeInForce"),
            RejectReason::WronglyRouted => write!(f, "EC_WronglyRouted"),
            RejectReason::MarketOrderPriceIsNotZero => write!(f, "EC_MarketOrderPriceIsNotZero"),
            RejectReason::LimitOrderInvalidPrice => write!(f, "EC_LimitOrderInvalidPrice"),
            RejectReason::NoEnoughQtyToFill => write!(f, "EC_NoEnoughQtyToFill"),
            RejectReason::NoImmediateQtyToFill => write!(f, "EC_NoImmediateQtyToFill"),
            RejectReason::PerCancelRequest => write!(f, "EC_PerCancelRequest"),
            RejectReason::MarketOrderCannotBePostOnly => {
                write!(f, "EC_MarketOrderCannotBePostOnly")
            }
            RejectReason::PostOnlyWillTakeLiquidity => write!(f, "EC_PostOnlyWillTakeLiquidity"),
            RejectReason::CancelReplaceOrder => write!(f, "EC_CancelReplaceOrder"),
            RejectReason::InvalidSymbolStatus => write!(f, "EC_InvalidSymbolStatus"),
            RejectReason::CancelForNoFullFill => write!(f, "EC_CancelForNoFullFill"),
        }
    }
}
