use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountInfoResult {
    #[serde(rename = "unifiedMarginStatus")]
    pub unified_margin_status: i32,
    #[serde(rename = "marginMode")]
    pub margin_mode: String,
    #[serde(rename = "isMasterPlayer")]
    pub is_master_player: bool,
    #[serde(rename = "dcpStatus")]
    pub dcp_status: String,
    #[serde(rename = "timeWindow")]
    pub time_window: i32,
    #[serde(rename = "smpGroup")]
    pub smp_group: i32,
    #[serde(rename = "updatedTime")]
    pub updated_time: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rest::client::ServerResponse;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_account_info() {
        let json_data = r#"
        {
            "retCode": 0,
            "retMsg": "OK",
            "result": {
                "unifiedMarginStatus": 3,
                "marginMode": "PORTFOLIO_MARGIN",
                "isMasterPlayer": true,
                "dcpStatus": "ON",
                "timeWindow": 10,
                "smpGroup": 0,
                "updatedTime": "1690872862481"
            },
            "retExtInfo": {},
            "time": 1690872862481
        }
        "#;
        let response: ServerResponse<AccountInfoResult> =
            from_str(json_data).expect("Failed to deserialize AccountInfoResult");
        assert_eq!(response.ret_code, 0);
        let result = response.result;
        assert_eq!(result.unified_margin_status, 3);
        assert_eq!(result.margin_mode, "PORTFOLIO_MARGIN");
        assert_eq!(result.is_master_player, true);
    }
}
