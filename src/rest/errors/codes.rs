use std::fmt::{Display, Formatter, Result};

use serde::{Deserialize, Serialize};

use crate::rest::errors::types::ErrorTypes;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum ErrorCodes {
    // HTTP error codes
    #[serde(rename = "400")]
    E400, // Bad request. Need to send the request with GET / POST (must be capitalized)
    #[serde(rename = "401")]
    E401, // Invalid request. 1. Need to use the correct key to access; 2. Need to put authentication params in the request header
    #[serde(rename = "403")]
    E403, // Forbidden request. Possible causes: 1. IP rate limit breached; 2. You send GET request with an empty json body; 3. You are using U.S IP
    #[serde(rename = "404")]
    E404, // Cannot find path. Possible causes: 1. Wrong path; 2. Category value does not match account mode

    // UTA & Classic Account error codes
    #[serde(rename = "0")]
    E0, // Ok
    #[serde(rename = "10000")]
    E10000, // Server Timeout
    #[serde(rename = "10001")]
    E10001, // Request parameter error
    #[serde(rename = "10002")]
    E10002, // The request time exceeds the time window range.
    #[serde(rename = "10003")]
    E10003, // API key is invalid. Check whether the key and domain are matched, there are 4 env: mainnet, testnet, mainnet-demo, testnet-demo
    #[serde(rename = "33004")]
    E33004, // Your api key has expired
    #[serde(rename = "10004")]
    E10004, // Error sign, please check your signature generation algorithm.
    #[serde(rename = "10005")]
    E10005, // Permission denied, please check your API key permissions.
    #[serde(rename = "10006")]
    E10006, // Too many visits. Exceeded the API Rate Limit.
    #[serde(rename = "10007")]
    E10007, // User authentication failed.
    #[serde(rename = "10008")]
    E10008, // Common banned, please check your account mode
    #[serde(rename = "10009")]
    E10009, // IP has been banned.
    #[serde(rename = "10010")]
    E10010, // Unmatched IP, please check your API key's bound IP addresses.
    #[serde(rename = "10014")]
    E10014, // Invalid duplicate request.
    #[serde(rename = "10016")]
    E10016, // Server error.
    #[serde(rename = "10017")]
    E10017, // Route not found.
    #[serde(rename = "10018")]
    E10018, // Exceeded the IP Rate Limit.
    #[serde(rename = "10024")]
    E10024, // Compliance rules triggered
    #[serde(rename = "10027")]
    E10027, // Transactions are banned.
    #[serde(rename = "10029")]
    E10029, // The requested symbol is invalid, please check symbol whitelist
    #[serde(rename = "10028")]
    E10028, // The API can only be accessed by unified account users.
    #[serde(rename = "30133")]
    E30133, // OTC loan: The symbol you select for USDT Perpetual is not allowed by Institutional Lending
    #[serde(rename = "30134")]
    E30134, // OTC loan: The symbol you select for USDC Contract is not allowed by Institutional Lending
    #[serde(rename = "30135")]
    E30135, // The leverage you select for USDT Perpetual trading cannot exceed the maximum leverage allowed by Institutional Lending.
    #[serde(rename = "30136")]
    E30136, // The leverage you select for USDC Perpetual or Futures trading cannot exceed the maximum leverage allowed by Institutional Lending.
    #[serde(rename = "40004")]
    E40004, // the order is modified during the process of replacing , please check the order status again
    #[serde(rename = "100028")]
    E100028, // The API cannot be accessed by unified account users.
    #[serde(rename = "110001")]
    E110001, // Order does not exist
    #[serde(rename = "110003")]
    E110003, // Order price exceeds the allowable range.
    #[serde(rename = "110004")]
    E110004, // Wallet balance is insufficient
    #[serde(rename = "110005")]
    E110005, // position status
    #[serde(rename = "110006")]
    E110006, // The assets are estimated to be unable to cover the position margin
    #[serde(rename = "110007")]
    E110007, // Available balance is insufficient
    #[serde(rename = "110008")]
    E110008, // The order has been completed or cancelled.
    #[serde(rename = "110009")]
    E110009, // The number of stop orders exceeds the maximum allowable limit. You can find references in our API doc.
    #[serde(rename = "110010")]
    E110010, // The order has been cancelled
    #[serde(rename = "110011")]
    E110011, // Liquidation will be triggered immediately by this adjustment
    #[serde(rename = "110012")]
    E110012, // Insufficient available balance.
    #[serde(rename = "110013")]
    E110013, // Cannot set leverage due to risk limit level.
    #[serde(rename = "110014")]
    E110014, // Insufficient available balance to add additional margin.
    #[serde(rename = "110015")]
    E110015, // The position is in cross margin mode.
    #[serde(rename = "110016")]
    E110016, // The quantity of contracts requested exceeds the risk limit, please adjust your risk limit level before trying again
    #[serde(rename = "110017")]
    E110017, // Reduce-only rule not satisfied
    #[serde(rename = "110018")]
    E110018, // User ID is illegal.
    #[serde(rename = "110019")]
    E110019, // Order ID is illegal.
    #[serde(rename = "110020")]
    E110020, // Not allowed to have more than 500 active orders.
    #[serde(rename = "110021")]
    E110021, // Not allowed to exceeded position limits due to Open Interest.
    #[serde(rename = "110022")]
    E110022, // Quantity has been restricted and orders cannot be modified to increase the quantity.
    #[serde(rename = "110023")]
    E110023, // Currently you can only reduce your position on this contract. please check our announcement or contact customer service for details.
    #[serde(rename = "110024")]
    E110024, // You have an existing position, so the position mode cannot be switched.
    #[serde(rename = "110025")]
    E110025, // Position mode has not been modified.
    #[serde(rename = "110026")]
    E110026, // Cross/isolated margin mode has not been modified.
    #[serde(rename = "110027")]
    E110027, // Margin has not been modified.
    #[serde(rename = "110028")]
    E110028, // You have existing open orders, so the position mode cannot be switched.
    #[serde(rename = "110029")]
    E110029, // Hedge mode is not supported for this symbol.
    #[serde(rename = "110030")]
    E110030, // Duplicate orderId
    #[serde(rename = "110031")]
    E110031, // Non-existing risk limit info, please check the risk limit rules.
    #[serde(rename = "110032")]
    E110032, // Order is illegal
    #[serde(rename = "110033")]
    E110033, // You can't set margin without an open position
    #[serde(rename = "110034")]
    E110034, // There is no net position
    #[serde(rename = "110035")]
    E110035, // Cancellation of orders was not completed before liquidation
    #[serde(rename = "110036")]
    E110036, // You are not allowed to change leverage due to cross margin mode.
    #[serde(rename = "110037")]
    E110037, // User setting list does not have this symbol
    #[serde(rename = "110038")]
    E110038, // You are not allowed to change leverage due to portfolio margin mode.
    #[serde(rename = "110039")]
    E110039, // Maintenance margin rate is too high. This may trigger liquidation.
    #[serde(rename = "110040")]
    E110040, // The order will trigger a forced liquidation, please re-submit the order.
    #[serde(rename = "110041")]
    E110041, // Skip liquidation is not allowed when a position or maker order exists
    #[serde(rename = "110042")]
    E110042, // Currently,due to pre-delivery status, you can only reduce your position on this contract.
    #[serde(rename = "110043")]
    E110043, // Set leverage has not been modified.
    #[serde(rename = "110044")]
    E110044, // Available margin is insufficient.
    #[serde(rename = "110045")]
    E110045, // Wallet balance is insufficient.
    #[serde(rename = "110046")]
    E110046, // Liquidation will be triggered immediately by this adjustment.
    #[serde(rename = "110047")]
    E110047, // Risk limit cannot be adjusted due to insufficient available margin.
    #[serde(rename = "110048")]
    E110048, // Risk limit cannot be adjusted as the current/expected position value exceeds the revised risk limit.
    #[serde(rename = "110049")]
    E110049, // Tick notes can only be numbers
    #[serde(rename = "110050")]
    E110050, // Invalid coin
    #[serde(rename = "110051")]
    E110051, // The user's available balance cannot cover the lowest price of the current market
    #[serde(rename = "110052")]
    E110052, // Your available balance is insufficient to set the price
    #[serde(rename = "110053")]
    E110053, // The user's available balance cannot cover the current market price and upper limit price
    #[serde(rename = "110054")]
    E110054, // This position has at least one take profit link order, so the take profit and stop loss mode cannot be switched
    #[serde(rename = "110055")]
    E110055, // This position has at least one stop loss link order, so the take profit and stop loss mode cannot be switched
    #[serde(rename = "110056")]
    E110056, // This position has at least one trailing stop link order, so the take profit and stop loss mode cannot be switched
    #[serde(rename = "110057")]
    E110057, // Conditional order or limit order contains TP/SL related params
    #[serde(rename = "110058")]
    E110058, // You can't set take profit and stop loss due to insufficient size of remaining position size.
    #[serde(rename = "110059")]
    E110059, // Not allowed to modify the TP/SL of a partially filled open order
    #[serde(rename = "110060")]
    E110060, // Under full TP/SL mode, it is not allowed to modify TP/SL
    #[serde(rename = "110061")]
    E110061, // Not allowed to have more than 20 TP/SLs under Partial tpSlMode
    #[serde(rename = "110062")]
    E110062, // There is no MMP information of the institution found.
    #[serde(rename = "110063")]
    E110063, // Settlement in progress! {{key0}} not available for trading.
    #[serde(rename = "110064")]
    E110064, // The modified contract quantity cannot be less than or equal to the filled quantity.
    #[serde(rename = "110065")]
    E110065, // MMP hasn't yet been enabled for your account. Please contact your BD manager.
    #[serde(rename = "110066")]
    E110066, // Trading is currently not allowed.
    #[serde(rename = "110067")]
    E110067, // Unified account is not supported.
    #[serde(rename = "110068")]
    E110068, // Leveraged trading is not allowed.
    #[serde(rename = "110069")]
    E110069, // Ins lending customer is not allowed to trade.
    #[serde(rename = "110070")]
    E110070, // ETP symbols cannot be traded.
    #[serde(rename = "110071")]
    E110071, // Sorry, we're revamping the Unified Margin Account! Currently, new upgrades are not supported. If you have any questions, please contact our 24/7 customer support.
    #[serde(rename = "110072")]
    E110072, // OrderLinkedID is duplicate
    #[serde(rename = "110073")]
    E110073, // Set margin mode failed
    #[serde(rename = "110075")]
    E110075, // RiskId not modified
    #[serde(rename = "182021")]
    E182021, // Cannot enable spot margin while in isolated margin mode. Please switch to cross margin mode or portfolio margin mode to trade spot with margin.
    #[serde(rename = "110076")]
    E110076, // Only isolated mode can set auto-add-margin
    #[serde(rename = "110077")]
    E110077, // Pm mode cannot support
    #[serde(rename = "110078")]
    E110078, // Added margin more than max can reduce margin
    #[serde(rename = "110079")]
    E110079, // The order is processing and can not be operated, please try again later
    #[serde(rename = "110080")]
    E110080, // Operations Restriction: The current LTV ratio of your Institutional Lending has hit the liquidation threshold. Assets in your account are being liquidated (trade/risk limit/leverage)
    #[serde(rename = "110082")]
    E110082, // You cannot lift Reduce-Only restrictions, as no Reduce-Only restrictions are applied to your position
    #[serde(rename = "110083")]
    E110083, // Reduce-Only restrictions must be lifted for both Long and Short positions at the same time
    #[serde(rename = "110085")]
    E110085, // The risk limit and margin ratio for this contract has been updated, please select a supported risk limit and place your order again
    #[serde(rename = "110086")]
    E110086, // Current order leverage exceeds the maximum available for your current Risk Limit tier. Please lower leverage before placing an order
    #[serde(rename = "110087")]
    E110087, // Leverage for Perpetual or Futures contracts cannot exceed the maximum allowed for your Institutional loan
    #[serde(rename = "110088")]
    E110088, // Please Upgrade to UTA to trade
    #[serde(rename = "110089")]
    E110089, // Exceeds the maximum risk limit level
    #[serde(rename = "110090")]
    E110090, // Exceeds the maximum leverage limit of the current risk limit level.
    #[serde(rename = "110092")]
    E110092, // expect Rising, but trigger_price[XXXXX] <= current[XXXXX]??laste
    #[serde(rename = "110093")]
    E110093, // expect Falling, but trigger_price[XXXXX] >= current[XXXXX]??last
    #[serde(rename = "110094")]
    E110094, // Order notional value below the lower limit
    #[serde(rename = "181017")]
    E181017, // OrderStatus must be final status
    #[serde(rename = "182100")]
    E182100, // Compulsory closing of positions, no repayment allowed
    #[serde(rename = "182101")]
    E182101, // Failed repayment, insufficient collateral balance
    #[serde(rename = "182102")]
    E182102, // Failed repayment, there are no liabilities in the current currency
    #[serde(rename = "182103")]
    E182103, // Institutional lending users are not supported
    #[serde(rename = "182108")]
    E182108, // Switching failed, margin verification failed, please re-adjust the currency status
    #[serde(rename = "182110")]
    E182110, // Failed to switch
    #[serde(rename = "182111")]
    E182111, // The requested currency has a non guaranteed gold currency or does not support switching status currencies
    #[serde(rename = "182112")]
    E182112, // Duplicate currency, please re-adjust
    #[serde(rename = "3100181")]
    E3100181, // UID can not be null
    #[serde(rename = "3100197")]
    E3100197, // Temporary banned due to the upgrade to UTA
    #[serde(rename = "3200316")]
    E3200316, // USDC Options Trading Restriction: The current LTV ratio for your Institutional Lending has reached the maximum allowable amount for USDC Options trading.
    #[serde(rename = "3200317")]
    E3200317, // USDC Options Open Position Restriction: The current LTV ratio for your Institutional Lending has reached the maximum allowable amount for opening USDC Options positions.
    #[serde(rename = "3100326")]
    E3100326, // BaseCoin is required
    #[serde(rename = "3200403")]
    E3200403, // isolated margin can not create order
    #[serde(rename = "3200320")]
    E3200320, // Operations Restriction: The current LTV ratio of your Institutional Lending has hit the liquidation threshold. Assets in your account are being liquidated. (margin mode or spot leverage)
    #[serde(rename = "3400208")]
    E3400208, // You have unclosed hedge mode or isolated mode USDT perpetual positions
    #[serde(rename = "3400209")]
    E3400209, // You have USDT perpetual positions, so upgrading is prohibited for 10 minutes before and after the hour every hour
    #[serde(rename = "3400210")]
    E3400210, // The risk rate of your Derivatives account is too high
    #[serde(rename = "3400211")]
    E3400211, // Once upgraded, the estimated risk rate will be too high
    #[serde(rename = "3400212")]
    E3400212, // You have USDC perpetual positions or Options positions, so upgrading is prohibited for 10 minutes before and after the hour every hour
    #[serde(rename = "3400213")]
    E3400213, // The risk rate of your USDC Derivatives account is too high
    #[serde(rename = "3400052")]
    E3400052, // You have uncancelled USDC perpetual orders
    #[serde(rename = "3400053")]
    E3400053, // You have uncancelled Options orders
    #[serde(rename = "3400054")]
    E3400054, // You have uncancelled USDT perpetual orders
    #[serde(rename = "3400214")]
    E3400214, // Server error, please try again later
    #[serde(rename = "3400071")]
    E3400071, // The net asset is not satisfied
    #[serde(rename = "3401010")]
    E3401010, // Cannot switch to PM mode (for copy trading master trader)
    #[serde(rename = "3400139")]
    E3400139, // The total value of your positions and orders has exceeded the risk limit for a Perpetual or Futures contract
    #[serde(rename = "500010")]
    E500010, // The sub-account specified does not belong to the parent account
    #[serde(rename = "500011")]
    E500011, // The Uid 592334 provided is not associated with a Unified Trading Account

    // Spot Trade
    #[serde(rename = "170001")]
    E170001, // Internal error.
    #[serde(rename = "170005")]
    E170005, // Too many new orders; current limit is %s orders per %s.
    #[serde(rename = "170007")]
    E170007, // Timeout waiting for response from backend server.
    #[serde(rename = "170010")]
    E170010, // Purchase failed: Exceed the maximum position limit of leveraged tokens, the current available limit is %s USDT
    #[serde(rename = "170011")]
    E170011, // Purchase failed: Exceed the maximum position limit of innovation tokens, the current available limit is ''{{.replaceKey0}}'' USDT
    #[serde(rename = "170019")]
    E170019, // Purchase failed: Exceed the maximum position limit of innovation tokens, the current available limit is ''{{.replaceKey0}}'' USDT
    #[serde(rename = "170031")]
    E170031, // The feature has been suspended
    #[serde(rename = "170032")]
    E170032, // Network error. Please try again later
    #[serde(rename = "170033")]
    E170033, // margin Insufficient account balance
    #[serde(rename = "170034")]
    E170034, // Liability over flow in spot leverage trade!
    #[serde(rename = "170035")]
    E170035, // Submitted to the system for processing!
    #[serde(rename = "170036")]
    E170036, // You haven't enabled Cross Margin Trading yet. To do so, please head to the PC trading site or the Bybit app
    #[serde(rename = "170037")]
    E170037, // Cross Margin Trading not yet supported by the selected coin
    #[serde(rename = "170105")]
    E170105, // Parameter '%s' was empty.
    #[serde(rename = "170115")]
    E170115, // Invalid timeInForce.
    #[serde(rename = "170116")]
    E170116, // Invalid orderType.
    #[serde(rename = "170117")]
    E170117, // Invalid side.
    #[serde(rename = "170121")]
    E170121, // Invalid symbol.
    #[serde(rename = "170124")]
    E170124, // Order amount too large.
    #[serde(rename = "170130")]
    E170130, // Data sent for paramter '%s' is not valid.
    #[serde(rename = "170131")]
    E170131, // Balance insufficient
    #[serde(rename = "170132")]
    E170132, // Order price too high.
    #[serde(rename = "170133")]
    E170133, // Order price lower than the minimum.
    #[serde(rename = "170134")]
    E170134, // Order price decimal too long.
    #[serde(rename = "170135")]
    E170135, // Order quantity too large.
    #[serde(rename = "170136")]
    E170136, // Order quantity lower than the minimum.
    #[serde(rename = "170137")]
    E170137, // Order volume decimal too long
    #[serde(rename = "170139")]
    E170139, // Order has been filled.
    #[serde(rename = "170140")]
    E170140, // Transaction amount lower than the minimum.
    #[serde(rename = "170141")]
    E170141, // Duplicate clientOrderId
    #[serde(rename = "170142")]
    E170142, // Order has been canceled
    #[serde(rename = "170143")]
    E170143, // Cannot be found on order book
    #[serde(rename = "170144")]
    E170144, // Order has been locked
    #[serde(rename = "170145")]
    E170145, // This order type does not support cancellation
    #[serde(rename = "170146")]
    E170146, // Order creation timeout
    #[serde(rename = "170147")]
    E170147, // Order cancellation timeout
    #[serde(rename = "170148")]
    E170148, // Market order amount decimal too long
    #[serde(rename = "170149")]
    E170149, // Create order failed
    #[serde(rename = "170150")]
    E170150, // Cancel order failed
    #[serde(rename = "170151")]
    E170151, // The trading pair is not open yet
    #[serde(rename = "170157")]
    E170157, // The trading pair is not available for api trading
    #[serde(rename = "170159")]
    E170159, // Market Order is not supported within the first %s minutes of newly launched pairs due to risk control.
    #[serde(rename = "170190")]
    E170190, // Cancel order has been finished
    #[serde(rename = "170191")]
    E170191, // Can not cancel order, please try again later
    #[serde(rename = "170192")]
    E170192, // Order price cannot be higher than %s .
    #[serde(rename = "170193")]
    E170193, // Buy order price cannot be higher than %s.
    #[serde(rename = "170194")]
    E170194, // Sell order price cannot be lower than %s.
    #[serde(rename = "170195")]
    E170195, // Please note that your order may not be filled. ETP buy order price deviates from risk control
    #[serde(rename = "170196")]
    E170196, // Please note that your order may not be filled. ETP sell order price deviates from risk control
    #[serde(rename = "170197")]
    E170197, // Your order quantity to buy is too large. The filled price may deviate significantly from the market price. Please try again
    #[serde(rename = "170198")]
    E170198, // Your order quantity to sell is too large. The filled price may deviate significantly from the market price. Please try again
    #[serde(rename = "170199")]
    E170199, // Your order quantity to buy is too large. The filled price may deviate significantly from the nav. Please try again.
    #[serde(rename = "170200")]
    E170200, // Your order quantity to sell is too large. The filled price may deviate significantly from the nav. Please try again.
    #[serde(rename = "170201")]
    E170201, // Invalid orderFilter parameter
    #[serde(rename = "170202")]
    E170202, // Please enter the TP/SL price.
    #[serde(rename = "170203")]
    E170203, // trigger price cannot be higher than 110% price.
    #[serde(rename = "170204")]
    E170204, // trigger price cannot be lower than 90% of qty.
    #[serde(rename = "170206")]
    E170206, // Stop_limit Order is not supported within the first 5 minutes of newly launched pairs
    #[serde(rename = "170207")]
    E170207, // The loan amount of the platform is not enough.
    #[serde(rename = "170210")]
    E170210, // New order rejected.
    #[serde(rename = "170212")]
    E170212, // Cancel order request processing
    #[serde(rename = "170213")]
    E170213, // Order does not exist.
    #[serde(rename = "170215")]
    E170215, // Spot Trading (Buy) Restriction: The current LTV ratio of your institutional lending has reached the maximum allowable amount for buy orders
    #[serde(rename = "170216")]
    E170216, // The leverage you select for Spot Trading cannot exceed the maximum leverage allowed by Institutional Lending
    #[serde(rename = "170217")]
    E170217, // Only LIMIT-MAKER order is supported for the current pair.
    #[serde(rename = "170218")]
    E170218, // The LIMIT-MAKER order is rejected due to invalid price.
    #[serde(rename = "170219")]
    E170219, // UID {{xxx}} is not available to this feature
    #[serde(rename = "170220")]
    E170220, // Spot Trading Restriction: The current LTV ratio of your institutional lending has reached the maximum allowable amount for Spot trading
    #[serde(rename = "170221")]
    E170221, // This coin does not exist.
    #[serde(rename = "170222")]
    E170222, // Too many requests in this time frame.
    #[serde(rename = "170223")]
    E170223, // Your Spot Account with Institutional Lending triggers an alert or liquidation.
    #[serde(rename = "170224")]
    E170224, // You're not a user of the Innovation Zone.
    #[serde(rename = "170226")]
    E170226, // Your Spot Account for Margin Trading is being liquidated.
    #[serde(rename = "170227")]
    E170227, // This feature is not supported.
    #[serde(rename = "170228")]
    E170228, // The purchase amount of each order exceeds the estimated maximum purchase amount.
    #[serde(rename = "170229")]
    E170229, // The sell quantity per order exceeds the estimated maximum sell quantity.
    #[serde(rename = "170230")]
    E170230, // Operations Restriction: Due to the deactivation of Margin Trading for institutional loan
    #[serde(rename = "170234")]
    E170234, // System Error
    #[serde(rename = "170241")]
    E170241, // To proceed with trading, users must read through and confirm that they fully understand the project's risk disclosure document. For App users, please update your Bybit App to version 4.16.0 to process.
    #[serde(rename = "170310")]
    E170310, // Order modification timeout
    #[serde(rename = "170311")]
    E170311, // Order modification failed
    #[serde(rename = "170312")]
    E170312, // The current order does not support modification
    #[serde(rename = "170313")]
    E170313, // The modified contract quantity cannot be less than to the filled quantity
    #[serde(rename = "170341")]
    E170341, // Request order quantity exceeds maximum limit
    #[serde(rename = "170344")]
    E170344, // symbol loanable limit
    #[serde(rename = "170709")]
    E170709, // OTC loan: The select trading pair is not in the whitelist pair

    // Spot Leverage Token
    #[serde(rename = "175000")]
    E175000, // The serialNum is already in use.
    #[serde(rename = "175001")]
    E175001, // Daily purchase limit has been exceeded. Please try again later.
    #[serde(rename = "175002")]
    E175002, // There's a large number of purchase orders. Please try again later.
    #[serde(rename = "175003")]
    E175003, // Insufficient available balance. Please make a deposit and try again.
    #[serde(rename = "175004")]
    E175004, // Daily redemption limit has been exceeded. Please try again later.
    #[serde(rename = "175005")]
    E175005, // There's a large number of redemption orders. Please try again later.
    #[serde(rename = "175006")]
    E175006, // Insufficient available balance. Please make a deposit and try again.
    #[serde(rename = "175007")]
    E175007, // Order not found.
    #[serde(rename = "175008")]
    E175008, // Purchase period hasn't started yet.
    #[serde(rename = "175009")]
    E175009, // Purchase amount has exceeded the upper limit.
    #[serde(rename = "175010")]
    E175010, // You haven't passed the quiz yet! To purchase and/or redeem an LT, please complete the quiz first.
    #[serde(rename = "175012")]
    E175012, // Redemption period hasn't started yet.
    #[serde(rename = "175013")]
    E175013, // Redemption amount has exceeded the upper limit.
    #[serde(rename = "175014")]
    E175014, // Purchase of the LT has been temporarily suspended.
    #[serde(rename = "175015")]
    E175015, // Redemption of the LT has been temporarily suspended.
    #[serde(rename = "175016")]
    E175016, // Invalid format. Please check the length and numeric precision.
    #[serde(rename = "175017")]
    E175017, // Failed to place order：Exceed the maximum position limit of leveraged tokens, the current available limit is %s USDT
    #[serde(rename = "175027")]
    E175027, // Subscriptions and redemptions are temporarily unavailable while account upgrade is in progress

    // Spot Margin Trade
    #[serde(rename = "176002")]
    E176002, // Query user account info error. Confirm that if you have completed quiz in GUI
    #[serde(rename = "176003")]
    E176003, // Query user loan history error
    #[serde(rename = "176004")]
    E176004, // Query order history start time exceeds end time
    #[serde(rename = "176005")]
    E176005, // Failed to borrow
    #[serde(rename = "176006")]
    E176006, // Repayment Failed
    #[serde(rename = "176007")]
    E176007, // User not found
    #[serde(rename = "176008")]
    E176008, // You haven't enabled Cross Margin Trading yet. To do so, please head to the PC trading site
    #[serde(rename = "176009")]
    E176009, // You haven't enabled Cross Margin Trading yet. Confirm that if you have turned on margin trade
    #[serde(rename = "176010")]
    E176010, // Failed to locate the coins to borrow
    #[serde(rename = "176011")]
    E176011, // Cross Margin Trading not yet supported by the selected coin
    #[serde(rename = "176012")]
    E176012, // Pair not available
    #[serde(rename = "176013")]
    E176013, // Cross Margin Trading not yet supported by the selected pair
    #[serde(rename = "176014")]
    E176014, // Repeated repayment requests
    #[serde(rename = "176015")]
    E176015, // Insufficient available balance
    #[serde(rename = "176016")]
    E176016, // No repayment required
    #[serde(rename = "176017")]
    E176017, // Repayment amount has exceeded the total liability
    #[serde(rename = "176018")]
    E176018, // Settlement in progress
    #[serde(rename = "176019")]
    E176019, // Liquidation in progress
    #[serde(rename = "176020")]
    E176020, // Failed to locate repayment history
    #[serde(rename = "176021")]
    E176021, // Repeated borrowing requests
    #[serde(rename = "176022")]
    E176022, // Coins to borrow not generally available yet
    #[serde(rename = "176023")]
    E176023, // Pair to borrow not generally available yet
    #[serde(rename = "176024")]
    E176024, // Invalid user status
    #[serde(rename = "176025")]
    E176025, // Amount to borrow cannot be lower than the min. amount to borrow (per transaction)
    #[serde(rename = "176026")]
    E176026, // Amount to borrow cannot be larger than the max. amount to borrow (per transaction)
    #[serde(rename = "176027")]
    E176027, // Amount to borrow cannot be higher than the max. amount to borrow per user
    #[serde(rename = "176028")]
    E176028, // Amount to borrow has exceeded Bybit's max. amount to borrow
    #[serde(rename = "176029")]
    E176029, // Amount to borrow has exceeded the user's estimated max. amount to borrow
    #[serde(rename = "176030")]
    E176030, // Query user loan info error
    #[serde(rename = "176031")]
    E176031, // Number of decimals for borrow amount has exceeded the maximum precision
    #[serde(rename = "176034")]
    E176034, // The leverage ratio is out of range
    #[serde(rename = "176035")]
    E176035, // Failed to close the leverage switch during liquidation
    #[serde(rename = "176036")]
    E176036, // Failed to adjust leverage switch during forced liquidation
    #[serde(rename = "176037")]
    E176037, // For non-unified transaction users, the operation failed
    #[serde(rename = "176038")]
    E176038, // The spot leverage is closed and the current operation is not allowed
    #[serde(rename = "176039")]
    E176039, // Borrowing, current operation is not allowed
    #[serde(rename = "176040")]
    E176040, // There is a spot leverage order, and the adjustment of the leverage switch failed!
    #[serde(rename = "176132")]
    E176132, // Number of decimals for repay amount has exceeded the maximum precision
    #[serde(rename = "176133")]
    E176133, // Liquidation may be triggered! Please adjust your transaction amount and try again
    #[serde(rename = "176134")]
    E176134, // Account has been upgraded (upgrading) to UTA
    #[serde(rename = "176135")]
    E176135, // Failed to get bond data
    #[serde(rename = "176136")]
    E176136, // Failed to get borrow data
    #[serde(rename = "176137")]
    E176137, // Failed to switch user status
    #[serde(rename = "176138")]
    E176138, // You need to repay all your debts before closing your disabling cross margin account
    #[serde(rename = "176139")]
    E176139, // Sorry, you are not eligible to enable cross margin, as you have already enabled OTC lending
    #[serde(rename = "176201")]
    E176201, // Account exception. Check if the UID is bound to an institutional loan
    #[serde(rename = "182104")]
    E182104, // This action could not be completed as your Unified Margin Account's IM/MM utilization rate has exceeded the threshold
    #[serde(rename = "182105")]
    E182105, // Adjustment failed, user is upgrading
    #[serde(rename = "182106")]
    E182106, // Adjustment failed, user forced liquidation in progress.
    #[serde(rename = "182107")]
    E182107, // Adjustment failed, Maintenance Margin Rate too high

    // Asset
    #[serde(rename = "131001")]
    E131001, // OpenAPI service error
    #[serde(rename = "131002")]
    E131002, // Parameter error | Withdraw address chain or destination tag are not equal
    #[serde(rename = "131003")]
    E131003, // Internal error
    #[serde(rename = "131004")]
    E131004, // KYC needed
    #[serde(rename = "131075")]
    E131075, // InternalAddressCannotBeYourself
    #[serde(rename = "131076")]
    E131076, // Internal transfer not support sub-accounts
    #[serde(rename = "131077")]
    E131077, // Receive user not exist
    #[serde(rename = "131078")]
    E131078, // Receive user deposit has been banned
    #[serde(rename = "131079")]
    E131079, // Receive user need KYC
    #[serde(rename = "131080")]
    E131080, // User left retry times is zero
    #[serde(rename = "131081")]
    E131081, // Do not input memo/tag, please.
    #[serde(rename = "131082")]
    E131082, // Do not repeat the request
    #[serde(rename = "131083")]
    E131083, // Withdraw only allowed from address book
    #[serde(rename = "131084")]
    E131084, // Withdraw failed because of Uta Upgrading
    #[serde(rename = "131085")]
    E131085, // Withdrawal amount is greater than your available balance (the delayed withdrawal is triggered)
    #[serde(rename = "131086")]
    E131086, // Withdrawal amount exceeds risk limit (the risk limit of margin trade is triggered)
    #[serde(rename = "131087")]
    E131087, // Your current account spot risk level is too high, withdrawal is prohibited, please adjust and try again
    #[serde(rename = "131088")]
    E131088, // The withdrawal amount exceeds the remaining withdrawal limit of your identity verification level. The current available amount for withdrawal : %s
    #[serde(rename = "131089")]
    E131089, // User sensitive operation, withdrawal is prohibited within 24 hours
    #[serde(rename = "131090")]
    E131090, // User withdraw has been banned
    #[serde(rename = "131091")]
    E131091, // Blocked login status does not allow withdrawals
    #[serde(rename = "131092")]
    E131092, // User status is abnormal
    #[serde(rename = "131093")]
    E131093, // The withdrawal address is not in the whitelist
    #[serde(rename = "131094")]
    E131094, // UserId is not in the whitelist
    #[serde(rename = "131095")]
    E131095, // Withdrawal amount exceeds the 24-hour platform limit
    #[serde(rename = "131096")]
    E131096, // Withdraw amount does not satisfy the lower limit or upper limit
    #[serde(rename = "131097")]
    E131097, // Withdrawal of this currency has been closed
    #[serde(rename = "131098")]
    E131098, // Withdrawal currently is not available from new address
    #[serde(rename = "90040")]
    E90040, // Please wait at least 10 seconds between withdrawals. You need to wait for another 10 sec when operate the same coin and chain
    #[serde(rename = "131099")]
    E131099, // Hot wallet status can cancel the withdraw
    #[serde(rename = "131200")]
    E131200, // Service error
    #[serde(rename = "131201")]
    E131201, // Internal error
    #[serde(rename = "131202")]
    E131202, // Invalid memberId
    #[serde(rename = "131203")]
    E131203, // Request parameter error
    #[serde(rename = "131204")]
    E131204, // Account info error
    #[serde(rename = "131205")]
    E131205, // Query transfer error
    #[serde(rename = "131206")]
    E131206, // Cannot be transfer
    #[serde(rename = "131207")]
    E131207, // Account not exist
    #[serde(rename = "131208")]
    E131208, // Forbid transfer
    #[serde(rename = "131209")]
    E131209, // Get subMember relation error
    #[serde(rename = "131210")]
    E131210, // Amount accuracy error
    #[serde(rename = "131211")]
    E131211, // fromAccountType can't be the same as toAccountType
    #[serde(rename = "131212")]
    E131212, // Insufficient balance
    #[serde(rename = "131213")]
    E131213, // TransferLTV check error
    #[serde(rename = "131214")]
    E131214, // TransferId exist
    #[serde(rename = "131215")]
    E131215, // Amount error
    #[serde(rename = "131216")]
    E131216, // Query balance error
    #[serde(rename = "131217")]
    E131217, // Risk check error
    #[serde(rename = "131227")]
    E131227, // Sub-account do not have universal transfer permission
    #[serde(rename = "131228")]
    E131228, // Your balance is not enough. Please check transfer safe amount
    #[serde(rename = "131229")]
    E131229, // Due to compliance requirements, the current currency is not allowed to transfer
    #[serde(rename = "131230")]
    E131230, // The system is busy, please try again later
    #[serde(rename = "131231")]
    E131231, // Transfers into this account are not supported
    #[serde(rename = "131232")]
    E131232, // Transfers out this account are not supported
    #[serde(rename = "140001")]
    E140001, // Switching the PM spot hedging switch is not allowed in non-PM mode
    #[serde(rename = "140002")]
    E140002, // Institutional lending users do not support PM spot hedging
    #[serde(rename = "140003")]
    E140003, // You have position(s) being liquidated, please try again later
    #[serde(rename = "140004")]
    E140004, // Operations Restriction: The current LTV ratio of your Institutional Loan has hit the liquidation threshold. Assets in your account are being liquidated
    #[serde(rename = "140005")]
    E140005, // Risk level after switching modes exceeds threshold
    #[serde(rename = "141004")]
    E141004, // Sub member is not normal
    #[serde(rename = "141025")]
    E141025, // This sub-account has assets and cannot be deleted
    #[serde(rename = "181000")]
    E181000, // Category is null
    #[serde(rename = "181001")]
    E181001, // Category only support linear or option or spot.
    #[serde(rename = "181002")]
    E181002, // Symbol is null.
    #[serde(rename = "181003")]
    E181003, // Side is null.
    #[serde(rename = "181004")]
    E181004, // Side only support Buy or Sell.
    #[serde(rename = "181005")]
    E181005, // OrderStatus is wrong
    #[serde(rename = "181006")]
    E181006, // StartTime is not number
    #[serde(rename = "181007")]
    E181007, // EndTime is not number
    #[serde(rename = "181008")]
    E181008, // Parameter StartTime and EndTime are both needed
    #[serde(rename = "181009")]
    E181009, // Parameter StartTime needs to be smaller than EndTime
    #[serde(rename = "181010")]
    E181010, // The time range between StartTime and EndTime cannot exceed 7 days
    #[serde(rename = "181011")]
    E181011, // Limit is not a number
    #[serde(rename = "181012")]
    E181012, // Symbol not exist
    #[serde(rename = "181013")]
    E181013, // Only support settleCoin: usdc
    #[serde(rename = "181014")]
    E181014, // Classic account is not supported
    #[serde(rename = "181018")]
    E181018, // Invalid expDate.
    #[serde(rename = "181019")]
    E181019, // Parameter expDate can't be earlier than 2 years
    #[serde(rename = "182000")]
    E182000, // Symbol related quote price is null

    // Institutional Loan
    #[serde(rename = "3777002")]
    E3777002, // UID cannot be bound repeatedly.
    #[serde(rename = "3777003")]
    E3777003, // UID cannot be unbound because the UID has not been bound to a risk unit.
    #[serde(rename = "3777004")]
    E3777004, // The main UID of the risk unit cannot be unbound.
    #[serde(rename = "3777006")]
    E3777006, // UID cannot be bound, please try again with a different UID.
    #[serde(rename = "3777007")]
    E3777007, // UID cannot be bound, please upgrade to UTA Pro.
    #[serde(rename = "3777027")]
    E3777027, // UID cannot be bound, leveraged trading closure failed.
    #[serde(rename = "3777029")]
    E3777029, // You currently have orders for pre-market trading that can’t be bind UIDs

    // Broker
    #[serde(rename = "3500402")]
    E3500402, // Parameter verification failed for 'limit'.
    #[serde(rename = "3500403")]
    E3500403, // Only available to exchange broker main-account
    #[serde(rename = "3500404")]
    E3500404, // Invalid Cursor
    #[serde(rename = "3500405")]
    E3500405, // Parameter "startTime" and "endTime" need to be input in pairs.
    #[serde(rename = "3500406")]
    E3500406, // Out of query time range.
    #[serde(rename = "3500407")]
    E3500407, // Parameter begin and end need to be input in pairs.
}

impl Display for ErrorCodes {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            // HTTP error codes
            ErrorCodes::E400 => write!(f, "400 - Bad request. Need to send the request with GET / POST (must be capitalized)"),
            ErrorCodes::E401 => write!(f, "401 - Invalid request. 1. Need to use the correct key to access; 2. Need to put authentication params in the request header"),
            ErrorCodes::E403 => write!(f, "403 - Forbidden request. Possible causes: 1. IP rate limit breached; 2. You send GET request with an empty json body; 3. You are using U.S IP"),
            ErrorCodes::E404 => write!(f, "404 - Cannot find path. Possible causes: 1. Wrong path; 2. Category value does not match account mode"),

            // UTA & Classic Account error codes
            ErrorCodes::E0 => write!(f, "0 - Ok"),
            ErrorCodes::E10000 => write!(f, "10000 - Server Timeout"),
            ErrorCodes::E10001 => write!(f, "10001 - Request parameter error"),
            ErrorCodes::E10002 => write!(f, "10002 - The request time exceeds the time window range."),
            ErrorCodes::E10003 => write!(f, "10003 - API key is invalid. Check whether the key and domain are matched, there are 4 env: mainnet, testnet, mainnet-demo, testnet-demo"),
            ErrorCodes::E33004 => write!(f, "33004 - Your api key has expired"),
            ErrorCodes::E10004 => write!(f, "10004 - Error sign, please check your signature generation algorithm."),
            ErrorCodes::E10005 => write!(f, "10005 - Permission denied, please check your API key permissions."),
            ErrorCodes::E10006 => write!(f, "10006 - Too many visits. Exceeded the API Rate Limit."),
            ErrorCodes::E10007 => write!(f, "10007 - User authentication failed."),
            ErrorCodes::E10008 => write!(f, "10008 - Common banned, please check your account mode"),
            ErrorCodes::E10009 => write!(f, "10009 - IP has been banned."),
            ErrorCodes::E10010 => write!(f, "10010 - Unmatched IP, please check your API key's bound IP addresses."),
            ErrorCodes::E10014 => write!(f, "10014 - Invalid duplicate request."),
            ErrorCodes::E10016 => write!(f, "10016 - Server error."),
            ErrorCodes::E10017 => write!(f, "10017 - Route not found."),
            ErrorCodes::E10018 => write!(f, "10018 - Exceeded the IP Rate Limit."),
            ErrorCodes::E10024 => write!(f, "10024 - Compliance rules triggered"),
            ErrorCodes::E10027 => write!(f, "10027 - Transactions are banned."),
            ErrorCodes::E10028 => write!(f, "10028 - The API can only be accessed by unified account users."),
            ErrorCodes::E10029 => write!(f, "10029 - The requested symbol is invalid, please check symbol whitelist"),
            ErrorCodes::E30133 => write!(f, "30133 - OTC loan: The symbol you select for USDT Perpetual is not allowed by Institutional Lending"),
            ErrorCodes::E30134 => write!(f, "30134 - OTC loan: The symbol you select for USDC Contract is not allowed by Institutional Lending"),
            ErrorCodes::E30135 => write!(f, "30135 - The leverage you select for USDT Perpetual trading cannot exceed the maximum leverage allowed by Institutional Lending."),
            ErrorCodes::E30136 => write!(f, "30136 - The leverage you select for USDC Perpetual or Futures trading cannot exceed the maximum leverage allowed by Institutional Lending."),
            ErrorCodes::E40004 => write!(f, "40004 - The order is modified during the process of replacing, please check the order status again"),
            ErrorCodes::E110001 => write!(f, "110001 - Order does not exist"),
            ErrorCodes::E110003 => write!(f, "110003 - Order price exceeds the allowable range."),
            ErrorCodes::E110004 => write!(f, "110004 - Wallet balance is insufficient"),
            ErrorCodes::E110005 => write!(f, "110005 - Position status"),
            ErrorCodes::E110006 => write!(f, "110006 - The assets are estimated to be unable to cover the position margin"),
            ErrorCodes::E110007 => write!(f, "110007 - Available balance is insufficient"),
            ErrorCodes::E110008 => write!(f, "110008 - The order has been completed or cancelled."),
            ErrorCodes::E110009 => write!(f, "110009 - The number of stop orders exceeds the maximum allowable limit. You can find references in our API doc."),
            ErrorCodes::E110010 => write!(f, "110010 - The order has been cancelled"),
            ErrorCodes::E110011 => write!(f, "110011 - Liquidation will be triggered immediately by this adjustment"),
            ErrorCodes::E110012 => write!(f, "110012 - Insufficient available balance."),
            ErrorCodes::E110013 => write!(f, "110013 - Cannot set leverage due to risk limit level."),
            ErrorCodes::E110014 => write!(f, "110014 - Insufficient available balance to add additional margin."),
            ErrorCodes::E110015 => write!(f, "110015 - The position is in cross margin mode."),
            ErrorCodes::E110016 => write!(f, "110016 - The quantity of contracts requested exceeds the risk limit, please adjust your risk limit level before trying again"),
            ErrorCodes::E110017 => write!(f, "110017 - Reduce-only rule not satisfied"),
            ErrorCodes::E110018 => write!(f, "110018 - User ID is illegal."),
            ErrorCodes::E110019 => write!(f, "110019 - Order ID is illegal."),
            ErrorCodes::E110020 => write!(f, "110020 - Not allowed to have more than 500 active orders."),
            ErrorCodes::E110021 => write!(f, "110021 - Not allowed to exceeded position limits due to Open Interest."),
            ErrorCodes::E110022 => write!(f, "110022 - Quantity has been restricted and orders cannot be modified to increase the quantity."),
            ErrorCodes::E110023 => write!(f, "110023 - Currently you can only reduce your position on this contract. please check our announcement or contact customer service for details."),
            ErrorCodes::E110024 => write!(f, "110024 - You have an existing position, so the position mode cannot be switched."),
            ErrorCodes::E110025 => write!(f, "110025 - Position mode has not been modified."),
            ErrorCodes::E110026 => write!(f, "110026 - Cross/isolated margin mode has not been modified."),
            ErrorCodes::E110027 => write!(f, "110027 - Margin has not been modified."),
            ErrorCodes::E110028 => write!(f, "110028 - You have existing open orders, so the position mode cannot be switched."),
            ErrorCodes::E110029 => write!(f, "110029 - Hedge mode is not supported for this symbol."),
            ErrorCodes::E110030 => write!(f, "110030 - Duplicate orderId"),
            ErrorCodes::E110031 => write!(f, "110031 - Non-existing risk limit info, please check the risk limit rules."),
            ErrorCodes::E110032 => write!(f, "110032 - Order is illegal"),
            ErrorCodes::E110033 => write!(f, "110033 - You can't set margin without an open position"),
            ErrorCodes::E110034 => write!(f, "110034 - There is no net position"),
            ErrorCodes::E110035 => write!(f, "110035 - Cancellation of orders was not completed before liquidation"),
            ErrorCodes::E110036 => write!(f, "110036 - You are not allowed to change leverage due to cross margin mode."),
            ErrorCodes::E110037 => write!(f, "110037 - User setting list does not have this symbol"),
            ErrorCodes::E110038 => write!(f, "110038 - You are not allowed to change leverage due to portfolio margin mode."),
            ErrorCodes::E110039 => write!(f, "110039 - Maintenance margin rate is too high. This may trigger liquidation."),
            ErrorCodes::E110040 => write!(f, "110040 - The order will trigger a forced liquidation, please re-submit the order."),
            ErrorCodes::E110041 => write!(f, "110041 - Skip liquidation is not allowed when a position or maker order exists"),
            ErrorCodes::E110042 => write!(f, "110042 - Currently,due to pre-delivery status, you can only reduce your position on this contract."),
            ErrorCodes::E110043 => write!(f, "110043 - Set leverage has not been modified."),
            ErrorCodes::E110044 => write!(f, "110044 - Available margin is insufficient."),
            ErrorCodes::E110045 => write!(f, "110045 - Wallet balance is insufficient."),
            ErrorCodes::E110046 => write!(f, "110046 - Liquidation will be triggered immediately by this adjustment."),
            ErrorCodes::E110047 => write!(f, "110047 - Risk limit cannot be adjusted due to insufficient available margin."),
            ErrorCodes::E110048 => write!(f, "110048 - Risk limit cannot be adjusted as the current/expected position value exceeds the revised risk limit."),
            ErrorCodes::E110049 => write!(f, "110049 - Tick notes can only be numbers"),
            ErrorCodes::E110050 => write!(f, "110050 - Invalid coin"),
            ErrorCodes::E110051 => write!(f, "110051 - The user's available balance cannot cover the lowest price of the current market"),
            ErrorCodes::E110052 => write!(f, "110052 - Your available balance is insufficient to set the price"),
            ErrorCodes::E110053 => write!(f, "110053 - The user's available balance cannot cover the current market price and upper limit price"),
            ErrorCodes::E110054 => write!(f, "110054 - This position has at least one take profit link order, so the take profit and stop loss mode cannot be switched"),
            ErrorCodes::E110055 => write!(f, "110055 - This position has at least one stop loss link order, so the take profit and stop loss mode cannot be switched"),
            ErrorCodes::E110056 => write!(f, "110056 - This position has at least one trailing stop link order, so the take profit and stop loss mode cannot be switched"),
            ErrorCodes::E110057 => write!(f, "110057 - Conditional order or limit order contains TP/SL related params"),
            ErrorCodes::E110058 => write!(f, "110058 - You can't set take profit and stop loss due to insufficient size of remaining position size."),
            ErrorCodes::E110059 => write!(f, "110059 - Not allowed to modify the TP/SL of a partially filled open order"),
            ErrorCodes::E110060 => write!(f, "110060 - Under full TP/SL mode, it is not allowed to modify TP/SL"),
            ErrorCodes::E110061 => write!(f, "110061 - Not allowed to have more than 20 TP/SLs under Partial tpSlMode"),
            ErrorCodes::E110062 => write!(f, "110062 - There is no MMP information of the institution found."),
            ErrorCodes::E110063 => write!(f, "110063 - Settlement in progress! {{key0}} not available for trading."),
            ErrorCodes::E110064 => write!(f, "110064 - The modified contract quantity cannot be less than or equal to the filled quantity."),
            ErrorCodes::E110065 => write!(f, "110065 - MMP hasn't yet been enabled for your account. Please contact your BD manager."),
            ErrorCodes::E110066 => write!(f, "110066 - Trading is currently not allowed."),
            ErrorCodes::E110067 => write!(f, "110067 - Unified account is not supported."),
            ErrorCodes::E110068 => write!(f, "110068 - Leveraged trading is not allowed."),
            ErrorCodes::E110069 => write!(f, "110069 - Ins lending customer is not allowed to trade."),
            ErrorCodes::E110070 => write!(f, "110070 - ETP symbols cannot be traded."),
            ErrorCodes::E110071 => write!(f, "110071 - Sorry, we're revamping the Unified Margin Account! Currently, new upgrades are not supported. If you have any questions, please contact our 24/7 customer support."),
            ErrorCodes::E110072 => write!(f, "110072 - OrderLinkedID is duplicate"),
            ErrorCodes::E110073 => write!(f, "110073 - Set margin mode failed"),
            ErrorCodes::E110075 => write!(f, "110075 - RiskId not modified"),
            ErrorCodes::E182021 => write!(f, "182021 - Cannot enable spot margin while in isolated margin mode. Please switch to cross margin mode or portfolio margin mode to trade spot with margin."),
            ErrorCodes::E110076 => write!(f, "110076 - Only isolated mode can set auto-add-margin"),
            ErrorCodes::E110077 => write!(f, "110077 - Pm mode cannot support"),
            ErrorCodes::E110078 => write!(f, "110078 - Added margin more than max can reduce margin"),
            ErrorCodes::E110079 => write!(f, "110079 - The order is processing and can not be operated, please try again later"),
            ErrorCodes::E110080 => write!(f, "110080 - Operations Restriction: The current LTV ratio of your Institutional Lending has hit the liquidation threshold. Assets in your account are being liquidated (trade/risk limit/leverage)"),
            ErrorCodes::E110082 => write!(f, "110082 - You cannot lift Reduce-Only restrictions, as no Reduce-Only restrictions are applied to your position"),
            ErrorCodes::E110083 => write!(f, "110083 - Reduce-Only restrictions must be lifted for both Long and Short positions at the same time"),
            ErrorCodes::E110085 => write!(f, "110085 - The risk limit and margin ratio for this contract has been updated, please select a supported risk limit and place your order again"),
            ErrorCodes::E110086 => write!(f, "110086 - Current order leverage exceeds the maximum available for your current Risk Limit tier. Please lower leverage before placing an order"),
            ErrorCodes::E110087 => write!(f, "110087 - Leverage for Perpetual or Futures contracts cannot exceed the maximum allowed for your Institutional loan"),
            ErrorCodes::E110088 => write!(f, "110088 - Please Upgrade to UTA to trade"),
            ErrorCodes::E110089 => write!(f, "110089 - Exceeds the maximum risk limit level"),
            ErrorCodes::E110090 => write!(f, "110090 - Exceeds the maximum leverage limit of the current risk"),
            ErrorCodes::E110092 => write!(f, "110092 - expect Rising, but trigger_price[XXXXX] <= current[XXXXX]??laste"),
            ErrorCodes::E110093 => write!(f, "110093 - expect Falling, but trigger_price[XXXXX] >= current[XXXXX]??last"),
            ErrorCodes::E110094 => write!(f, "110094 - Order notional value below the lower limit"),
            ErrorCodes::E181017 => write!(f, "181017 - OrderStatus must be final status"),
            ErrorCodes::E182100 => write!(f, "182100 - Compulsory closing of positions, no repayment allowed"),
            ErrorCodes::E182101 => write!(f, "182101 - Failed repayment, insufficient collateral balance"),
            ErrorCodes::E182102 => write!(f, "182102 - Failed repayment, there are no liabilities in the current currency"),
             
            ErrorCodes::E176013 => write!(f, "176013 - Cross Margin Trading not yet supported by the selected pair"),
            ErrorCodes::E176014 => write!(f, "176014 - Repeated repayment requests"),
            ErrorCodes::E176015 => write!(f, "176015 - Insufficient available balance"),
            ErrorCodes::E176016 => write!(f, "176016 - No repayment required"),
            ErrorCodes::E176017 => write!(f, "176017 - Repayment amount has exceeded the total liability"),
            ErrorCodes::E176018 => write!(f, "176018 - Settlement in progress"),
            ErrorCodes::E176019 => write!(f, "176019 - Liquidation in progress"),
            ErrorCodes::E176020 => write!(f, "176020 - Failed to locate repayment history"),
            ErrorCodes::E176021 => write!(f, "176021 - Repeated borrowing requests"),
            ErrorCodes::E176022 => write!(f, "176022 - Coins to borrow not generally available yet"),
            ErrorCodes::E176023 => write!(f, "176023 - Pair to borrow not generally available yet"),
            ErrorCodes::E176024 => write!(f, "176024 - Invalid user status"),
            ErrorCodes::E176025 => write!(f, "176025 - Amount to borrow cannot be lower than the min. amount to borrow (per transaction)"),
            ErrorCodes::E176026 => write!(f, "176026 - Amount to borrow cannot be lower than the min. amount to borrow (per user)"),
            ErrorCodes::E176027 => write!(f, "176027 - Amount to borrow has exceeded the max. amount to borrow"),
            ErrorCodes::E176028 => write!(f, "176028 - Amount to borrow has exceeded Bybit's max. amount to borrow"),
            ErrorCodes::E176029 => write!(f, "176029 - Amount to borrow has exceeded the max. amount to borrow (per user)"),
            ErrorCodes::E176030 => write!(f, "176030 - Amount to borrow has exceeded the max. amount to borrow (per transaction)"),
            ErrorCodes::E176031 => write!(f, "176031 - Amount to borrow has exceeded the max. amount to borrow (per user per coin)"),
            

            /*
           
    #[serde(rename = "110090")]
    E110090, // Exceeds the maximum leverage limit of the current risk limit level.
    #[serde(rename = "110092")]
    E110092, // expect Rising, but trigger_price[XXXXX] <= current[XXXXX]??laste
    #[serde(rename = "110093")]
    E110093, // expect Falling, but trigger_price[XXXXX] >= current[XXXXX]??last
    #[serde(rename = "110094")]
    E110094, // Order notional value below the lower limit
    #[serde(rename = "181017")]
    E181017, // OrderStatus must be final status
    #[serde(rename = "182100")]
    E182100, // Compulsory closing of positions, no repayment allowed
    #[serde(rename = "182101")]
    E182101, // Failed repayment, insufficient collateral balance
    #[serde(rename = "182102")]
    E182102, // Failed repayment, there are no liabilities in the current currency
    #[serde(rename = "182103")]
    E182103, // Institutional lending users are not supported
    #[serde(rename = "182108")]
    E182108, // Switching failed, margin verification failed, please re-adjust the currency status
    #[serde(rename = "182110")]
    E182110, // Failed to switch
    #[serde(rename = "182111")]
    E182111, // The requested currency has a non guaranteed gold currency or does not support switching status currencies
    #[serde(rename = "182112")]
    E182112, // Duplicate currency, please re-adjust
    #[serde(rename = "3100181")]
    E3100181, // UID can not be null
    #[serde(rename = "3100197")]
    E3100197, // Temporary banned due to the upgrade to UTA
    #[serde(rename = "3200316")]
    E3200316, // USDC Options Trading Restriction: The current LTV ratio for your Institutional Lending has reached the maximum allowable amount for USDC Options trading.
    #[serde(rename = "3200317")]
    E3200317, // USDC Options Open Position Restriction: The current LTV ratio for your Institutional Lending has reached the maximum allowable amount for opening USDC Options positions.
    #[serde(rename = "3100326")]
    E3100326, // BaseCoin is required
    #[serde(rename = "3200403")]
    E3200403, // isolated margin can not create order
    #[serde(rename = "3200320")]
    E3200320, // Operations Restriction: The current LTV ratio of your Institutional Lending has hit the liquidation threshold. Assets in your account are being liquidated. (margin mode or spot leverage)
    #[serde(rename = "3400208")]
    E3400208, // You have unclosed hedge mode or isolated mode USDT perpetual positions
    #[serde(rename = "3400209")]
    E3400209, // You have USDT perpetual positions, so upgrading is prohibited for 10 minutes before and after the hour every hour
    #[serde(rename = "3400210")]
    E3400210, // The risk rate of your Derivatives account is too high
    #[serde(rename = "3400211")]
    E3400211, // Once upgraded, the estimated risk rate will be too high
    #[serde(rename = "3400212")]
    E3400212, // You have USDC perpetual positions or Options positions, so upgrading is prohibited for 10 minutes before and after the hour every hour
    #[serde(rename = "3400213")]
    E3400213, // The risk rate of your USDC Derivatives account is too high
    #[serde(rename = "3400052")]
    E3400052, // You have uncancelled USDC perpetual orders
    #[serde(rename = "3400053")]
    E3400053, // You have uncancelled Options orders
    #[serde(rename = "3400054")]
    E3400054, // You have uncancelled USDT perpetual orders
    #[serde(rename = "3400214")]
    E3400214, // Server error, please try again later
    #[serde(rename = "3400071")]
    E3400071, // The net asset is not satisfied
    #[serde(rename = "3401010")]
    E3401010, // Cannot switch to PM mode (for copy trading master trader)
    #[serde(rename = "3400139")]
    E3400139, // The total value of your positions and orders has exceeded the risk limit for a Perpetual or Futures contract
    #[serde(rename = "500010")]
    E500010, // The sub-account specified does not belong to the parent account
    #[serde(rename = "500011")]
    E500011, // The Uid 592334 provided is not associated with a Unified Trading Account
             */

        }
    }
}

impl ErrorCodes {
    pub fn get_error_type(&self) -> ErrorTypes {
        match self {
            // HTTP error codes
            ErrorCodes::E400 => ErrorTypes::Http,
            ErrorCodes::E401 => ErrorTypes::Http,
            ErrorCodes::E403 => ErrorTypes::Http,
            ErrorCodes::E404 => ErrorTypes::Http,
            // UTA & Classic Account error codes
        }
    }
}
