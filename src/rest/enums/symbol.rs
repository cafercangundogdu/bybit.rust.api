use std::fmt::{Display, Formatter, Result};

use serde::{Deserialize, Serialize};

use crate::rest::enums::symbol_type::SymbolType;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Symbol {
    pub symbol: String,
    pub symbol_type: SymbolType,
}

impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.symbol)
    }
}

impl Symbol {
    pub fn new(symbol: String, symbol_type: SymbolType) -> Self {
        Symbol {
            symbol,
            symbol_type,
        }
    }

    pub fn is_type(&self, symbol_type: SymbolType) -> bool {
        self.symbol_type == symbol_type
    }
    pub fn is_usdt_perpetual(&self) -> bool {
        self.symbol_type == SymbolType::USDTPerpetual
    }
    pub fn is_usdc_perpetual(&self) -> bool {
        self.symbol_type == SymbolType::USDCPerpetual
    }
    pub fn is_usdc_futures(&self) -> bool {
        self.symbol_type == SymbolType::USDCFutures
    }
    pub fn is_inverse_perpetual(&self) -> bool {
        self.symbol_type == SymbolType::InversePerpetual
    }
    pub fn is_inverse_futures(&self) -> bool {
        self.symbol_type == SymbolType::InverseFutures
    }
    pub fn is_spot(&self) -> bool {
        self.symbol_type == SymbolType::Spot
    }
}
