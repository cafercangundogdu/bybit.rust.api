use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy, Default)]
pub enum Category {
    #[default]
    #[serde(rename = "spot")]
    Spot,
    #[serde(rename = "linear")]
    Linear,
    #[serde(rename = "inverse")]
    Inverse,
    #[serde(rename = "option")]
    Option,
}

impl Display for Category {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Category::Spot => write!(f, "spot"),
            Category::Linear => write!(f, "linear"),
            Category::Inverse => write!(f, "inverse"),
            Category::Option => write!(f, "option"),
        }
    }
}
