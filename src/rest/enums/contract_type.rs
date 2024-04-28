use std::fmt::{Display, Formatter, Result};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum ContractType {
    #[serde(rename = "InversePerpetual")]
    InversePerpetual,
    #[serde(rename = "LinearPerpetual")]
    LinearPerpetual,
    #[serde(rename = "LinearFutures")]
    LinearFutures, // USDC Futures
    #[serde(rename = "InverseFutures")]
    InverseFutures,
}

impl Display for ContractType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            ContractType::InversePerpetual => write!(f, "InversePerpetual"),
            ContractType::LinearPerpetual => write!(f, "LinearPerpetual"),
            ContractType::LinearFutures => write!(f, "LinearFutures"),
            ContractType::InverseFutures => write!(f, "InverseFutures"),
        }
    }
}
