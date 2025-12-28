use std::fmt::{Display, Formatter, Result};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum CancelType {
    #[serde(rename = "CancelByUser")]
    CancelByUser,
    #[serde(rename = "CancelByReduceOnly")]
    CancelByReduceOnly, // cancelled by reduceOnly
    #[serde(rename = "CancelByPrepareLiq")]
    CancelByPrepareLiq, // cancelled in order to attempt liquidation prevention by freeing up margin
    #[serde(rename = "CancelAllBeforeLiq")]
    CancelAllBeforeLiq, // cancelled in order to attempt liquidation prevention by freeing up margin
    #[serde(rename = "CancelByPrepareAdl")]
    CancelByPrepareAdl, // cancelled due to ADL
    #[serde(rename = "CancelAllBeforeAdl")]
    CancelAllBeforeAdl, // cancelled due to ADL
    #[serde(rename = "CancelBySettle")]
    CancelBySettle,
    #[serde(rename = "CancelByCannotAffordOrderCost")]
    CancelByCannotAffordOrderCost,
    #[serde(rename = "CancelByPmTrialMmOverEquity")]
    CancelByPmTrialMmOverEquity,
    #[serde(rename = "CancelByAccountBlocking")]
    CancelByAccountBlocking,
    #[serde(rename = "CancelByDelivery")]
    CancelByDelivery,
    #[serde(rename = "CancelByMmpTriggered")]
    CancelByMmpTriggered,
    #[serde(rename = "CancelByCrossSelfMuch")]
    CancelByCrossSelfMuch,
    #[serde(rename = "CancelByCrossReachMaxTradeNum")]
    CancelByCrossReachMaxTradeNum,
    #[serde(rename = "CancelByDCP")]
    CancelByDCP,
    #[serde(rename = "CancelByAdmin")]
    CancelByAdmin,
    #[serde(rename = "CancelByTpSlTsClear")]
    CancelByTpSlTsClear, // TP/SL order cancelled when the position is cleared
    #[serde(rename = "CancelBySmp")]
    CancelBySmp, // cancelled by SMP
}

impl Display for CancelType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            CancelType::CancelByUser => write!(f, "CancelByUser"),
            CancelType::CancelByReduceOnly => write!(f, "CancelByReduceOnly"),
            CancelType::CancelByPrepareLiq => write!(f, "CancelByPrepareLiq"),
            CancelType::CancelAllBeforeLiq => write!(f, "CancelAllBeforeLiq"),
            CancelType::CancelByPrepareAdl => write!(f, "CancelByPrepareAdl"),
            CancelType::CancelAllBeforeAdl => write!(f, "CancelAllBeforeAdl"),
            CancelType::CancelByAdmin => write!(f, "CancelByAdmin"),
            CancelType::CancelByTpSlTsClear => write!(f, "CancelByTpSlTsClear"),
            CancelType::CancelBySmp => write!(f, "CancelBySmp"),
            CancelType::CancelBySettle => write!(f, "CancelBySettle"),
            CancelType::CancelByCannotAffordOrderCost => write!(f, "CancelByCannotAffordOrderCost"),
            CancelType::CancelByPmTrialMmOverEquity => write!(f, "CancelByPmTrialMmOverEquity"),
            CancelType::CancelByAccountBlocking => write!(f, "CancelByAccountBlocking"),
            CancelType::CancelByDelivery => write!(f, "CancelByDelivery"),
            CancelType::CancelByMmpTriggered => write!(f, "CancelByMmpTriggered"),
            CancelType::CancelByCrossSelfMuch => write!(f, "CancelByCrossSelfMuch"),
            CancelType::CancelByCrossReachMaxTradeNum => write!(f, "CancelByCrossReachMaxTradeNum"),
            CancelType::CancelByDCP => write!(f, "CancelByDCP"),
        }
    }
}
