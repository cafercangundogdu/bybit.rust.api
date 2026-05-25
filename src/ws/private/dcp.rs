//! DCP (Disconnected Cancel All Protection) stream.
//!
//! # Topic
//! `dcp` or `dcp.{category}`

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct DcpData {
    #[serde(rename = "category")]
    #[serde(default)]
    pub category: Option<String>,
    /// Whether DCP is triggered (connection lost)
    #[serde(rename = "dcpStatus")]
    #[serde(default)]
    pub dcp_status: Option<String>,
    /// Time window in ms before DCP triggers
    #[serde(rename = "timeWindow")]
    #[serde(default)]
    pub time_window: Option<i64>,
    #[serde(rename = "updatedTime")]
    #[serde(default)]
    pub updated_time: Option<i64>,
}
