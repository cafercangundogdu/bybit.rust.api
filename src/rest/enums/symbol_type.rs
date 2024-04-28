use std::fmt::{Display, Formatter, Result};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum SymbolType {
    USDTPerpetual,
    USDCPerpetual,
    USDCFutures,
    InversePerpetual,
    InverseFutures,
    Spot,
}

impl Display for SymbolType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            SymbolType::USDTPerpetual => write!(f, "USDTPerpetual"),
            SymbolType::USDCPerpetual => write!(f, "USDCPerpetual"),
            SymbolType::USDCFutures => write!(f, "USDCFutures"),
            SymbolType::InversePerpetual => write!(f, "InversePerpetual"),
            SymbolType::InverseFutures => write!(f, "InverseFutures"),
            SymbolType::Spot => write!(f, "Spot"),
        }
    }
}
