use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CollateralInfoDetails {
    pub currency: String,
    pub equity: String,
    pub free: String,
    #[serde(rename = "collateralRatio")]
    pub collateral_ratio: String,
    #[serde(rename = "collateralSwitch")]
    pub collateral_switch: bool,
    #[serde(rename = "marginCollateral")]
    pub margin_collateral: bool,
    #[serde(rename = "availableToBorrow")]
    pub available_to_borrow: String,
    #[serde(rename = "borrowAmount")]
    pub borrow_amount: String,
    #[serde(rename = "maxBorrowingAmount")]
    pub max_borrowing_amount: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollateralInfoResult {
    pub list: Vec<CollateralInfoDetails>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BorrowHistoryDetails {
    pub currency: String,
    #[serde(rename = "borrowTime")]
    pub borrow_time: String,
    #[serde(rename = "borrowAmount")]
    pub borrow_amount: String,
    #[serde(rename = "borrowCost")]
    pub borrow_cost: String,
    #[serde(rename = "unpaidCost")]
    pub unpaid_cost: String,
    #[serde(rename = "unpaidInterest")]
    pub unpaid_interest: String,
    #[serde(rename = "interestRate")]
    pub interest_rate: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BorrowHistoryResult {
    #[serde(rename = "nextPageCursor")]
    pub next_page_cursor: String,
    pub list: Vec<BorrowHistoryDetails>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rest::client::ServerResponse;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_collateral_info() {
         let json_data = r#"
        {
            "retCode": 0,
            "retMsg": "OK",
            "result": {
                "list": [
                    {
                        "currency": "BTC",
                        "equity": "10.5",
                        "free": "5.2",
                        "collateralRatio": "0.9",
                        "collateralSwitch": true,
                        "marginCollateral": true,
                        "availableToBorrow": "100",
                        "borrowAmount": "0",
                        "maxBorrowingAmount": "200"
                    }
                ]
            },
            "retExtInfo": {},
            "time": 1690872862481
        }
        "#;
        let response: ServerResponse<CollateralInfoResult> = from_str(json_data).expect("Failed to deserialize CollateralInfoResult");
        assert_eq!(response.ret_code, 0);
        let result = response.result;
        assert_eq!(result.list.len(), 1);
        assert_eq!(result.list[0].currency, "BTC");
        assert_eq!(result.list[0].collateral_switch, true);
    }

     #[test]
    fn test_deserialize_borrow_history() {
         let json_data = r#"
        {
            "retCode": 0,
            "retMsg": "OK",
            "result": {
                "nextPageCursor": "cursor_borrow",
                "list": [
                    {
                        "currency": "USDT",
                        "borrowTime": "1690872862481",
                        "borrowAmount": "100",
                        "borrowCost": "0.01",
                        "unpaidCost": "0",
                        "unpaidInterest": "0",
                        "interestRate": "0.0001"
                    }
                ]
            },
            "retExtInfo": {},
            "time": 1690872862481
        }
        "#;
        let response: ServerResponse<BorrowHistoryResult> = from_str(json_data).expect("Failed to deserialize BorrowHistoryResult");
        assert_eq!(response.ret_code, 0);
        let result = response.result;
        assert_eq!(result.list.len(), 1);
        assert_eq!(result.list[0].currency, "USDT");
        assert_eq!(result.list[0].borrow_amount, "100");
    }
}
