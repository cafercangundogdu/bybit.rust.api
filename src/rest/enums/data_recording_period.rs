use std::fmt::{Display, Formatter, Result};

use serde::{Deserialize, Serialize};

// https://bybit-exchange.github.io/docs/v5/enum#optionperiod
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum DataRecordingPeriod {
    #[serde(rename = "5min")]
    FiveMinute, // 5 minutes
    #[serde(rename = "15min")]
    FifteenMinute, // 15 minutes
    #[serde(rename = "30min")]
    ThirtyMinute, // 30 minutes
    #[serde(rename = "1h")]
    OneHour, // 1 hour
    #[serde(rename = "4h")]
    FourHour, // 4 hours
    #[serde(rename = "4d")]
    FourDay, // 4 days
}

impl Display for DataRecordingPeriod {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            DataRecordingPeriod::FiveMinute => write!(f, "5min"),
            DataRecordingPeriod::FifteenMinute => write!(f, "15min"),
            DataRecordingPeriod::ThirtyMinute => write!(f, "30min"),
            DataRecordingPeriod::OneHour => write!(f, "1h"),
            DataRecordingPeriod::FourHour => write!(f, "4h"),
            DataRecordingPeriod::FourDay => write!(f, "4d"),
        }
    }
}
