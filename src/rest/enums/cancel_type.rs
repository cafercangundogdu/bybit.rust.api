use std::fmt::{Display, Formatter, Result};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum TriggerBy {
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

impl Display for TriggerBy {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            TriggerBy::CancelByUser => write!(f, "CancelByUser"),
            TriggerBy::CancelByReduceOnly => write!(f, "CancelByReduceOnly"),
            TriggerBy::CancelByPrepareLiq => write!(f, "CancelByPrepareLiq"),
            TriggerBy::CancelAllBeforeLiq => write!(f, "CancelAllBeforeLiq"),
            TriggerBy::CancelByPrepareAdl => write!(f, "CancelByPrepareAdl"),
            TriggerBy::CancelAllBeforeAdl => write!(f, "CancelAllBeforeAdl"),
            TriggerBy::CancelByAdmin => write!(f, "CancelByAdmin"),
            TriggerBy::CancelByTpSlTsClear => write!(f, "CancelByTpSlTsClear"),
            TriggerBy::CancelBySmp => write!(f, "CancelBySmp"),
            TriggerBy::CancelBySettle => write!(f, "CancelBySettle"),
            TriggerBy::CancelByCannotAffordOrderCost => write!(f, "CancelByCannotAffordOrderCost"),
            TriggerBy::CancelByPmTrialMmOverEquity => write!(f, "CancelByPmTrialMmOverEquity"),
            TriggerBy::CancelByAccountBlocking => write!(f, "CancelByAccountBlocking"),
            TriggerBy::CancelByDelivery => write!(f, "CancelByDelivery"),
            TriggerBy::CancelByMmpTriggered => write!(f, "CancelByMmpTriggered"),
            TriggerBy::CancelByCrossSelfMuch => write!(f, "CancelByCrossSelfMuch"),
            TriggerBy::CancelByCrossReachMaxTradeNum => write!(f, "CancelByCrossReachMaxTradeNum"),
            TriggerBy::CancelByDCP => write!(f, "CancelByDCP"),
        }
    }
}
