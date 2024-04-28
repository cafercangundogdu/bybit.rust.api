use std::fmt::{Display, Formatter, Result};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum VipLevel {
    #[serde(rename = "No VIP")]
    NoVip,
    #[serde(rename = "VIP-1")]
    Vip1,
    #[serde(rename = "VIP-2")]
    Vip2,
    #[serde(rename = "VIP-3")]
    Vip3,
    #[serde(rename = "VIP-4")]
    Vip4,
    #[serde(rename = "VIP-5")]
    Vip5,
    #[serde(rename = "VIP-Supreme")]
    VipSupreme,
    #[serde(rename = "PRO-1")]
    Pro1,
    #[serde(rename = "PRO-2")]
    Pro2,
    #[serde(rename = "PRO-3")]
    Pro3,
    #[serde(rename = "PRO-4")]
    Pro4,
    #[serde(rename = "PRO-5")]
    Pro5,
}

impl Display for VipLevel {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            VipLevel::NoVip => write!(f, "No VIP"),
            VipLevel::Vip1 => write!(f, "VIP-1"),
            VipLevel::Vip2 => write!(f, "VIP-2"),
            VipLevel::Vip3 => write!(f, "VIP-3"),
            VipLevel::Vip4 => write!(f, "VIP-4"),
            VipLevel::Vip5 => write!(f, "VIP-5"),
            VipLevel::VipSupreme => write!(f, "VIP-Supreme"),
            VipLevel::Pro1 => write!(f, "PRO-1"),
            VipLevel::Pro2 => write!(f, "PRO-2"),
            VipLevel::Pro3 => write!(f, "PRO-3"),
            VipLevel::Pro4 => write!(f, "PRO-4"),
            VipLevel::Pro5 => write!(f, "PRO-5"),
        }
    }
}
