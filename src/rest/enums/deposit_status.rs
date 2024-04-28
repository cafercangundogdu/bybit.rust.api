use std::fmt::{Display, Formatter, Result};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum DepositStatus {
    #[serde(rename = "0")]
    UNKNOWN, // unknown
    #[serde(rename = "1")]
    TO_BE_CONFIRMED, // toBeConfirmed
    #[serde(rename = "2")]
    PROCESSING, // processing
    #[serde(rename = "3")]
    SUCCESS, // success (finalised status of a success deposit)
    #[serde(rename = "4")]
    DEPOSIT_FAILED, // deposit failed
    #[serde(rename = "10011")]
    PENDING_TO_BE_CREDITED_TO_FUNDING_POOL, // pending to be credited to funding pool
    #[serde(rename = "10012")]
    CREDITED_TO_FUNDING_POOL_SUCCESSFULLY, // Credited to funding pool successfully
}

impl Display for DepositStatus {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            DepositStatus::UNKNOWN => write!(f, "0"),
            DepositStatus::TO_BE_CONFIRMED => write!(f, "1"),
            DepositStatus::PROCESSING => write!(f, "2"),
            DepositStatus::SUCCESS => write!(f, "3"),
            DepositStatus::DEPOSIT_FAILED => write!(f, "4"),
            DepositStatus::PENDING_TO_BE_CREDITED_TO_FUNDING_POOL => write!(f, "10011"),
            DepositStatus::CREDITED_TO_FUNDING_POOL_SUCCESSFULLY => write!(f, "10012"),
        }
    }
}
