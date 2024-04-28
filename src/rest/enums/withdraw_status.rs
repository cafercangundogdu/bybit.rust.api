use std::fmt::{Display, Formatter, Result};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum WithdrawStatus {
    #[serde(rename = "SecurityCheck")]
    SecurityCheck,
    #[serde(rename = "Pending")]
    Pending,
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "CancelByUser")]
    CancelByUser,
    #[serde(rename = "Reject")]
    Reject,
    #[serde(rename = "Fail")]
    Fail,
    #[serde(rename = "BlockchainConfirmed")]
    BlockchainConfirmed,
    #[serde(rename = "MoreInformationRequired")]
    MoreInformationRequired,
    #[serde(rename = "Unknown")]
    UNKNOWN, // a rare status
}

impl Display for WithdrawStatus {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            WithdrawStatus::SecurityCheck => write!(f, "SecurityCheck"),
            WithdrawStatus::Pending => write!(f, "Pending"),
            WithdrawStatus::Success => write!(f, "success"),
            WithdrawStatus::CancelByUser => write!(f, "CancelByUser"),
            WithdrawStatus::Reject => write!(f, "Reject"),
            WithdrawStatus::Fail => write!(f, "Fail"),
            WithdrawStatus::BlockchainConfirmed => write!(f, "BlockchainConfirmed"),
            WithdrawStatus::MoreInformationRequired => write!(f, "MoreInformationRequired"),
            WithdrawStatus::UNKNOWN => write!(f, "Unknown"),
        }
    }
}
