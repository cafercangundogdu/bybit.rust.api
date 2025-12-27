use crate::rest::client::{RestClient, SecType, ServerResponse};
use crate::rest::enums::category::Category;
use crate::rest::enums::interval::Interval;
use crate::rest::enums::interval_time::IntervalTime;
use crate::rest::market::dto::{
    DeliveryPriceResult, FundingRateHistoryResult, HistoricalVolatilityResult,
    IndexPriceKlineResult, InsuranceResult, KlineResult, LongShortRatioResult,
    MarkPriceKlineResult, OpenInterestResult, OrderBookResult, PremiumIndexPriceKlineResult,
    RecentTradeResult, RiskLimitResult, ServerTimeResult, TickersResult,
};
use anyhow::Result;
use serde_json::json;

#[derive(Clone)]
pub struct MarketClient {
    client: RestClient,
}

impl MarketClient {
    pub fn new(client: RestClient) -> Self {
        MarketClient { client }
    }

    /// Get server time
    ///
    /// API: GET /v5/market/time
    /// https://bybit-exchange.github.io/docs/v5/market/time
    pub async fn get_server_time(&self) -> Result<ServerResponse<ServerTimeResult>> {
        let endpoint = "v5/market/time";
        let response: ServerResponse<ServerTimeResult> =
            self.client.get(endpoint, json!({}), SecType::None).await?;
        Ok(response)
    }

    /// Get kline data
    ///
    /// API: GET /v5/market/kline
    /// https://bybit-exchange.github.io/docs/v5/market/kline
    pub async fn get_kline(
        &self,
        category: Category,
        symbol: &str,
        interval: Interval,
        start: Option<i64>,
        end: Option<i64>,
        limit: Option<i32>,
    ) -> Result<ServerResponse<KlineResult>> {
        let endpoint = "v5/market/kline";
        let mut params = json!({
            "category": category,
            "symbol": symbol,
            "interval": interval,
        });

        if let Some(start) = start {
            params["start"] = json!(start);
        }
        if let Some(end) = end {
            params["end"] = json!(end);
        }
        if let Some(limit) = limit {
            params["limit"] = json!(limit);
        }

        let response: ServerResponse<KlineResult> =
            self.client.get(endpoint, params, SecType::None).await?;
        Ok(response)
    }

    /// Get mark price kline
    ///
    /// API: GET /v5/market/mark-price-kline
    /// https://bybit-exchange.github.io/docs/v5/market/mark-price-kline
    pub async fn get_mark_price_kline(
        &self,
        category: Category,
        symbol: &str,
        interval: Interval,
        start: Option<i64>,
        end: Option<i64>,
        limit: Option<i32>,
    ) -> Result<ServerResponse<MarkPriceKlineResult>> {
        let endpoint = "v5/market/mark-price-kline";
        let mut params = json!({
            "category": category,
            "symbol": symbol,
            "interval": interval,
        });

        if let Some(start) = start {
            params["start"] = json!(start);
        }
        if let Some(end) = end {
            params["end"] = json!(end);
        }
        if let Some(limit) = limit {
            params["limit"] = json!(limit);
        }

        let response: ServerResponse<MarkPriceKlineResult> =
            self.client.get(endpoint, params, SecType::None).await?;
        Ok(response)
    }

    /// Get index price kline
    ///
    /// API: GET /v5/market/index-price-kline
    /// https://bybit-exchange.github.io/docs/v5/market/index-price-kline
    pub async fn get_index_price_kline(
        &self,
        category: Category,
        symbol: &str,
        interval: Interval,
        start: Option<i64>,
        end: Option<i64>,
        limit: Option<i32>,
    ) -> Result<ServerResponse<IndexPriceKlineResult>> {
        let endpoint = "v5/market/index-price-kline";
        let mut params = json!({
            "category": category,
            "symbol": symbol,
            "interval": interval,
        });

        if let Some(start) = start {
            params["start"] = json!(start);
        }
        if let Some(end) = end {
            params["end"] = json!(end);
        }
        if let Some(limit) = limit {
            params["limit"] = json!(limit);
        }

        let response: ServerResponse<IndexPriceKlineResult> =
            self.client.get(endpoint, params, SecType::None).await?;
        Ok(response)
    }

    /// Get premium index price kline
    ///
    /// API: GET /v5/market/premium-index-price-kline
    /// https://bybit-exchange.github.io/docs/v5/market/preimum-index-price-kline
    pub async fn get_premium_index_price_kline(
        &self,
        category: Category,
        symbol: &str,
        interval: Interval,
        start: Option<i64>,
        end: Option<i64>,
        limit: Option<i32>,
    ) -> Result<ServerResponse<PremiumIndexPriceKlineResult>> {
        let endpoint = "v5/market/premium-index-price-kline";
        let mut params = json!({
            "category": category,
            "symbol": symbol,
            "interval": interval,
        });

        if let Some(start) = start {
            params["start"] = json!(start);
        }
        if let Some(end) = end {
            params["end"] = json!(end);
        }
        if let Some(limit) = limit {
            params["limit"] = json!(limit);
        }

        let response: ServerResponse<PremiumIndexPriceKlineResult> =
            self.client.get(endpoint, params, SecType::None).await?;
        Ok(response)
    }

    /// Get orderbook
    ///
    /// API: GET /v5/market/orderbook
    /// https://bybit-exchange.github.io/docs/v5/market/orderbook
    pub async fn get_orderbook(
        &self,
        category: Category,
        symbol: &str,
        limit: Option<i32>,
    ) -> Result<ServerResponse<OrderBookResult>> {
        let endpoint = "v5/market/orderbook";
        let mut params = json!({
            "category": category,
            "symbol": symbol,
        });

        if let Some(limit) = limit {
            params["limit"] = json!(limit);
        }

        let response: ServerResponse<OrderBookResult> =
            self.client.get(endpoint, params, SecType::None).await?;
        Ok(response)
    }

    /// Get instruments info
    ///
    /// API: GET /v5/market/instruments-info
    /// https://bybit-exchange.github.io/docs/v5/market/instrument
    pub async fn get_instruments_info(
        &self,
        category: Category,
        symbol: Option<&str>,
        base_coin: Option<&str>,
        limit: Option<i32>,
        cursor: Option<&str>,
    ) -> Result<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/market/instruments-info";
        let mut params = json!({
            "category": category,
        });

        if let Some(symbol) = symbol {
            params["symbol"] = json!(symbol);
        }
        if let Some(base_coin) = base_coin {
            params["baseCoin"] = json!(base_coin);
        }
        if let Some(limit) = limit {
            params["limit"] = json!(limit);
        }
        if let Some(cursor) = cursor {
            params["cursor"] = json!(cursor);
        }

        let response: ServerResponse<serde_json::Value> =
            self.client.get(endpoint, params, SecType::None).await?;
        Ok(response)
    }

    /// Get tickers
    ///
    /// API: GET /v5/market/tickers
    /// https://bybit-exchange.github.io/docs/v5/market/tickers
    pub async fn get_tickers(
        &self,
        category: Category,
        symbol: Option<&str>,
        base_coin: Option<&str>,
        exp_date: Option<&str>,
    ) -> Result<ServerResponse<TickersResult>> {
        let endpoint = "v5/market/tickers";
        let mut params = json!({
            "category": category,
        });

        if let Some(symbol) = symbol {
            params["symbol"] = json!(symbol);
        }
        if let Some(base_coin) = base_coin {
            params["baseCoin"] = json!(base_coin);
        }
        if let Some(exp_date) = exp_date {
            params["expDate"] = json!(exp_date);
        }

        let response: ServerResponse<TickersResult> =
            self.client.get(endpoint, params, SecType::None).await?;
        Ok(response)
    }

    /// Get funding rate history
    ///
    /// API: GET /v5/market/funding/history
    /// https://bybit-exchange.github.io/docs/v5/market/funding-rate
    pub async fn get_funding_history(
        &self,
        category: Category,
        symbol: &str,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: Option<i32>,
    ) -> Result<ServerResponse<FundingRateHistoryResult>> {
        let endpoint = "v5/market/funding/history";
        let mut params = json!({
            "category": category,
            "symbol": symbol,
        });

        if let Some(start_time) = start_time {
            params["startTime"] = json!(start_time);
        }
        if let Some(end_time) = end_time {
            params["endTime"] = json!(end_time);
        }
        if let Some(limit) = limit {
            params["limit"] = json!(limit);
        }

        let response: ServerResponse<FundingRateHistoryResult> =
            self.client.get(endpoint, params, SecType::None).await?;
        Ok(response)
    }

    /// Get risk limit
    ///
    /// API: GET /v5/market/risk-limit
    /// https://bybit-exchange.github.io/docs/v5/market/risk-limit
    pub async fn get_risk_limit(
        &self,
        category: Category,
        symbol: Option<&str>,
    ) -> Result<ServerResponse<RiskLimitResult>> {
        let endpoint = "v5/market/risk-limit";
        let mut params = json!({
            "category": category,
        });

        if let Some(symbol) = symbol {
            params["symbol"] = json!(symbol);
        }

        let response: ServerResponse<RiskLimitResult> =
            self.client.get(endpoint, params, SecType::None).await?;
        Ok(response)
    }

    /// Get open interest
    ///
    /// API: GET /v5/market/open-interest
    /// https://bybit-exchange.github.io/docs/v5/market/open-interest
    pub async fn get_open_interest(
        &self,
        category: Category,
        symbol: &str,
        interval_time: IntervalTime,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: Option<i32>,
        cursor: Option<&str>,
    ) -> Result<ServerResponse<OpenInterestResult>> {
        let endpoint = "v5/market/open-interest";
        let mut params = json!({
            "category": category,
            "symbol": symbol,
            "intervalTime": interval_time,
        });

        if let Some(start_time) = start_time {
            params["startTime"] = json!(start_time);
        }
        if let Some(end_time) = end_time {
            params["endTime"] = json!(end_time);
        }
        if let Some(limit) = limit {
            params["limit"] = json!(limit);
        }
        if let Some(cursor) = cursor {
            params["cursor"] = json!(cursor);
        }

        let response: ServerResponse<OpenInterestResult> =
            self.client.get(endpoint, params, SecType::None).await?;
        Ok(response)
    }

    /// Get insurance
    ///
    /// API: GET /v5/market/insurance
    /// https://bybit-exchange.github.io/docs/v5/market/insurance
    pub async fn get_insurance(
        &self,
        coin: Option<&str>,
    ) -> Result<ServerResponse<InsuranceResult>> {
        let endpoint = "v5/market/insurance";
        let mut params = json!({});

        if let Some(coin) = coin {
            params["coin"] = json!(coin);
        }

        let response: ServerResponse<InsuranceResult> =
            self.client.get(endpoint, params, SecType::None).await?;
        Ok(response)
    }

    /// Get recent trades
    ///
    /// API: GET /v5/market/recent-trade
    /// https://bybit-exchange.github.io/docs/v5/market/recent-trade
    pub async fn get_recent_trade(
        &self,
        category: Category,
        symbol: Option<&str>,
        base_coin: Option<&str>,
        option_type: Option<&str>,
        limit: Option<i32>,
    ) -> Result<ServerResponse<RecentTradeResult>> {
        let endpoint = "v5/market/recent-trade";
        let mut params = json!({
            "category": category,
        });

        if let Some(symbol) = symbol {
            params["symbol"] = json!(symbol);
        }
        if let Some(base_coin) = base_coin {
            params["baseCoin"] = json!(base_coin);
        }
        if let Some(option_type) = option_type {
            params["optionType"] = json!(option_type);
        }
        if let Some(limit) = limit {
            params["limit"] = json!(limit);
        }

        let response: ServerResponse<RecentTradeResult> =
            self.client.get(endpoint, params, SecType::None).await?;
        Ok(response)
    }

    /// Get delivery price
    ///
    /// API: GET /v5/market/delivery-price
    /// https://bybit-exchange.github.io/docs/v5/market/delivery-price
    pub async fn get_delivery_price(
        &self,
        category: Category,
        symbol: Option<&str>,
        base_coin: Option<&str>,
        limit: Option<i32>,
        cursor: Option<&str>,
    ) -> Result<ServerResponse<DeliveryPriceResult>> {
        let endpoint = "v5/market/delivery-price";
        let mut params = json!({
            "category": category,
        });

        if let Some(symbol) = symbol {
            params["symbol"] = json!(symbol);
        }
        if let Some(base_coin) = base_coin {
            params["baseCoin"] = json!(base_coin);
        }
        if let Some(limit) = limit {
            params["limit"] = json!(limit);
        }
        if let Some(cursor) = cursor {
            params["cursor"] = json!(cursor);
        }

        let response: ServerResponse<DeliveryPriceResult> =
            self.client.get(endpoint, params, SecType::None).await?;
        Ok(response)
    }

    /// Get long short ratio
    ///
    /// API: GET /v5/market/account-ratio
    /// https://bybit-exchange.github.io/docs/v5/market/long-short-ratio
    pub async fn get_long_short_ratio(
        &self,
        category: Category,
        symbol: &str,
        period: &str,
        limit: Option<i32>,
    ) -> Result<ServerResponse<LongShortRatioResult>> {
        let endpoint = "v5/market/account-ratio";
        let mut params = json!({
            "category": category,
            "symbol": symbol,
            "period": period,
        });

        if let Some(limit) = limit {
            params["limit"] = json!(limit);
        }

        let response: ServerResponse<LongShortRatioResult> =
            self.client.get(endpoint, params, SecType::None).await?;
        Ok(response)
    }

    /// Get historical volatility
    ///
    /// API: GET /v5/market/historical-volatility
    /// https://bybit-exchange.github.io/docs/v5/market/iv
    pub async fn get_historical_volatility(
        &self,
        category: Category,
        base_coin: Option<&str>,
        period: Option<i32>,
    ) -> Result<ServerResponse<HistoricalVolatilityResult>> {
        let endpoint = "v5/market/historical-volatility";
        let mut params = json!({
            "category": category,
        });

        if let Some(base_coin) = base_coin {
            params["baseCoin"] = json!(base_coin);
        }
        if let Some(period) = period {
            params["period"] = json!(period);
        }

        let response: ServerResponse<HistoricalVolatilityResult> =
            self.client.get(endpoint, params, SecType::None).await?;
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rest::ApiKeyPair;

    fn create_test_client() -> MarketClient {
        let api_key_pair = ApiKeyPair::new("test".to_string(), "".to_string(), "".to_string());
        let rest_client = RestClient::new(api_key_pair, "https://api.bybit.com".to_string());
        MarketClient::new(rest_client)
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_server_time() {
        let client = create_test_client();
        let result = client.get_server_time().await;
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.ret_code, 0);
        assert!(!response.result.time_second.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_kline() {
        let client = create_test_client();
        let result = client
            .get_kline(
                Category::Spot,
                "BTCUSDT",
                Interval::OneHour,
                None,
                None,
                Some(10),
            )
            .await;
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.ret_code, 0);
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_orderbook() {
        let client = create_test_client();
        let result = client
            .get_orderbook(Category::Spot, "BTCUSDT", Some(5))
            .await;
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.ret_code, 0);
        assert!(response.result.s.len() > 0);
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_tickers() {
        let client = create_test_client();
        let result = client
            .get_tickers(Category::Spot, Some("BTCUSDT"), None, None)
            .await;
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.ret_code, 0);
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_instruments_info() {
        let client = create_test_client();
        let result = client
            .get_instruments_info(Category::Spot, Some("BTCUSDT"), None, Some(10), None)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_mark_price_kline() {
        let client = create_test_client();
        let result = client
            .get_mark_price_kline(
                Category::Linear,
                "BTCUSDT",
                Interval::OneHour,
                None,
                None,
                Some(10),
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_funding_history() {
        let client = create_test_client();
        let result = client
            .get_funding_history(Category::Linear, "BTCUSDT", None, None, Some(10))
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_recent_trade() {
        let client = create_test_client();
        let result = client
            .get_recent_trade(Category::Spot, Some("BTCUSDT"), None, None, Some(10))
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_open_interest() {
        let client = create_test_client();
        let result = client
            .get_open_interest(
                Category::Linear,
                "BTCUSDT",
                IntervalTime::OneHour,
                None,
                None,
                Some(10),
                None,
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_risk_limit() {
        let client = create_test_client();
        let result = client
            .get_risk_limit(Category::Linear, Some("BTCUSDT"))
            .await;
        assert!(result.is_ok());
    }
}
