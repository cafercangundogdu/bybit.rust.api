use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeRateDetails {
    pub symbol: String,
    #[serde(rename = "takerFeeRate")]
    pub taker_fee_rate: String,
    #[serde(rename = "makerFeeRate")]
    pub maker_fee_rate: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeRateResult {
    pub list: Vec<FeeRateDetails>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rest::client::ServerResponse;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_fee_rate() {
        let json_data = r#"
        {
            "retCode": 0,
            "retMsg": "OK",
            "result": {
                "list": [
                    {
                        "symbol": "BTCUSDT",
                        "takerFeeRate": "0.0006",
                        "makerFeeRate": "0.0001"
                    }
                ]
            },
            "retExtInfo": {},
            "time": 1690872862481
        }
        "#;
        let response: ServerResponse<FeeRateResult> =
            from_str(json_data).expect("Failed to deserialize FeeRateResult");
        assert_eq!(response.ret_code, 0);
        let result = response.result;
        assert_eq!(result.list.len(), 1);
        assert_eq!(result.list[0].symbol, "BTCUSDT");
        assert_eq!(result.list[0].taker_fee_rate, "0.0006");
    }
}
