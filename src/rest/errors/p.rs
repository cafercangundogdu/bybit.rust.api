use std::fmt::{Display, Formatter, Result};

use serde::{Deserialize, Serialize};

use crate::rest::errors::types::ErrorTypes;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum ErrorCodes {
    // HTTP error codes
    #[serde(rename = "400")]
    BadRequest, // Bad request. Need to send the request with GET / POST (must be capitalized)
    #[serde(rename = "401")]
    InvalidRequest, // Invalid request. 1. Need to use the correct key to access; 2. Need to put authentication params in the request header
    #[serde(rename = "403")]
    ForbiddenRequest, // Forbidden request. Possible causes: 1. IP rate limit breached; 2. You send GET request with an empty json body; 3. You are using U.S IP
    #[serde(rename = "404")]
    CannotFindPath, // Cannot find path. Possible causes: 1. Wrong path; 2. Category value does not match account mode

    // UTA & Classic Account error codes
    #[serde(rename = "0")]
    Ok,
    #[serde(rename = "10000")]
    ServerTimeout,
    #[serde(rename = "10001")]
    RequestParameterError,
    #[serde(rename = "10002")]
    RequestTimeExceedsTimeWindowRange,
    #[serde(rename = "10003")]
    ApiKeyInvalid,
    #[serde(rename = "33004")]
    ApiKeyExpired,
    #[serde(rename = "10004")]
    ErrorSign,
    #[serde(rename = "10005")]
    PermissionDenied,
    #[serde(rename = "10006")]
    TooManyVisits,
    #[serde(rename = "10007")]
    UserAuthenticationFailed,
    #[serde(rename = "10008")]
    CommonBanned,
    #[serde(rename = "10009")]
    IpBanned,
    #[serde(rename = "10010")]
    UnmatchedIp,
    #[serde(rename = "10014")]
    InvalidDuplicateRequest,
    #[serde(rename = "10016")]
    ServerError,
    #[serde(rename = "10017")]
    RouteNotFound,
    #[serde(rename = "10018")]
    ExceededIpRateLimit,
    #[serde(rename = "10024")]
    ComplianceRulesTriggered,
    #[serde(rename = "10027")]
    TransactionsBanned,
    #[serde(rename = "10029")]
    InvalidSymbol,
    #[serde(rename = "10028")]
    ApiAccessUnifiedAccount,
    #[serde(rename = "30133")]
    OtcLoanUsdtPerpetualSymbolNotAllowed,
    #[serde(rename = "30134")]
    OtcLoanUsdcContractSymbolNotAllowed,
    #[serde(rename = "30135")]
    LeverageSelectUsdtPerpetualTradingCannotExceedMaxLeverage,
    #[serde(rename = "30136")]
    LeverageSelectUsdcPerpetualOrFuturesTradingCannotExceedMaxLeverage,
    #[serde(rename = "40004")]
    OrderModifiedDuringProcessOfReplacing,
    #[serde(rename = "100028")]
    ApiAccessUnifiedAccountUsers,
    #[serde(rename = "110001")]
    OrderDoesNotExist,
    #[serde(rename = "110003")]
    OrderPriceExceedsAllowableRange,
    #[serde(rename = "110004")]
    WalletBalanceInsufficient1,
    #[serde(rename = "110005")]
    PositionStatus,
    #[serde(rename = "110006")]
    AssetsEstimatedUnableCoverPositionMargin,
    #[serde(rename = "110007")]
    AvailableBalanceInsufficient,
    #[serde(rename = "110008")]
    OrderCompletedOrCancelled,
    #[serde(rename = "110009")]
    NumberOfStopOrdersExceedsMaxAllowableLimit,
    #[serde(rename = "110010")]
    OrderCancelled,
    #[serde(rename = "110011")]
    LiquidationTriggeredImmediatelyByAdjustment1,
    #[serde(rename = "110012")]
    InsufficientAvailableBalance,
    #[serde(rename = "110013")]
    CannotSetLeverageDueToRiskLimitLevel,
    #[serde(rename = "110014")]
    InsufficientAvailableBalanceToAddAdditionalMargin,
    #[serde(rename = "110015")]
    PositionInCrossMarginMode,
    #[serde(rename = "110016")]
    QuantityContractsRequestedExceedsRiskLimit,
    #[serde(rename = "110017")]
    ReduceOnlyRuleNotSatisfied,
    #[serde(rename = "110018")]
    UserIdIllegal,
    #[serde(rename = "110019")]
    OrderIdIllegal,
    #[serde(rename = "110020")]
    NotAllowedMoreThan500ActiveOrders,
    #[serde(rename = "110021")]
    NotAllowedExceededPositionLimitsDueToOpenInterest,
    #[serde(rename = "110022")]
    QuantityRestrictedOrdersCannotBeModifiedIncreaseQuantity,
    #[serde(rename = "110023")]
    CurrentlyOnlyReducePositionOnContract,
    #[serde(rename = "110024")]
    ExistingPositionPositionModeCannotBeSwitched,
    #[serde(rename = "110025")]
    PositionModeNotModified,
    #[serde(rename = "110026")]
    CrossIsolatedMarginModeNotModified,
    #[serde(rename = "110027")]
    MarginNotModified,
    #[serde(rename = "110028")]
    ExistingOpenOrdersPositionModeCannotBeSwitched,
    #[serde(rename = "110029")]
    HedgeModeNotSupportedSymbol,
    #[serde(rename = "110030")]
    DuplicateOrderId,
    #[serde(rename = "110031")]
    NonExistingRiskLimitInfo,
    #[serde(rename = "110032")]
    OrderIllegal,
    #[serde(rename = "110033")]
    CannotSetMarginWithoutOpenPosition,
    #[serde(rename = "110034")]
    NoNetPosition,
    #[serde(rename = "110035")]
    CancellationOrdersNotCompletedBeforeLiquidation,
    #[serde(rename = "110036")]
    NotAllowedChangeLeverageDueToCrossMarginMode,
    #[serde(rename = "110037")]
    UserSettingListDoesNotHaveSymbol,
    #[serde(rename = "110038")]
    NotAllowedChangeLeverageDueToPortfolioMarginMode,
    #[serde(rename = "110039")]
    MaintenanceMarginRateTooHighMayTriggerLiquidation,
    #[serde(rename = "110040")]
    OrderWillTriggerForcedLiquidation,
    #[serde(rename = "110041")]
    SkipLiquidationNotAllowedWhenPositionOrMakerOrderExists,
    #[serde(rename = "110042")]
    CurrentlyDueToPreDeliveryStatusOnlyReducePositionOnContract,
    #[serde(rename = "110043")]
    SetLeverageNotModified,
    #[serde(rename = "110044")]
    AvailableMarginInsufficient,
    #[serde(rename = "110045")]
    WalletBalanceInsufficient2,
    #[serde(rename = "110046")]
    LiquidationTriggeredImmediatelyByAdjustment2,
    #[serde(rename = "110047")]
    RiskLimitCannotBeAdjustedDueToInsufficientAvailableMargin,
    #[serde(rename = "110048")]
    RiskLimitCannotBeAdjustedCurrentExpectedPositionValueExceedsRevisedRiskLimit,
    #[serde(rename = "110049")]
    TickNotesCanOnlyBeNumbers,
    #[serde(rename = "110050")]
    InvalidCoin,
    #[serde(rename = "110051")]
    UsersAvailableBalanceCannotCoverLowestPriceCurrentMarket,
    #[serde(rename = "110052")]
    UsersAvailableBalanceInsufficientSetPrice,
    #[serde(rename = "110053")]
    UsersAvailableBalanceCannotCoverCurrentMarketPriceUpperLimitPrice,
    #[serde(rename = "110054")]
    PositionHasAtLeastOneTakeProfitLinkOrderTakeProfitStopLossModeCannotBeSwitched,
    #[serde(rename = "110055")]
    PositionHasAtLeastOneStopLossLinkOrderTakeProfitStopLossModeCannotBeSwitched,
    #[serde(rename = "110056")]
    PositionHasAtLeastOneTrailingStopLinkOrderTakeProfitStopLossModeCannotBeSwitched,
    #[serde(rename = "110057")]
    ConditionalOrderLimitOrderContainsTpSlRelatedParams,
    #[serde(rename = "110058")]
    CannotSetTakeProfitStopLossDueToInsufficientSizeRemainingPositionSize,
    #[serde(rename = "110059")]
    NotAllowedModifyTpSlPartiallyFilledOpenOrder,
    #[serde(rename = "110060")]
    UnderFullTpSlModeNotAllowedModifyTpSl,
    #[serde(rename = "110061")]
    NotAllowedMoreThan20TpSlsUnderPartialTpSlMode,
    #[serde(rename = "110062")]
    NoMmpInformationInstitutionFound,
    #[serde(rename = "110063")]
    SettlementInProgressKey0NotAvailableForTrading,
    #[serde(rename = "110064")]
    ModifiedContractQuantityCannotBeLessThanEqualToFilledQuantity,
    #[serde(rename = "110065")]
    MmpNotEnabledForAccountContactBdManager,
    #[serde(rename = "110066")]
    TradingCurrentlyNotAllowed,
    #[serde(rename = "110067")]
    UnifiedAccountNotSupported,
    #[serde(rename = "110068")]
    LeveragedTradingNotAllowed,
    #[serde(rename = "110069")]
    InsLendingCustomerNotAllowedTrade,
    #[serde(rename = "110070")]
    EtpSymbolsCannotBeTraded,
    #[serde(rename = "110071")]
    UnifiedMarginAccountRevampingUpgradesNotSupported,
    #[serde(rename = "110072")]
    OrderLinkedIdDuplicate,
    #[serde(rename = "110073")]
    SetMarginModeFailed,
    #[serde(rename = "110075")]
    RiskIdNotModified,
    #[serde(rename = "110075 182021")]
    CannotEnableSpotMarginWhileInIsolatedMarginModeSwitchCrossMarginModePortfolioMarginModeTradeSpotWithMargin,
    #[serde(rename = "110076")]
    OnlyIsolatedModeCanSetAutoAddMargin,
    #[serde(rename = "110077")]
    PmModeCannotSupport,
    #[serde(rename = "110078")]
    AddedMarginMoreThanMaxCanReduceMargin,
    #[serde(rename = "110079")]
    OrderProcessingCannotOperatedTryAgainLater,
    #[serde(rename = "110080")]
    OperationsRestrictionCurrentLtvRatioInstitutionalLendingHitLiquidationThresholdAssetsLiquidatedTradeRiskLimitLeverage,
    #[serde(rename = "110082")]
    CannotLiftReduceOnlyRestrictionsNoReduceOnlyRestrictionsAppliedPosition,
    #[serde(rename = "110083")]
    ReduceOnlyRestrictionsMustLiftBothLongShortPositionsSameTime,
    #[serde(rename = "110085")]
    RiskLimitMarginRatioContractUpdatedSelectSupportedRiskLimitPlaceOrder,
    #[serde(rename = "110086")]
    CurrentOrderLeverageExceedsMaxAvailableCurrentRiskLimitTierLowerLeveragePlacingOrder,
    #[serde(rename = "110087")]
    UpgradeToUtaTrade,
    #[serde(rename = "110089")]
    ExceedsMaxRiskLimitLevel,
    #[serde(rename = "110090")]
    ExceedsMaxLeverageLimitCurrentRiskLimitLevel,
    #[serde(rename = "110092")]
    ExpectRisingTriggerPriceCurrentLast,
    #[serde(rename = "110093")]
    ExpectFallingTriggerPriceCurrentLast,
    #[serde(rename = "110094")]
    OrderNotionalValueBelowLowerLimit,
    #[serde(rename = "181017")]
    OrderStatusMustBeFinalStatus,
    #[serde(rename = "182100")]
    CompulsoryClosingPositionsNoRepaymentAllowed,
    #[serde(rename = "182101")]
    FailedRepaymentInsufficientCollateralBalance,
    #[serde(rename = "182102")]
    FailedRepaymentNoLiabilitiesCurrentCurrency,
    #[serde(rename = "182103")]
    InstitutionalLendingUsersNotSupported,
    #[serde(rename = "182108")]
    SwitchingFailedMarginVerificationFailedReadjustCurrencyStatus,
    #[serde(rename = "182110")]
    FailedSwitch,
    #[serde(rename = "182111")]
    RequestedCurrencyNonGuaranteedGoldCurrencyNotSupportSwitchingStatusCurrencies,
    #[serde(rename = "182112")]
    DuplicateCurrencyReadjust,
    #[serde(rename = "3100181")]
    UidCanNotNull,
    #[serde(rename = "3100197")]
    TemporaryBannedUpgradeUta,
    #[serde(rename = "3200316")]
    UsdcOptionsTradingRestrictionCurrentLtvRatioInstitutionalLendingReachedMaxAllowableAmountUsdcOptionsTrading,
    #[serde(rename = "3200317")]
    UsdcOptionsOpenPositionRestrictionCurrentLtvRatioInstitutionalLendingReachedMaxAllowableAmountOpeningUsdcOptionsPositions,
    #[serde(rename = "3100326")]
    BaseCoinRequired,
    #[serde(rename = "3200403")]
    IsolatedMarginCannotCreateOrder,
    #[serde(rename = "3200320")]
    OperationsRestrictionCurrentLtvRatioInstitutionalLendingHitLiquidationThresholdAssetsLiquidatedMarginModeSpotLeverage,
    #[serde(rename = "3400208")]
    UnclosedHedgeModeIsolatedModeUsdtPerpetualPositions,
    #[serde(rename = "3400209")]
    UsdtPerpetualPositionsUpgradingProhibited10MinutesBeforeAfterHourEveryHour,
    #[serde(rename = "3400210")]
    RiskRateDerivativesAccountTooHigh,
    #[serde(rename = "3400211")]
    UpgradedEstimatedRiskRateTooHigh,
    #[serde(rename = "3400212")]
    UsdcPerpetualPositionsOptionsPositionsUpgradingProhibited10MinutesBeforeAfterHourEveryHour,
    #[serde(rename = "3400213")]
    RiskRateUsdcDerivativesAccountTooHigh,
    #[serde(rename = "3400052")]
    UncancelledUsdcPerpetualOrders,
    #[serde(rename = "3400053")]
    UncancelledOptionsOrders,
    #[serde(rename = "3400054")]
    UncancelledUsdtPerpetualOrders,
    #[serde(rename = "3400214")]
    ServerErrorTryAgainLater,
    #[serde(rename = "3400071")]
    NetAssetNotSatisfied,
    #[serde(rename = "3401010")]
    CannotSwitchPmModeCopyTradingMasterTrader,
    #[serde(rename = "3400139")]
    TotalValuePositionsOrdersExceededRiskLimitPerpetualFuturesContract,
    #[serde(rename = "500010")]
    SubAccountNotBelongParentAccount,
    #[serde(rename = "500011")]
    UidNotAssociatedUnifiedTradingAccount,

    // Spot Trade
    #[serde(rename = "170001")]
    InternalError,
    #[serde(rename = "170005")]
    TooManyNewOrders,
    #[serde(rename = "170007")]
    TimeoutWaitingForResponse,
    #[serde(rename = "170010")]
    PurchaseFailedExceedMaxPositionLimitLeveragedTokens,
    #[serde(rename = "170011")]
    PurchaseFailedExceedMaxPositionLimitInnovationTokens,
    #[serde(rename = "170019")]
    PurchaseFailedExceedMaxPositionLimit,
    #[serde(rename = "170031")]
    FeatureSuspended,
    #[serde(rename = "170032")]
    NetworkError,
    #[serde(rename = "170033")]
    MarginInsufficientAccountBalance,
    #[serde(rename = "170034")]
    LiabilityOverflowInSpotLeverageTrade,
    #[serde(rename = "170035")]
    SubmittedToSystemForProcessing,
    #[serde(rename = "170036")]
    CrossMarginTradingNotEnabled,
    #[serde(rename = "170037")]
    CrossMarginTradingNotSupported,
    #[serde(rename = "170105")]
    ParameterEmpty,
    #[serde(rename = "170115")]
    InvalidTimeInForce,
    #[serde(rename = "170116")]
    InvalidOrderType,
    #[serde(rename = "170117")]
    InvalidSide,
    #[serde(rename = "170121")]
    InvalidSymbol,
    #[serde(rename = "170124")]
    OrderAmountTooLarge,
    #[serde(rename = "170130")]
    DataSentForParameterNotValid,
    #[serde(rename = "170131")]
    BalanceInsufficient,
    #[serde(rename = "170132")]
    OrderPriceTooHigh,
    #[serde(rename = "170133")]
    OrderPriceLowerThanMinimum,
    #[serde(rename = "170134")]
    OrderPriceDecimalTooLong,
    #[serde(rename = "170135")]
    OrderQuantityTooLarge,
    #[serde(rename = "170136")]
    OrderQuantityLowerThanMinimum,
    #[serde(rename = "170137")]
    OrderVolumeDecimalTooLong,
    #[serde(rename = "170139")]
    OrderFilled,
    #[serde(rename = "170140")]
    TransactionAmountLowerThanMinimum,
    #[serde(rename = "170141")]
    DuplicateClientOrderId,
    #[serde(rename = "170142")]
    OrderCancelled,
    #[serde(rename = "170143")]
    CannotBeFoundOnOrderBook,
    #[serde(rename = "170144")]
    OrderLocked,
    #[serde(rename = "170145")]
    OrderTypeDoesNotSupportCancellation,
    #[serde(rename = "170146")]
    OrderCreationTimeout,
    #[serde(rename = "170147")]
    OrderCancellationTimeout,
    #[serde(rename = "170148")]
    MarketOrderAmountDecimalTooLong,
    #[serde(rename = "170149")]
    CreateOrderFailed,
    #[serde(rename = "170150")]
    CancelOrderFailed,
    #[serde(rename = "170151")]
    TradingPairNotOpenYet,
    #[serde(rename = "170157")]
    TradingPairNotAvailableForApiTrading,
    #[serde(rename = "170159")]
    MarketOrderNotSupportedFirstMinutesNewlyLaunchedPairs,
    #[serde(rename = "170190")]
    CancelOrderFinished,
    /*
        170001	Internal error.
    170005	Too many new orders; current limit is %s orders per %s.
    170007	Timeout waiting for response from backend server.
    170010	Purchase failed: Exceed the maximum position limit of leveraged tokens, the current available limit is %s USDT
    170011	"Purchase failed: Exceed the maximum position limit of innovation tokens,
    170019	the current available limit is ''{{.replaceKey0}}'' USDT"
    170031	The feature has been suspended
    170032	Network error. Please try again later
    170033	margin Insufficient account balance
    170034	Liability over flow in spot leverage trade!
    170035	Submitted to the system for processing!
    170036	You haven't enabled Cross Margin Trading yet. To do so, please head to the PC trading site or the Bybit app
    170037	Cross Margin Trading not yet supported by the selected coin
    170105	Parameter '%s' was empty.
    170115	Invalid timeInForce.
    170116	Invalid orderType.
    170117	Invalid side.
    170121	Invalid symbol.
    170124	Order amount too large.
    170130	Data sent for paramter '%s' is not valid.
    170131	Balance insufficient
    170132	Order price too high.
    170133	Order price lower than the minimum.
    170134	Order price decimal too long.
    170135	Order quantity too large.
    170136	Order quantity lower than the minimum.
    170137	Order volume decimal too long
    170139	Order has been filled.
    170140	Transaction amount lower than the minimum.
    170141	Duplicate clientOrderId
    170142	Order has been canceled
    170143	Cannot be found on order book
    170144	Order has been locked
    170145	This order type does not support cancellation
    170146	Order creation timeout
    170147	Order cancellation timeout
    170148	Market order amount decimal too long
    170149	Create order failed
    170150	Cancel order failed
    170151	The trading pair is not open yet
    170157	The trading pair is not available for api trading
    170159	Market Order is not supported within the first %s minutes of newly launched pairs due to risk control.
    170190	Cancel order has been finished
    170191	Can not cancel order, please try again later
    170192	Order price cannot be higher than %s .
    170193	Buy order price cannot be higher than %s.
    170194	Sell order price cannot be lower than %s.
    170195	Please note that your order may not be filled. ETP buy order price deviates from risk control
    170196	Please note that your order may not be filled. ETP sell order price deviates from risk control
    170197	Your order quantity to buy is too large. The filled price may deviate significantly from the market price. Please try again
    170198	Your order quantity to sell is too large. The filled price may deviate significantly from the market price. Please try again
    170199	Your order quantity to buy is too large. The filled price may deviate significantly from the nav. Please try again.
    170200	Your order quantity to sell is too large. The filled price may deviate significantly from the nav. Please try again.
    170201	Invalid orderFilter parameter
    170202	Please enter the TP/SL price.
    170203	trigger price cannot be higher than 110% price.
    170204	trigger price cannot be lower than 90% of qty.
    170206	Stop_limit Order is not supported within the first 5 minutes of newly launched pairs
    170207	The loan amount of the platform is not enough.
    170210	New order rejected.
    170212	Cancel order request processing
    170213	Order does not exist.
    170215	Spot Trading (Buy) Restriction: The current LTV ratio of your institutional lending has reached the maximum allowable amount for buy orders
    170216	The leverage you select for Spot Trading cannot exceed the maximum leverage allowed by Institutional Lending
    170217	Only LIMIT-MAKER order is supported for the current pair.
    170218	The LIMIT-MAKER order is rejected due to invalid price.
    170219	UID {{xxx}} is not available to this feature
    170220	Spot Trading Restriction: The current LTV ratio of your institutional lending has reached the maximum allowable amount for Spot trading
    170221	This coin does not exist.
    170222	Too many requests in this time frame.
    170223	Your Spot Account with Institutional Lending triggers an alert or liquidation.
    170224	You're not a user of the Innovation Zone.
    170226	Your Spot Account for Margin Trading is being liquidated.
    170227	This feature is not supported.
    170228	The purchase amount of each order exceeds the estimated maximum purchase amount.
    170229	The sell quantity per order exceeds the estimated maximum sell quantity.
    170230	Operations Restriction: Due to the deactivation of Margin Trading for institutional loan
    170234	System Error
    170241	To proceed with trading, users must read through and confirm that they fully understand the project's risk disclosure document. For App users, please update your Bybit App to version 4.16.0 to process.
    170310	Order modification timeout
    170311	Order modification failed
    170312	The current order does not support modification
    170313	The modified contract quantity cannot be less than to the filled quantity
    170341	Request order quantity exceeds maximum limit
    170344	symbol loanable limit
    170709	OTC loan: The select trading pair is not in the whitelist pair

         */
}

impl Display for ErrorCodes {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            // HTTP error codes
            ErrorCodes::BadRequest => write!(f, "400"),
            ErrorCodes::InvalidRequest => write!(f, "401"),
            ErrorCodes::ForbiddenRequest => write!(f, "403"),
            ErrorCodes::CannotFindPath => write!(f, "404"),

            // UTA & Classic Account error codes
            ErrorCodes::Ok => write!(f, "0"),
            ErrorCodes::ServerTimeout => write!(f, "10000"),
            ErrorCodes::RequestParameterError => write!(f, "10001"),
            ErrorCodes::RequestTimeExceedsTimeWindowRange => write!(f, "10002"),
            ErrorCodes::ApiKeyInvalid => write!(f, "10003"),
            ErrorCodes::ApiKeyExpired => write!(f, "33004"),
            ErrorCodes::ErrorSign => write!(f, "10004"),
            ErrorCodes::PermissionDenied => write!(f, "10005"),
            ErrorCodes::TooManyVisits => write!(f, "10006"),
            ErrorCodes::UserAuthenticationFailed => write!(f, "10007"),
            ErrorCodes::CommonBanned => write!(f, "10008"),
            ErrorCodes::IpBanned => write!(f, "10009"),
            ErrorCodes::UnmatchedIp => write!(f, "10010"),
            ErrorCodes::InvalidDuplicateRequest => write!(f, "10014"),
            ErrorCodes::ServerError => write!(f, "10016"),
            ErrorCodes::RouteNotFound => write!(f, "10017"),
            ErrorCodes::ExceededIpRateLimit => write!(f, "10018"),
            ErrorCodes::ComplianceRulesTriggered => write!(f, "10024"),
            ErrorCodes::TransactionsBanned => write!(f, "10027"),
            ErrorCodes::InvalidSymbol => write!(f, "10029"),
            ErrorCodes::ApiAccessUnifiedAccount => write!(f, "10028"),
            ErrorCodes::OtcLoanUsdtPerpetualSymbolNotAllowed => write!(f, "30133"),
            ErrorCodes::OtcLoanUsdcContractSymbolNotAllowed => write!(f, "30134"),
            ErrorCodes::LeverageSelectUsdtPerpetualTradingCannotExceedMaxLeverage => write!(f, "30135"),
            ErrorCodes::LeverageSelectUsdcPerpetualOrFuturesTradingCannotExceedMaxLeverage => write!(f, "30136"),
            ErrorCodes::OrderModifiedDuringProcessOfReplacing => write!(f, "40004"),
            ErrorCodes::ApiAccessUnifiedAccountUsers => write!(f, "100028"),
            ErrorCodes::OrderDoesNotExist => write!(f, "110001"),
            ErrorCodes::OrderPriceExceedsAllowableRange => write!(f, "110003"),
            ErrorCodes::WalletBalanceInsufficient1 => write!(f, "110004"),
            ErrorCodes::PositionStatus => write!(f, "110005"),
            ErrorCodes::AssetsEstimatedUnableCoverPositionMargin => write!(f, "110006"),
            ErrorCodes::AvailableBalanceInsufficient => write!(f, "110007"),
            ErrorCodes::OrderCompletedOrCancelled => write!(f, "110008"),
            ErrorCodes::NumberOfStopOrdersExceedsMaxAllowableLimit => write!(f, "110009"),
            ErrorCodes::OrderCancelled => write!(f, "110010"),
            ErrorCodes::LiquidationTriggeredImmediatelyByAdjustment1 => write!(f, "110011"),
            ErrorCodes::InsufficientAvailableBalance => write!(f, "110012"),
            ErrorCodes::CannotSetLeverageDueToRiskLimitLevel => write!(f, "110013"),
            ErrorCodes::InsufficientAvailableBalanceToAddAdditionalMargin => write!(f, "110014"),
            ErrorCodes::PositionInCrossMarginMode => write!(f, "110015"),
            ErrorCodes::QuantityContractsRequestedExceedsRiskLimit => write!(f, "110016"),
            ErrorCodes::ReduceOnlyRuleNotSatisfied => write!(f, "110017"),
            ErrorCodes::UserIdIllegal => write!(f, "110018"),
            ErrorCodes::OrderIdIllegal => write!(f, "110019"),
            ErrorCodes::NotAllowedMoreThan500ActiveOrders => write!(f, "110020"),
            ErrorCodes::NotAllowedExceededPositionLimitsDueToOpenInterest => write!(f, "110021"),
            ErrorCodes::QuantityRestrictedOrdersCannotBeModifiedIncreaseQuantity => write!(f, "110022"),
            ErrorCodes::CurrentlyOnlyReducePositionOnContract => write!(f, "110023"),
            ErrorCodes::ExistingPositionPositionModeCannotBeSwitched => write!(f, "110024"),
            ErrorCodes::PositionModeNotModified => write!(f, "110025"),
            ErrorCodes::CrossIsolatedMarginModeNotModified => write!(f, "110026"),
            ErrorCodes::MarginNotModified => write!(f, "110027"),
            ErrorCodes::ExistingOpenOrdersPositionModeCannotBeSwitched => write!(f, "110028"),
            ErrorCodes::HedgeModeNotSupportedSymbol => write!(f, "110029"),
            ErrorCodes::DuplicateOrderId => write!(f, "110030"),
            ErrorCodes::NonExistingRiskLimitInfo => write!(f, "110031"),
            ErrorCodes::OrderIllegal => write!(f, "110032"),
            ErrorCodes::CannotSetMarginWithoutOpenPosition => write!(f, "110033"),
            ErrorCodes::NoNetPosition => write!(f, "110034"),
            ErrorCodes::CancellationOrdersNotCompletedBeforeLiquidation => write!(f, "110035"),
            ErrorCodes::NotAllowedChangeLeverageDueToCrossMarginMode => write!(f, "110036"),
            ErrorCodes::UserSettingListDoesNotHaveSymbol => write!(f, "110037"),
            ErrorCodes::NotAllowedChangeLeverageDueToPortfolioMarginMode => write!(f, "110038"),
            ErrorCodes::MaintenanceMarginRateTooHighMayTriggerLiquidation => write!(f, "110039"),
            ErrorCodes::OrderWillTriggerForcedLiquidation => write!(f, "110040"),
            ErrorCodes::SkipLiquidationNotAllowedWhenPositionOrMakerOrderExists => write!(f, "110041"),
            ErrorCodes::CurrentlyDueToPreDeliveryStatusOnlyReducePositionOnContract => write!(f, "110042"),
            ErrorCodes::SetLeverageNotModified => write!(f, "110043"),
            ErrorCodes::AvailableMarginInsufficient => write!(f, "110044"),
            ErrorCodes::WalletBalanceInsufficient2 => write!(f, "110045"),
            ErrorCodes::LiquidationTriggeredImmediatelyByAdjustment2 => write!(f, "110046"),
            ErrorCodes::RiskLimitCannotBeAdjustedDueToInsufficientAvailableMargin => write!(f, "110047"),
            ErrorCodes::RiskLimitCannotBeAdjustedCurrentExpectedPositionValueExceedsRevisedRiskLimit => write!(f, "110048"),
            ErrorCodes::TickNotesCanOnlyBeNumbers => write!(f, "110049"),
            ErrorCodes::InvalidCoin => write!(f, "110050"),
            ErrorCodes::UsersAvailableBalanceCannotCoverLowestPriceCurrentMarket => write!(f, "110051"),
            ErrorCodes::UsersAvailableBalanceInsufficientSetPrice => write!(f, "110052"),
            ErrorCodes::UsersAvailableBalanceCannotCoverCurrentMarketPriceUpperLimitPrice => write!(f, "110053"),
            ErrorCodes::PositionHasAtLeastOneTakeProfitLinkOrderTakeProfitStopLossModeCannotBeSwitched => write!(f, "110054"),
            ErrorCodes::PositionHasAtLeastOneStopLossLinkOrderTakeProfitStopLossModeCannotBeSwitched => write!(f, "110055"),
            ErrorCodes::PositionHasAtLeastOneTrailingStopLinkOrderTakeProfitStopLossModeCannotBeSwitched => write!(f, "110056"),
            ErrorCodes::ConditionalOrderLimitOrderContainsTpSlRelatedParams => write!(f, "110057"),
            ErrorCodes::CannotSetTakeProfitStopLossDueToInsufficientSizeRemainingPositionSize => write!(f, "110058"),
            ErrorCodes::NotAllowedModifyTpSlPartiallyFilledOpenOrder => write!(f, "110059"),
            ErrorCodes::UnderFullTpSlModeNotAllowedModifyTpSl => write!(f, "110060"),
            ErrorCodes::NotAllowedMoreThan20TpSlsUnderPartialTpSlMode => write!(f, "110061"),
            ErrorCodes::NoMmpInformationInstitutionFound => write!(f, "110062"),
            ErrorCodes::SettlementInProgressKey0NotAvailableForTrading => write!(f, "110063"),
            ErrorCodes::ModifiedContractQuantityCannotBeLessThanEqualToFilledQuantity => write!(f, "110064"),
            ErrorCodes::MmpNotEnabledForAccountContactBdManager => write!(f, "110065"),
            ErrorCodes::TradingCurrentlyNotAllowed => write!(f, "110066"),
            ErrorCodes::UnifiedAccountNotSupported => write!(f, "110067"),
            ErrorCodes::LeveragedTradingNotAllowed => write!(f, "110068"),
            ErrorCodes::InsLendingCustomerNotAllowedTrade => write!(f, "110069"),
            ErrorCodes::EtpSymbolsCannotBeTraded => write!(f, "110070"),
            ErrorCodes::UnifiedMarginAccountRevampingUpgradesNotSupported => write!(f, "110071"),
            ErrorCodes::OrderLinkedIdDuplicate => write!(f, "110072"),
            ErrorCodes::SetMarginModeFailed => write!(f, "110073"),
            ErrorCodes::RiskIdNotModified => write!(f, "110075"),
            ErrorCodes::CannotEnableSpotMarginWhileInIsolatedMarginModeSwitchCrossMarginModePortfolioMarginModeTradeSpotWithMargin => write!(f, "110075 182021"),
            ErrorCodes::OnlyIsolatedModeCanSetAutoAddMargin => write!(f, "110076"),
            ErrorCodes::PmModeCannotSupport => write!(f, "110077"),
            ErrorCodes::AddedMarginMoreThanMaxCanReduceMargin => write!(f, "110078"),
            ErrorCodes::OrderProcessingCannotOperatedTryAgainLater => write!(f, "110079"),
            ErrorCodes::OperationsRestrictionCurrentLtvRatioInstitutionalLendingHitLiquidationThresholdAssetsLiquidatedTradeRiskLimitLeverage => write!(f, "110080"),
            ErrorCodes::CannotLiftReduceOnlyRestrictionsNoReduceOnlyRestrictionsAppliedPosition => write!(f, "110082"),
            ErrorCodes::ReduceOnlyRestrictionsMustLiftBothLongShortPositionsSameTime => write!(f, "110083"),
            ErrorCodes::RiskLimitMarginRatioContractUpdatedSelectSupportedRiskLimitPlaceOrder => write!(f, "110085"),
            ErrorCodes::CurrentOrderLeverageExceedsMaxAvailableCurrentRiskLimitTierLowerLeveragePlacingOrder => write!(f, "110086"),
            ErrorCodes::UpgradeToUtaTrade => write!(f, "110087"),
            ErrorCodes::ExceedsMaxRiskLimitLevel => write!(f, "110089"),
            ErrorCodes::ExceedsMaxLeverageLimitCurrentRiskLimitLevel => write!(f, "110090"),
            ErrorCodes::ExpectRisingTriggerPriceCurrentLast => write!(f, "110092"),
            ErrorCodes::ExpectFallingTriggerPriceCurrentLast => write!(f, "110093"),
            ErrorCodes::OrderNotionalValueBelowLowerLimit => write!(f, "110094"),
            ErrorCodes::OrderStatusMustBeFinalStatus => write!(f, "181017"),
            ErrorCodes::CompulsoryClosingPositionsNoRepaymentAllowed => write!(f, "182100"),
            ErrorCodes::FailedRepaymentInsufficientCollateralBalance => write!(f, "182101"),
            ErrorCodes::FailedRepaymentNoLiabilitiesCurrentCurrency => write!(f, "182102"),
            ErrorCodes::InstitutionalLendingUsersNotSupported => write!(f, "182103"),
            ErrorCodes::SwitchingFailedMarginVerificationFailedReadjustCurrencyStatus => write!(f, "182108"),
            ErrorCodes::FailedSwitch => write!(f, "182110"),
            ErrorCodes::RequestedCurrencyNonGuaranteedGoldCurrencyNotSupportSwitchingStatusCurrencies => write!(f, "182111"),
            ErrorCodes::DuplicateCurrencyReadjust => write!(f, "182112"),
            ErrorCodes::UidCanNotNull => write!(f, "3100181"),
            ErrorCodes::TemporaryBannedUpgradeUta => write!(f, "3100197"),
            ErrorCodes::UsdcOptionsTradingRestrictionCurrentLtvRatioInstitutionalLendingReachedMaxAllowableAmountUsdcOptionsTrading => write!(f, "3200316"),
            ErrorCodes::UsdcOptionsOpenPositionRestrictionCurrentLtvRatioInstitutionalLendingReachedMaxAllowableAmountOpeningUsdcOptionsPositions => write!(f, "3200317"),
            ErrorCodes::BaseCoinRequired => write!(f, "3100326"),
            ErrorCodes::IsolatedMarginCannotCreateOrder => write!(f, "3200403"),
            ErrorCodes::OperationsRestrictionCurrentLtvRatioInstitutionalLendingHitLiquidationThresholdAssetsLiquidatedMarginModeSpotLeverage => write!(f, "3200320"),
            ErrorCodes::UnclosedHedgeModeIsolatedModeUsdtPerpetualPositions => write!(f, "3400208"),
            ErrorCodes::UsdtPerpetualPositionsUpgradingProhibited10MinutesBeforeAfterHourEveryHour => write!(f, "3400209"),
            ErrorCodes::RiskRateDerivativesAccountTooHigh => write!(f, "3400210"),
            ErrorCodes::UpgradedEstimatedRiskRateTooHigh => write!(f, "3400211"),
            ErrorCodes::UsdcPerpetualPositionsOptionsPositionsUpgradingProhibited10MinutesBeforeAfterHourEveryHour => write!(f, "3400212"),
            ErrorCodes::RiskRateUsdcDerivativesAccountTooHigh => write!(f, "3400213"),
            ErrorCodes::UncancelledUsdcPerpetualOrders => write!(f, "3400052"),
            ErrorCodes::UncancelledOptionsOrders => write!(f, "3400053"),
            ErrorCodes::UncancelledUsdtPerpetualOrders => write!(f, "3400054"),
            ErrorCodes::ServerErrorTryAgainLater => write!(f, "3400214"),
            ErrorCodes::NetAssetNotSatisfied => write!(f, "3400071"),
            ErrorCodes::CannotSwitchPmModeCopyTradingMasterTrader => write!(f, "3401010"),
            ErrorCodes::TotalValuePositionsOrdersExceededRiskLimitPerpetualFuturesContract => write!(f, "3400139"),
            ErrorCodes::SubAccountNotBelongParentAccount => write!(f, "500010"),
            ErrorCodes::UidNotAssociatedUnifiedTradingAccount => write!(f, "500011"),
        }
    }
}

impl ErrorCodes {
    pub fn get_error_type(&self) -> ErrorTypes {
        match self {
            // HTTP error codes
            ErrorCodes::BadRequest => ErrorTypes::Http,
            ErrorCodes::InvalidRequest => ErrorTypes::Http,
            ErrorCodes::ForbiddenRequest => ErrorTypes::Http,
            ErrorCodes::CannotFindPath => ErrorTypes::Http,

            // UTA & Classic Account error codes
            ErrorCodes::Ok => ErrorTypes::UTAClassicAccount,
            ErrorCodes::ServerTimeout => ErrorTypes::UTAClassicAccount,
            ErrorCodes::RequestParameterError => ErrorTypes::UTAClassicAccount,
            ErrorCodes::RequestTimeExceedsTimeWindowRange => ErrorTypes::UTAClassicAccount,
            ErrorCodes::ApiKeyInvalid => ErrorTypes::UTAClassicAccount,
            ErrorCodes::ApiKeyExpired => ErrorTypes::UTAClassicAccount,
            ErrorCodes::ErrorSign => ErrorTypes::UTAClassicAccount,
            ErrorCodes::PermissionDenied => ErrorTypes::UTAClassicAccount,
            ErrorCodes::TooManyVisits => ErrorTypes::UTAClassicAccount,
            ErrorCodes::UserAuthenticationFailed => ErrorTypes::UTAClassicAccount,
            ErrorCodes::CommonBanned => ErrorTypes::UTAClassicAccount,
            ErrorCodes::IpBanned => ErrorTypes::UTAClassicAccount,
            ErrorCodes::UnmatchedIp => ErrorTypes::UTAClassicAccount,
            ErrorCodes::InvalidDuplicateRequest => ErrorTypes::UTAClassicAccount,
            ErrorCodes::ServerError => ErrorTypes::UTAClassicAccount,
            ErrorCodes::RouteNotFound => ErrorTypes::UTAClassicAccount,
            ErrorCodes::ExceededIpRateLimit => ErrorTypes::UTAClassicAccount,
            ErrorCodes::ComplianceRulesTriggered => ErrorTypes::UTAClassicAccount,
            ErrorCodes::TransactionsBanned => ErrorTypes::UTAClassicAccount,
            ErrorCodes::InvalidSymbol => ErrorTypes::UTAClassicAccount,
            ErrorCodes::ApiAccessUnifiedAccount => ErrorTypes::UTAClassicAccount,
            ErrorCodes::OtcLoanUsdtPerpetualSymbolNotAllowed => ErrorTypes::UTAClassicAccount,
            ErrorCodes::OtcLoanUsdcContractSymbolNotAllowed => ErrorTypes::UTAClassicAccount,
            ErrorCodes::LeverageSelectUsdtPerpetualTradingCannotExceedMaxLeverage => ErrorTypes::UTAClassicAccount,
            ErrorCodes::LeverageSelectUsdcPerpetualOrFuturesTradingCannotExceedMaxLeverage => ErrorTypes::UTAClassicAccount,
            ErrorCodes::OrderModifiedDuringProcessOfReplacing => ErrorTypes::UTAClassicAccount,
            ErrorCodes::ApiAccessUnifiedAccountUsers => ErrorTypes::UTAClassicAccount,
            ErrorCodes::OrderDoesNotExist => ErrorTypes::UTAClassicAccount,
            ErrorCodes::OrderPriceExceedsAllowableRange => ErrorTypes::UTAClassicAccount,
            ErrorCodes::WalletBalanceInsufficient1 => ErrorTypes::UTAClassicAccount,
            ErrorCodes::PositionStatus => ErrorTypes::UTAClassicAccount,
            ErrorCodes::AssetsEstimatedUnableCoverPositionMargin => ErrorTypes::UTAClassicAccount,
            ErrorCodes::AvailableBalanceInsufficient => ErrorTypes::UTAClassicAccount,
            ErrorCodes::OrderCompletedOrCancelled => ErrorTypes::UTAClassicAccount,
            ErrorCodes::NumberOfStopOrdersExceedsMaxAllowableLimit => ErrorTypes::UTAClassicAccount,
            ErrorCodes::OrderCancelled => ErrorTypes::UTAClassicAccount,
            ErrorCodes::LiquidationTriggeredImmediatelyByAdjustment1 => ErrorTypes::UTAClassicAccount,
            ErrorCodes::InsufficientAvailableBalance => ErrorTypes::UTAClassicAccount,
            ErrorCodes::CannotSetLeverageDueToRiskLimitLevel => ErrorTypes::UTAClassicAccount,
            ErrorCodes::InsufficientAvailableBalanceToAddAdditionalMargin => ErrorTypes::UTAClassicAccount,
            ErrorCodes::PositionInCrossMarginMode => ErrorTypes::UTAClassicAccount,
            ErrorCodes::QuantityContractsRequestedExceedsRiskLimit => ErrorTypes::UTAClassicAccount,
            ErrorCodes::ReduceOnlyRuleNotSatisfied => ErrorTypes::UTAClassicAccount,
            ErrorCodes::UserIdIllegal => ErrorTypes::UTAClassicAccount,
            ErrorCodes::OrderIdIllegal => ErrorTypes::UTAClassicAccount,
            ErrorCodes::NotAllowedMoreThan500ActiveOrders => ErrorTypes::UTAClassicAccount,
            ErrorCodes::NotAllowedExceededPositionLimitsDueToOpenInterest => ErrorTypes::UTAClassicAccount,
            ErrorCodes::QuantityRestrictedOrdersCannotBeModifiedIncreaseQuantity => ErrorTypes::UTAClassicAccount,
            ErrorCodes::CurrentlyOnlyReducePositionOnContract => ErrorTypes::UTAClassicAccount,
            ErrorCodes::ExistingPositionPositionModeCannotBeSwitched => ErrorTypes::UTAClassicAccount,
            ErrorCodes::PositionModeNotModified => ErrorTypes::UTAClassicAccount,
            ErrorCodes::CrossIsolatedMarginModeNotModified => ErrorTypes::UTAClassicAccount,
            ErrorCodes::MarginNotModified => ErrorTypes::UTAClassicAccount,
            ErrorCodes::ExistingOpenOrdersPositionModeCannotBeSwitched => ErrorTypes::UTAClassicAccount,
            ErrorCodes::HedgeModeNotSupportedSymbol => ErrorTypes::UTAClassicAccount,
            ErrorCodes::DuplicateOrderId => ErrorTypes::UTAClassicAccount,
            ErrorCodes::NonExistingRiskLimitInfo => ErrorTypes::UTAClassicAccount,
            ErrorCodes::OrderIllegal => ErrorTypes::UTAClassicAccount,
            ErrorCodes::CannotSetMarginWithoutOpenPosition => ErrorTypes::UTAClassicAccount,
            ErrorCodes::NoNetPosition => ErrorTypes::UTAClassicAccount,
            ErrorCodes::CancellationOrdersNotCompletedBeforeLiquidation => ErrorTypes::UTAClassicAccount,
            ErrorCodes::NotAllowedChangeLeverageDueToCrossMarginMode => ErrorTypes::UTAClassicAccount,
            ErrorCodes::UserSettingListDoesNotHaveSymbol => ErrorTypes::UTAClassicAccount,
            ErrorCodes::NotAllowedChangeLeverageDueToPortfolioMarginMode => ErrorTypes::UTAClassicAccount,
            ErrorCodes::MaintenanceMarginRateTooHighMayTriggerLiquidation => ErrorTypes::UTAClassicAccount,
            ErrorCodes::OrderWillTriggerForcedLiquidation => ErrorTypes::UTAClassicAccount,
            ErrorCodes::SkipLiquidationNotAllowedWhenPositionOrMakerOrderExists => ErrorTypes::UTAClassicAccount,
            ErrorCodes::CurrentlyDueToPreDeliveryStatusOnlyReducePositionOnContract => ErrorTypes::UTAClassicAccount,
            ErrorCodes::SetLeverageNotModified => ErrorTypes::UTAClassicAccount,
            ErrorCodes::AvailableMarginInsufficient => ErrorTypes::UTAClassicAccount,
            ErrorCodes::WalletBalanceInsufficient2 => ErrorTypes::UTAClassicAccount,
            ErrorCodes::LiquidationTriggeredImmediatelyByAdjustment2 => ErrorTypes::UTAClassicAccount,
            ErrorCodes::RiskLimitCannotBeAdjustedDueToInsufficientAvailableMargin => ErrorTypes::UTAClassicAccount,
            ErrorCodes::RiskLimitCannotBeAdjustedCurrentExpectedPositionValueExceedsRevisedRiskLimit => ErrorTypes::UTAClassicAccount,
            ErrorCodes::TickNotesCanOnlyBeNumbers => ErrorTypes::UTAClassicAccount,
            ErrorCodes::InvalidCoin => ErrorTypes::UTAClassicAccount,
            ErrorCodes::UsersAvailableBalanceCannotCoverLowestPriceCurrentMarket => ErrorTypes::UTAClassicAccount,
            ErrorCodes::UsersAvailableBalanceInsufficientSetPrice => ErrorTypes::UTAClassicAccount,
            ErrorCodes::UsersAvailableBalanceCannotCoverCurrentMarketPriceUpperLimitPrice => ErrorTypes::UTAClassicAccount,
            ErrorCodes::PositionHasAtLeastOneTakeProfitLinkOrderTakeProfitStopLossModeCannotBeSwitched => ErrorTypes::UTAClassicAccount,
            ErrorCodes::PositionHasAtLeastOneStopLossLinkOrderTakeProfitStopLossModeCannotBeSwitched => ErrorTypes::UTAClassicAccount,
            ErrorCodes::PositionHasAtLeastOneTrailingStopLinkOrderTakeProfitStopLossModeCannotBeSwitched => ErrorTypes::UTAClassicAccount,
            ErrorCodes::ConditionalOrderLimitOrderContainsTpSlRelatedParams => ErrorTypes::UTAClassicAccount,
            ErrorCodes::CannotSetTakeProfitStopLossDueToInsufficientSizeRemainingPositionSize => ErrorTypes::UTAClassicAccount,
            ErrorCodes::NotAllowedModifyTpSlPartiallyFilledOpenOrder => ErrorTypes::UTAClassicAccount,
            ErrorCodes::UnderFullTpSlModeNotAllowedModifyTpSl => ErrorTypes::UTAClassicAccount,
            ErrorCodes::NotAllowedMoreThan20TpSlsUnderPartialTpSlMode => ErrorTypes::UTAClassicAccount,
            ErrorCodes::NoMmpInformationInstitutionFound => ErrorTypes::UTAClassicAccount,
            ErrorCodes::SettlementInProgressKey0NotAvailableForTrading => ErrorTypes::UTAClassicAccount,
            ErrorCodes::ModifiedContractQuantityCannotBeLessThanEqualToFilledQuantity => ErrorTypes::UTAClassicAccount,
            ErrorCodes::MmpNotEnabledForAccountContactBdManager => ErrorTypes::UTAClassicAccount,
            ErrorCodes::TradingCurrentlyNotAllowed => ErrorTypes::UTAClassicAccount,
            ErrorCodes::UnifiedAccountNotSupported => ErrorTypes::UTAClassicAccount,
            ErrorCodes::LeveragedTradingNotAllowed => ErrorTypes::UTAClassicAccount,
            ErrorCodes::InsLendingCustomerNotAllowedTrade => ErrorTypes::UTAClassicAccount,
            ErrorCodes::EtpSymbolsCannotBeTraded => ErrorTypes::UTAClassicAccount,
            ErrorCodes::UnifiedMarginAccountRevampingUpgradesNotSupported => ErrorTypes::UTAClassicAccount,
            ErrorCodes::OrderLinkedIdDuplicate => ErrorTypes::UTAClassicAccount,
            ErrorCodes::SetMarginModeFailed => ErrorTypes::UTAClassicAccount,
            ErrorCodes::RiskIdNotModified => ErrorTypes::UTAClassicAccount,
            ErrorCodes::CannotEnableSpotMarginWhileInIsolatedMarginModeSwitchCrossMarginModePortfolioMarginModeTradeSpotWithMargin => ErrorTypes::UTAClassicAccount,
            ErrorCodes::OnlyIsolatedModeCanSetAutoAddMargin => ErrorTypes::UTAClassicAccount,
            ErrorCodes::PmModeCannotSupport => ErrorTypes::UTAClassicAccount,
            ErrorCodes::AddedMarginMoreThanMaxCanReduceMargin => ErrorTypes::UTAClassicAccount,
            ErrorCodes::OrderProcessingCannotOperatedTryAgainLater => ErrorTypes::UTAClassicAccount,
            ErrorCodes::OperationsRestrictionCurrentLtvRatioInstitutionalLendingHitLiquidationThresholdAssetsLiquidatedTradeRiskLimitLeverage => ErrorTypes::UTAClassicAccount,
            ErrorCodes::CannotLiftReduceOnlyRestrictionsNoReduceOnlyRestrictionsAppliedPosition => ErrorTypes::UTAClassicAccount,
            ErrorCodes::ReduceOnlyRestrictionsMustLiftBothLongShortPositionsSameTime => ErrorTypes::UTAClassicAccount,
            ErrorCodes::RiskLimitMarginRatioContractUpdatedSelectSupportedRiskLimitPlaceOrder => ErrorTypes::UTAClassicAccount,
            ErrorCodes::CurrentOrderLeverageExceedsMaxAvailableCurrentRiskLimitTierLowerLeveragePlacingOrder => ErrorTypes::UTAClassicAccount,
            ErrorCodes::UpgradeToUtaTrade => ErrorTypes::UTAClassicAccount,
            ErrorCodes::ExceedsMaxRiskLimitLevel => ErrorTypes::UTAClassicAccount,
            ErrorCodes::ExceedsMaxLeverageLimitCurrentRiskLimitLevel => ErrorTypes::UTAClassicAccount,
            ErrorCodes::ExpectRisingTriggerPriceCurrentLast => ErrorTypes::UTAClassicAccount,
            ErrorCodes::ExpectFallingTriggerPriceCurrentLast => ErrorTypes::UTAClassicAccount,
            ErrorCodes::OrderNotionalValueBelowLowerLimit => ErrorTypes::UTAClassicAccount,
            ErrorCodes::OrderStatusMustBeFinalStatus => ErrorTypes::UTAClassicAccount,
            ErrorCodes::CompulsoryClosingPositionsNoRepaymentAllowed => ErrorTypes::UTAClassicAccount,
            ErrorCodes::FailedRepaymentInsufficientCollateralBalance => ErrorTypes::UTAClassicAccount,
            ErrorCodes::FailedRepaymentNoLiabilitiesCurrentCurrency => ErrorTypes::UTAClassicAccount,
            ErrorCodes::InstitutionalLendingUsersNotSupported => ErrorTypes::UTAClassicAccount,
            ErrorCodes::SwitchingFailedMarginVerificationFailedReadjustCurrencyStatus => ErrorTypes::UTAClassicAccount,
            ErrorCodes::FailedSwitch => ErrorTypes::UTAClassicAccount,
            ErrorCodes::RequestedCurrencyNonGuaranteedGoldCurrencyNotSupportSwitchingStatusCurrencies => ErrorTypes::UTAClassicAccount,
            ErrorCodes::DuplicateCurrencyReadjust => ErrorTypes::UTAClassicAccount,
            ErrorCodes::UidCanNotNull => ErrorTypes::UTAClassicAccount,
            ErrorCodes::TemporaryBannedUpgradeUta => ErrorTypes::UTAClassicAccount,
            ErrorCodes::UsdcOptionsTradingRestrictionCurrentLtvRatioInstitutionalLendingReachedMaxAllowableAmountUsdcOptionsTrading => ErrorTypes::UTAClassicAccount,
            ErrorCodes::UsdcOptionsOpenPositionRestrictionCurrentLtvRatioInstitutionalLendingReachedMaxAllowableAmountOpeningUsdcOptionsPositions => ErrorTypes::UTAClassicAccount,
            ErrorCodes::BaseCoinRequired => ErrorTypes::UTAClassicAccount,
            ErrorCodes::IsolatedMarginCannotCreateOrder => ErrorTypes::UTAClassicAccount,
            ErrorCodes::OperationsRestrictionCurrentLtvRatioInstitutionalLendingHitLiquidationThresholdAssetsLiquidatedMarginModeSpotLeverage => ErrorTypes::UTAClassicAccount,
            ErrorCodes::UnclosedHedgeModeIsolatedModeUsdtPerpetualPositions => ErrorTypes::UTAClassicAccount,
            ErrorCodes::UsdtPerpetualPositionsUpgradingProhibited10MinutesBeforeAfterHourEveryHour => ErrorTypes::UTAClassicAccount,
            ErrorCodes::RiskRateDerivativesAccountTooHigh => ErrorTypes::UTAClassicAccount,
            ErrorCodes::UpgradedEstimatedRiskRateTooHigh => ErrorTypes::UTAClassicAccount,
            ErrorCodes::UsdcPerpetualPositionsOptionsPositionsUpgradingProhibited10MinutesBeforeAfterHourEveryHour => ErrorTypes::UTAClassicAccount,
            ErrorCodes::RiskRateUsdcDerivativesAccountTooHigh => ErrorTypes::UTAClassicAccount,
            ErrorCodes::UncancelledUsdcPerpetualOrders => ErrorTypes::UTAClassicAccount,
            ErrorCodes::UncancelledOptionsOrders => ErrorTypes::UTAClassicAccount,
            ErrorCodes::UncancelledUsdtPerpetualOrders => ErrorTypes::UTAClassicAccount,
            ErrorCodes::ServerErrorTryAgainLater => ErrorTypes::UTAClassicAccount,
            ErrorCodes::NetAssetNotSatisfied => ErrorTypes::UTAClassicAccount,
            ErrorCodes::CannotSwitchPmModeCopyTradingMasterTrader => ErrorTypes::UTAClassicAccount,
            ErrorCodes::TotalValuePositionsOrdersExceededRiskLimitPerpetualFuturesContract => ErrorTypes::UTAClassicAccount,
            ErrorCodes::SubAccountNotBelongParentAccount => ErrorTypes::UTAClassicAccount,
            ErrorCodes::UidNotAssociatedUnifiedTradingAccount => ErrorTypes::UTAClassicAccount,
        }
    }
}
