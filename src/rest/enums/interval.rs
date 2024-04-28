use std::fmt::{Display, Formatter, Result};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Interval {
    #[serde(rename = "1")]
    OneMinute, // 1 minute
    #[serde(rename = "3")]
    ThreeMinute, // 3 minutes
    #[serde(rename = "5")]
    FiveMinute, // 5 minutes
    #[serde(rename = "15")]
    FifteenMinute, // 15 minutes
    #[serde(rename = "30")]
    ThirtyMinute, // 30 minutes
    #[serde(rename = "60")]
    OneHour, // 1 hour
    #[serde(rename = "120")]
    TwoHour, // 2 hours
    #[serde(rename = "240")]
    FourHour, // 4 hours
    #[serde(rename = "360")]
    SixHour, // 6 hours
    #[serde(rename = "720")]
    TwelveHour, // 12 hours
    #[serde(rename = "D")]
    OneDay, // 1 day
    #[serde(rename = "W")]
    OneWeek, // 1 week
    #[serde(rename = "M")]
    OneMonth, // 1 month
}

impl Display for Interval {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Interval::OneMinute => write!(f, "1"),
            Interval::ThreeMinute => write!(f, "3"),
            Interval::FiveMinute => write!(f, "5"),
            Interval::FifteenMinute => write!(f, "15"),
            Interval::ThirtyMinute => write!(f, "30"),
            Interval::OneHour => write!(f, "60"),
            Interval::TwoHour => write!(f, "120"),
            Interval::FourHour => write!(f, "240"),
            Interval::SixHour => write!(f, "360"),
            Interval::TwelveHour => write!(f, "720"),
            Interval::OneDay => write!(f, "D"),
            Interval::OneWeek => write!(f, "W"),
            Interval::OneMonth => write!(f, "M"),
        }
    }
}
