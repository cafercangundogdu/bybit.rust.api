use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ModifyMmpParams {
    #[serde(rename = "baseCoin")]
    pub base_coin: String,
    pub window: i32,
    #[serde(rename = "frozenPeriod")]
    pub frozen_period: i32,
    #[serde(rename = "qtyLimit")]
    pub qty_limit: String,
    #[serde(rename = "deltaLimit")]
    pub delta_limit: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MmpStateDetails {
    #[serde(rename = "baseCoin")]
    pub base_coin: String,
    #[serde(rename = "mmpEnabled")]
    pub mmp_enabled: bool,
    pub window: i32,
    #[serde(rename = "frozenPeriod")]
    pub frozen_period: i32,
    #[serde(rename = "qtyLimit")]
    pub qty_limit: String,
    #[serde(rename = "deltaLimit")]
    pub delta_limit: String,
    #[serde(rename = "mmpFrozen")]
    pub mmp_frozen: bool,
    #[serde(rename = "mmpFrozenUntil")]
    pub mmp_frozen_until: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MmpStateResult {
    pub result: Vec<MmpStateDetails>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rest::client::ServerResponse;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_mmp_state() {
        let json_data = r#"
        {
            "retCode": 0,
            "retMsg": "OK",
            "result": {
                "result": [
                    {
                        "baseCoin": "BTC",
                        "mmpEnabled": true,
                        "window": 5000,
                        "frozenPeriod": 100000,
                        "qtyLimit": "100",
                        "deltaLimit": "10",
                        "mmpFrozen": false,
                        "mmpFrozenUntil": "0"
                    }
                ]
            },
            "retExtInfo": {},
            "time": 1690872862481
        }
        "#;
        let response: ServerResponse<MmpStateResult> = from_str(json_data).expect("Failed to deserialize MmpStateResult");
        assert_eq!(response.ret_code, 0);
        let result = response.result;
        assert_eq!(result.result.len(), 1);
        assert_eq!(result.result[0].base_coin, "BTC");
        assert_eq!(result.result[0].mmp_enabled, true);
    }
}
