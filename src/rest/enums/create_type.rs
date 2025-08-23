use std::fmt::{Display, Formatter, Result};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum CreateType {
    // https://bybit-exchange.github.io/docs/v5/enum#createtype
    #[serde(rename = "CreateByUser")]
    CreateByUser,
    #[serde(rename = "CreateByAdminClosing")]
    CreateByAdminClosing,
    #[serde(rename = "CreateByStopOrder")]
    CreateByStopOrder, // Futures conditional order
    #[serde(rename = "CreateByTakeProfit")]
    CreateByTakeProfit, // Futures take profit order
    #[serde(rename = "CreateByPartialTakeProfit")]
    CreateByPartialTakeProfit, // Futures partial take profit order
    #[serde(rename = "CreateByStopLoss")]
    CreateByStopLoss, // Futures stop loss order
    #[serde(rename = "CreateByPartialStopLoss")]
    CreateByPartialStopLoss, // Futures partial stop loss order
    #[serde(rename = "CreateByTrailingStop")]
    CreateByTrailingStop, // Futures trailing stop order
    #[serde(rename = "CreateByLiq")]
    CreateByLiq, // Laddered liquidation to reduce the required maintenance margin
    #[serde(rename = "CreateByTakeOver_PassThrough")]
    CreateByTakeOverPassThrough, // If the position is still subject to liquidation (i.e., does not meet the required maintenance margin level), the position shall be taken over by the liquidation engine and closed at the bankruptcy price.
    #[serde(rename = "CreateByAdl_PassThrough")]
    CreateByAdlPassThrough, // Auto-Deleveraging(ADL)
    #[serde(rename = "CreateByBlock_PassThrough")]
    CreateByBlockPassThrough, // Order placed via Paradigm
    #[serde(rename = "CreateByBlockTradeMovePosition_PassThrough")]
    CreateByBlockTradeMovePositionPassThrough, // Order created by move position
    #[serde(rename = "CreateByClosing")]
    CreateByClosing, // The close order placed via web or app position area - web/app
    #[serde(rename = "CreateByFGridBot")]
    CreateByFGridBot, // Order created via grid bot - web/app
    #[serde(rename = "CloseByFGridBot")]
    CloseByFGridBot, // Order closed via grid bot - web/app
    #[serde(rename = "CreateByTWAP")]
    CreateByTWAP, // Order created by TWAP - web/app
    #[serde(rename = "CreateByTVSignal")]
    CreateByTVSignal, // Order created by TV webhook - web/app
    #[serde(rename = "CreateByMmRateClose")]
    CreateByMmRateClose, // Order created by Mm rate close function - web/app
    #[serde(rename = "CreateByMartingaleBot")]
    CreateByMartingaleBot, // Order created by Martingale bot - web/app
    #[serde(rename = "CloseByMartingaleBot")]
    CloseByMartingaleBot, // Order closed by Martingale bot - web/app
    #[serde(rename = "CreateByIceBerg")]
    CreateByIceBerg, // Order created by Ice berg strategy - web/app
    #[serde(rename = "CreateByArbitrage")]
    CreateByArbitrage, // Order created by arbitrage - web/app
    #[serde(rename = "CreateByDdh")]
    CreateByDdh, // Option dynamic delta hedge order - web/app
}

impl Display for CreateType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            CreateType::CreateByUser => write!(f, "CreateByUser"),
            CreateType::CreateByAdminClosing => write!(f, "CreateByAdminClosing"),
            CreateType::CreateByStopOrder => write!(f, "CreateByStopOrder"),
            CreateType::CreateByTakeProfit => write!(f, "CreateByTakeProfit"),
            CreateType::CreateByPartialTakeProfit => write!(f, "CreateByPartialTakeProfit"),
            CreateType::CreateByStopLoss => write!(f, "CreateByStopLoss"),
            CreateType::CreateByPartialStopLoss => write!(f, "CreateByPartialStopLoss"),
            CreateType::CreateByTrailingStop => write!(f, "CreateByTrailingStop"),
            CreateType::CreateByLiq => write!(f, "CreateByLiq"),
            CreateType::CreateByTakeOverPassThrough => write!(f, "CreateByTakeOver_PassThrough"),
            CreateType::CreateByAdlPassThrough => write!(f, "CreateByAdl_PassThrough"),
            CreateType::CreateByBlockPassThrough => write!(f, "CreateByBlock_PassThrough"),
            CreateType::CreateByBlockTradeMovePositionPassThrough => {
                write!(f, "CreateByBlockTradeMovePosition_PassThrough")
            }
            CreateType::CreateByClosing => write!(f, "CreateByClosing"),
            CreateType::CreateByFGridBot => write!(f, "CreateByFGridBot"),
            CreateType::CloseByFGridBot => write!(f, "CloseByFGridBot"),
            CreateType::CreateByTWAP => write!(f, "CreateByTWAP"),
            CreateType::CreateByTVSignal => write!(f, "CreateByTVSignal"),
            CreateType::CreateByMmRateClose => write!(f, "CreateByMmRateClose"),
            CreateType::CreateByMartingaleBot => write!(f, "CreateByMartingaleBot"),
            CreateType::CloseByMartingaleBot => write!(f, "CloseByMartingaleBot"),
            CreateType::CreateByIceBerg => write!(f, "CreateByIceBerg"),
            CreateType::CreateByArbitrage => write!(f, "CreateByArbitrage"),
            CreateType::CreateByDdh => write!(f, "CreateByDdh"),
        }
    }
}
