use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum IntervalTime {
    #[serde(rename = "5min")]
    FiveMinute, // 5min
    #[serde(rename = "15min")]
    FifteenMinute, // 15min
    #[serde(rename = "30min")]
    ThirtyMinute, // 30min
    #[serde(rename = "1h")]
    OneHour, // 1hour
    #[serde(rename = "4h")]
    FourHour, // 4hour
    #[serde(rename = "1d")]
    OneDay, // 1day
}

impl Display for IntervalTime {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            IntervalTime::FiveMinute => write!(f, "5min"),
            IntervalTime::FifteenMinute => write!(f, "15min"),
            IntervalTime::ThirtyMinute => write!(f, "30min"),
            IntervalTime::OneHour => write!(f, "1h"),
            IntervalTime::FourHour => write!(f, "4h"),
            IntervalTime::OneDay => write!(f, "1d"),
        }
    }
}
