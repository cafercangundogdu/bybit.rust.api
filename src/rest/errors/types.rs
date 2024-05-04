use std::fmt::{Display, Formatter, Result};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum ErrorTypes {
    #[serde(rename = "http")]
    Http, // HTTP error
    #[serde(rename = "UTAClassicAccount")]
    UTAClassicAccount,
    #[serde(rename = "SpotTrade")]
    SpotTrade,
    #[serde(rename = "SpotLeverageToken")]
    SpotLeverageToken,
    #[serde(rename = "SpotMarginTrade")]
    SpotMarginTrade,
    #[serde(rename = "Asset")]
    Asset,
    #[serde(rename = "InstitutionalLoan")]
    InstitutionalLoan,
    #[serde(rename = "Broker")]
    Broker,
}

impl Display for ErrorTypes {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            ErrorTypes::Http => write!(f, "http"),
            ErrorTypes::UTAClassicAccount => write!(f, "UTAClassicAccount"),
            ErrorTypes::SpotTrade => write!(f, "SpotTrade"),
            ErrorTypes::SpotLeverageToken => write!(f, "SpotLeverageToken"),
            ErrorTypes::SpotMarginTrade => write!(f, "SpotMarginTrade"),
            ErrorTypes::Asset => write!(f, "Asset"),
            ErrorTypes::InstitutionalLoan => write!(f, "InstitutionalLoan"),
            ErrorTypes::Broker => write!(f, "Broker"),
        }
    }
}
