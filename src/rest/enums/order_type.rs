use std::fmt::{Display, Formatter, Result};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum OrderType {
    // https://bybit-exchange.github.io/docs/v5/enum#ordertype
    #[serde(rename = "Market")]
    Market,
    #[serde(rename = "Limit")]
    Limit,
    #[serde(rename = "UNKNOWN")]
    Unknown, // is not a valid request parameter value. Is only used in some responses. Mainly, it is used when execType is Funding.
}

impl Display for OrderType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            OrderType::Market => write!(f, "Market"),
            OrderType::Limit => write!(f, "Limit"),
            OrderType::Unknown => write!(f, "UNKNOWN"),
        }
    }
}
