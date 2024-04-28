use std::fmt::{Display, Formatter, Result};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum LtStatus {
    #[serde(rename = "1")]
    PurchaseAndRedeem, // LT can be purchased and redeemed
    #[serde(rename = "2")]
    PurchaseNotRedeem, // LT can be purchased, but not redeemed
    #[serde(rename = "3")]
    RedeemNotPurchase, // LT can be redeemed, but not purchased
    #[serde(rename = "4")]
    NotPurchaseRedeem, // LT cannot be purchased nor redeemed
    #[serde(rename = "5")]
    AdjustingPosition, // Adjusting position
}

impl Display for LtStatus {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            LtStatus::PurchaseAndRedeem => write!(f, "1"),
            LtStatus::PurchaseNotRedeem => write!(f, "2"),
            LtStatus::RedeemNotPurchase => write!(f, "3"),
            LtStatus::NotPurchaseRedeem => write!(f, "4"),
            LtStatus::AdjustingPosition => write!(f, "5"),
        }
    }
}
