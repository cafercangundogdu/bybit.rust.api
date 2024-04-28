use std::fmt::{Display, Formatter, Result};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum TransferStatus {
    #[serde(rename = "SUCCESS")]
    SUCCESS, // SUCCESS
    #[serde(rename = "PENDING")]
    PENDING, // PENDING
    #[serde(rename = "FAILED")]
    FAILED, // FAILED
}

impl Display for TransferStatus {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            TransferStatus::SUCCESS => write!(f, "SUCCESS"),
            TransferStatus::PENDING => write!(f, "PENDING"),
            TransferStatus::FAILED => write!(f, "FAILED"),
        }
    }
}
