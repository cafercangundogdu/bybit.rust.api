use std::fmt::{Display, Formatter, Result};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum DepositStatus {
    #[serde(rename = "0")]
    Unknown, // unknown
    #[serde(rename = "1")]
    ToBeConfirmed, // toBeConfirmed
    #[serde(rename = "2")]
    Processing, // processing
    #[serde(rename = "3")]
    Success, // success (finalised status of a success deposit)
    #[serde(rename = "4")]
    DepositFailed, // deposit failed
    #[serde(rename = "10011")]
    PendingToBeCreditedToFundingPool, // pending to be credited to funding pool
    #[serde(rename = "10012")]
    CreditedToFundingPoolSuccessfully, // Credited to funding pool successfully
}

impl Display for DepositStatus {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            DepositStatus::Unknown => write!(f, "0"),
            DepositStatus::ToBeConfirmed => write!(f, "1"),
            DepositStatus::Processing => write!(f, "2"),
            DepositStatus::Success => write!(f, "3"),
            DepositStatus::DepositFailed => write!(f, "4"),
            DepositStatus::PendingToBeCreditedToFundingPool => write!(f, "10011"),
            DepositStatus::CreditedToFundingPoolSuccessfully => write!(f, "10012"),
        }
    }
}
