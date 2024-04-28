use std::fmt::{Display, Formatter, Result};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum UtaTranslogType {
    #[serde(rename = "TRANSFER_IN")]
    TransferIn, // Assets that transferred into Unified wallet
    #[serde(rename = "TRANSFER_OUT")]
    TransferOut, // Assets that transferred out from Unified wallet
    #[serde(rename = "TRADE")]
    Trade,
    #[serde(rename = "SETTLEMENT")]
    Settlement, // USDT Perp funding settlement, and USDC Perp funding settlement + USDC 8-hour session settlement
    #[serde(rename = "DELIVERY")]
    Delivery, // USDC Futures, Option delivery
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
    #[serde(rename = "INTEREST")]
    Interest, // Interest occurred due to borrowing
    #[serde(rename = "CURRENCY_BUY")]
    CurrencyBuy, // Currency convert, and the liquidation for borrowing asset
    #[serde(rename = "CURRENCY_SELL")]
    CurrencySell, // Currency convert, and the liquidation for borrowing asset
    #[serde(rename = "BORROWED_AMOUNT_INS_LOAN")]
    BorrowedAmountInsLoan,
    #[serde(rename = "PRINCIPLE_REPAYMENT_INS_LOAN")]
    PrincipleRepaymentInsLoan,
    #[serde(rename = "INTEREST_REPAYMENT_INS_LOAN")]
    InterestRepaymentInsLoan,
    #[serde(rename = "AUTO_SOLD_COLLATERAL_INS_LOAN")]
    AutoSoldCollateralInsLoan,
    #[serde(rename = "AUTO_BUY_LIABILITY_INS_LOAN")]
    AutoBuyLiabilityInsLoan,
    #[serde(rename = "AUTO_PRINCIPLE_REPAYMENT_INS_LOAN")]
    AutoPrincipleRepaymentInsLoan,
    #[serde(rename = "AUTO_INTEREST_REPAYMENT_INS_LOAN")]
    AutoInterestRepaymentInsLoan,
    #[serde(rename = "TRANSFER_IN_INS_LOAN")]
    TransferInInsLoan, // Transfer In when in the liquidation of OTC loan
    #[serde(rename = "TRANSFER_OUT_INS_LOAN")]
    TransferOutInsLoan, // Transfer Out when in the liquidation of OTC loan
    #[serde(rename = "SPOT_REPAYMENT_SELL")]
    SpotRepaymentSell, // One-click repayment currency sell
    #[serde(rename = "SPOT_REPAYMENT_BUY")]
    SpotRepaymentBuy, // One-click repayment currency buy
    #[serde(rename = "TOKENS_SUBSCRIPTION")]
    TokensSubscription, // Spot leverage token subscription
    #[serde(rename = "TOKENS_REDEMPTION")]
    TokensRedemption, // Spot leverage token redemption
    #[serde(rename = "AUTO_DEDUCTION")]
    AutoDeduction, // Asset auto deducted by system (roll back)
    #[serde(rename = "FLEXIBLE_STAKING_SUBSCRIPTION")]
    FlexibleStakingSubscription, // Byfi flexible stake subscription
    #[serde(rename = "FLEXIBLE_STAKING_REDEMPTION")]
    FlexibleStakingRedemption, // Byfi flexible stake redemption
    #[serde(rename = "FIXED_STAKING_SUBSCRIPTION")]
    FixedStakingSubscription, // Byfi fixed stake subscription
    #[serde(rename = "PREMARKET_TRANSFER_OUT")]
    PremarketTransferOut,
    #[serde(rename = "PREMARKET_DELIVERY_SELL_NEW_COIN")]
    PremarketDeliverySellNewCoin,
    #[serde(rename = "PREMARKET_DELIVERY_BUY_NEW_COIN")]
    PremarketDeliveryBuyNewCoin,
    #[serde(rename = "PREMARKET_DELIVERY_PLEDGE_PAY_SELLER")]
    PremarketDeliveryPledgePaySeller,
    #[serde(rename = "PREMARKET_DELIVERY_PLEDGE_BACK")]
    PremarketDeliveryPledgeBack,
    #[serde(rename = "PREMARKET_ROLLBACK_PLEDGE_BACK")]
    PremarketRollbackPledgeBack,
    #[serde(rename = "PREMARKET_ROLLBACK_PLEDGE_PENALTY_TO_BUYER")]
    PremarketRollbackPledgePenaltyToBuyer,
    #[serde(rename = "CUSTODY_NETWORK_FEE")]
    CustodyNetworkFee, // fireblocks business
    #[serde(rename = "CUSTODY_SETTLE_FEE")]
    CustodySettleFee, // fireblocks business
    #[serde(rename = "CUSTODY_LOCK")]
    CustodyLock, // fireblocks / copper business
    #[serde(rename = "CUSTODY_UNLOCK")]
    CustodyUnlock, // fireblocks business
    #[serde(rename = "CUSTODY_UNLOCK_REFUND")]
    CustodyUnlockRefund, // fireblocks business
    #[serde(rename = "LOANS_BORROW_FUNDS")]
    LoansBorrowFunds, // crypto loan
    #[serde(rename = "LOANS_ASSET_REDEMPTION")]
    LoansAssetRedemption, // crypto loan repayment
}

impl Display for UtaTranslogType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            UtaTranslogType::TransferIn => write!(f, "TRANSFER_IN"),
            UtaTranslogType::TransferOut => write!(f, "TRANSFER_OUT"),
            UtaTranslogType::Trade => write!(f, "TRADE"),
            UtaTranslogType::Settlement => write!(f, "SETTLEMENT"),
            UtaTranslogType::Delivery => write!(f, "DELIVERY"),
            UtaTranslogType::Liquidation => write!(f, "LIQUIDATION"),
            UtaTranslogType::Adl => write!(f, "ADL"),
            UtaTranslogType::Airdrop => write!(f, "AIRDROP"),
            UtaTranslogType::Bonus => write!(f, "BONUS"),
            UtaTranslogType::BonusRecollect => write!(f, "BONUS_RECOLLECT"),
            UtaTranslogType::FeeRefund => write!(f, "FEE_REFUND"),
            UtaTranslogType::Interest => write!(f, "INTEREST"),
            UtaTranslogType::CurrencyBuy => write!(f, "CURRENCY_BUY"),
            UtaTranslogType::CurrencySell => write!(f, "CURRENCY_SELL"),
            UtaTranslogType::BorrowedAmountInsLoan => write!(f, "BORROWED_AMOUNT_INS_LOAN"),
            UtaTranslogType::PrincipleRepaymentInsLoan => write!(f, "PRINCIPLE_REPAYMENT_INS_LOAN"),
            UtaTranslogType::InterestRepaymentInsLoan => write!(f, "INTEREST_REPAYMENT_INS_LOAN"),
            UtaTranslogType::AutoSoldCollateralInsLoan => {
                write!(f, "AUTO_SOLD_COLLATERAL_INS_LOAN")
            }
            UtaTranslogType::AutoBuyLiabilityInsLoan => write!(f, "AUTO_BUY_LIABILITY_INS_LOAN"),
            UtaTranslogType::AutoPrincipleRepaymentInsLoan => {
                write!(f, "AUTO_PRINCIPLE_REPAYMENT_INS_LOAN")
            }
            UtaTranslogType::AutoInterestRepaymentInsLoan => {
                write!(f, "AUTO_INTEREST_REPAYMENT_INS_LOAN")
            }
            UtaTranslogType::TransferInInsLoan => write!(f, "TRANSFER_IN_INS_LOAN"),
            UtaTranslogType::TransferOutInsLoan => write!(f, "TRANSFER_OUT_INS_LOAN"),
            UtaTranslogType::SpotRepaymentSell => write!(f, "SPOT_REPAYMENT_SELL"),
            UtaTranslogType::SpotRepaymentBuy => write!(f, "SPOT_REPAYMENT_BUY"),
            UtaTranslogType::TokensSubscription => write!(f, "TOKENS_SUBSCRIPTION"),
            UtaTranslogType::TokensRedemption => write!(f, "TOKENS_REDEMPTION"),
            UtaTranslogType::AutoDeduction => write!(f, "AUTO_DEDUCTION"),
            UtaTranslogType::FlexibleStakingSubscription => {
                write!(f, "FLEXIBLE_STAKING_SUBSCRIPTION")
            }
            UtaTranslogType::FlexibleStakingRedemption => write!(f, "FLEXIBLE_STAKING_REDEMPTION"),
            UtaTranslogType::FixedStakingSubscription => write!(f, "FIXED_STAKING_SUBSCRIPTION"),
            UtaTranslogType::PremarketTransferOut => write!(f, "PREMARKET_TRANSFER_OUT"),
            UtaTranslogType::PremarketDeliverySellNewCoin => {
                write!(f, "PREMARKET_DELIVERY_SELL_NEW_COIN")
            }
            UtaTranslogType::PremarketDeliveryBuyNewCoin => {
                write!(f, "PREMARKET_DELIVERY_BUY_NEW_COIN")
            }
            UtaTranslogType::PremarketDeliveryPledgePaySeller => {
                write!(f, "PREMARKET_DELIVERY_PLEDGE_PAY_SELLER")
            }
            UtaTranslogType::PremarketDeliveryPledgeBack => {
                write!(f, "PREMARKET_DELIVERY_PLEDGE_BACK")
            }
            UtaTranslogType::PremarketRollbackPledgeBack => {
                write!(f, "PREMARKET_ROLLBACK_PLEDGE_BACK")
            }
            UtaTranslogType::PremarketRollbackPledgePenaltyToBuyer => {
                write!(f, "PREMARKET_ROLLBACK_PLEDGE_PENALTY_TO_BUYER")
            }
            UtaTranslogType::CustodyNetworkFee => write!(f, "CUSTODY_NETWORK_FEE"),
            UtaTranslogType::CustodySettleFee => write!(f, "CUSTODY_SETTLE_FEE"),
            UtaTranslogType::CustodyLock => write!(f, "CUSTODY_LOCK"),
            UtaTranslogType::CustodyUnlock => write!(f, "CUSTODY_UNLOCK"),
            UtaTranslogType::CustodyUnlockRefund => write!(f, "CUSTODY_UNLOCK_REFUND"),
            UtaTranslogType::LoansBorrowFunds => write!(f, "LOANS_BORROW_FUNDS"),
            UtaTranslogType::LoansAssetRedemption => write!(f, "LOANS_ASSET_REDEMPTION"),
        }
    }
}
