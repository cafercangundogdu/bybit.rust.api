use std::fmt::{Display, Formatter, Result};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum ContractTranslogType {
    #[serde(rename = "TRANSFER_IN")]
    TransferIn, // Assets that transferred into (inverse) derivatives wallet
    #[serde(rename = "TRANSFER_OUT")]
    TransferOut, // Assets that transferred out from (inverse) derivatives wallet
    #[serde(rename = "TRADE")]
    Trade,
    #[serde(rename = "SETTLEMENT")]
    Settlement, // USDT / Inverse Perp funding settlement
    #[serde(rename = "DELIVERY")]
    Delivery, // Inverse Futures delivery
    #[serde(rename = "LIQUIDATION")]
    Liquidation,
    #[serde(rename = "ADL")]
    Adl, // Auto-Deleveraging
    #[serde(rename = "AIRDROP")]
    Airdrop,
    #[serde(rename = "BONUS")]
    Bonus, // Bonus claimed
    #[serde(rename = "BONUS_RECOLLECT")]
    BonusRecollect, // Bonus expired
    #[serde(rename = "FEE_REFUND")]
    FeeRefund, // Trading fee refunded
    #[serde(rename = "CURRENCY_BUY")]
    CurrencyBuy, // Currency convert
    #[serde(rename = "CURRENCY_SELL")]
    CurrencySell, // Currency convert
    #[serde(rename = "AUTO_DEDUCTION")]
    AutoDeduction, // Asset auto deducted by system (roll back)
    #[serde(rename = "Others")]
    Others,
}

impl Display for ContractTranslogType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            ContractTranslogType::TransferIn => write!(f, "TRANSFER_IN"),
            ContractTranslogType::TransferOut => write!(f, "TRANSFER_OUT"),
            ContractTranslogType::Trade => write!(f, "TRADE"),
            ContractTranslogType::Settlement => write!(f, "SETTLEMENT"),
            ContractTranslogType::Delivery => write!(f, "DELIVERY"),
            ContractTranslogType::Liquidation => write!(f, "LIQUIDATION"),
            ContractTranslogType::Adl => write!(f, "ADL"),
            ContractTranslogType::Airdrop => write!(f, "AIRDROP"),
            ContractTranslogType::Bonus => write!(f, "BONUS"),
            ContractTranslogType::BonusRecollect => write!(f, "BONUS_RECOLLECT"),
            ContractTranslogType::FeeRefund => write!(f, "FEE_REFUND"),
            ContractTranslogType::CurrencyBuy => write!(f, "CURRENCY_BUY"),
            ContractTranslogType::CurrencySell => write!(f, "CURRENCY_SELL"),
            ContractTranslogType::AutoDeduction => write!(f, "AUTO_DEDUCTION"),
            ContractTranslogType::Others => write!(f, "Others"),
        }
    }
}
