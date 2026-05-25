//! Wiremock-based integration tests for REST client.
//!
//! These tests mock the Bybit HTTP API using a local wiremock server,
//! allowing thorough testing of HTTP request/response handling,
//! authentication signatures, error parsing, and edge cases
//! without hitting the real API.

use bybit_rust_api::rest::{ApiKeyPair, MarketClient, RestClient};
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

/// Helper: create a test client pointing at a mock server URL.
fn mock_client(server: &MockServer) -> MarketClient {
    let api_key_pair = ApiKeyPair::new(
        "test".to_string(),
        "test_key".to_string(),
        "test_secret".to_string(),
    );
    let rest_client = RestClient::new(api_key_pair, server.uri());
    MarketClient::new(rest_client)
}

#[tokio::test]
async fn test_get_server_time_success() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("v5/market/time"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "retCode": 0,
            "retMsg": "OK",
            "retExtInfo": {},
            "time": 1672828800000_i64,
            "result": {
                "timeSecond": "1672828800",
                "timeNano": "1672828800000000000"
            }
        })))
        .mount(&server)
        .await;

    let client = mock_client(&server);
    let result = client.get_server_time().await;

    assert!(result.is_ok(), "Expected Ok, got: {:?}", result.err());
    let response = result.unwrap();
    assert_eq!(response.ret_code, 0);
    assert_eq!(response.ret_msg, "OK");
}

#[tokio::test]
async fn test_api_error_handling() {
    let server = MockServer::start().await;

    // Simulate an API error (e.g., invalid parameter)
    Mock::given(method("GET"))
        .and(path("v5/market/time"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "retCode": 10001,
            "retMsg": "internal error",
            "retExtInfo": {},
            "time": 1672828800000_i64,
            "result": {}
        })))
        .mount(&server)
        .await;

    let client = mock_client(&server);
    let result = client.get_server_time().await;

    assert!(result.is_err(), "Expected error for non-zero retCode");
    let err = result.unwrap_err();
    let err_str = err.to_string();
    assert!(
        err_str.contains("API error"),
        "Expected API error, got: {}",
        err_str
    );
}

#[tokio::test]
async fn test_http_500_error() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("v5/market/time"))
        .respond_with(ResponseTemplate::new(500).set_body_string("Internal Server Error"))
        .mount(&server)
        .await;

    let client = mock_client(&server);
    let result = client.get_server_time().await;

    assert!(result.is_err(), "Expected error for HTTP 500");
}

#[tokio::test]
async fn test_malformed_json_response() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("v5/market/time"))
        .respond_with(ResponseTemplate::new(200).set_body_string("not json"))
        .mount(&server)
        .await;

    let client = mock_client(&server);
    let result = client.get_server_time().await;

    assert!(result.is_err(), "Expected error for malformed JSON");
}

#[tokio::test]
async fn test_get_tickers_success() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("v5/market/tickers"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "retCode": 0,
            "retMsg": "OK",
            "retExtInfo": {},
            "time": 1672828800000_i64,
            "result": {
                "category": "spot",
                "list": [{
                    "symbol": "BTCUSDT",
                    "lastPrice": "50000.00",
                    "highPrice24h": "51000.00",
                    "lowPrice24h": "49000.00",
                    "volume24h": "15000.5",
                    "turnover24h": "750000000.00",
                    "bid1Price": "49990.00",
                    "bid1Size": "1.5",
                    "ask1Price": "50010.00",
                    "ask1Size": "2.0",
                    "prevPrice24h": "49500.00",
                    "price24hPcnt": "0.01"
                }]
            }
        })))
        .mount(&server)
        .await;

    let client = mock_client(&server);
    let result = client
        .get_tickers(bybit_rust_api::Category::Spot, Some("BTCUSDT"), None, None)
        .await;

    assert!(result.is_ok(), "Expected Ok, got: {:?}", result.err());
    let response = result.unwrap();
    assert_eq!(response.ret_code, 0);
}

#[tokio::test]
async fn test_get_kline_success() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("v5/market/kline"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "retCode": 0,
            "retMsg": "OK",
            "retExtInfo": {},
            "time": 1672828800000_i64,
            "result": {
                "symbol": "BTCUSDT",
                "category": "spot",
                "list": [
                    ["1672828800000", "50000", "50100", "50200", "49900", "150", "7500000"]
                ]
            }
        })))
        .mount(&server)
        .await;

    let client = mock_client(&server);
    let result = client
        .get_kline(
            bybit_rust_api::Category::Spot,
            "BTCUSDT",
            bybit_rust_api::Interval::OneHour,
            None,
            None,
            Some(1),
        )
        .await;

    assert!(result.is_ok(), "Expected Ok, got: {:?}", result.err());
}

#[tokio::test]
async fn test_rate_limit_response() {
    let server = MockServer::start().await;

    // Simulate rate limit exceeded
    Mock::given(method("GET"))
        .and(path("v5/market/time"))
        .respond_with(ResponseTemplate::new(429).set_body_json(json!({
            "retCode": 10006,
            "retMsg": "rate limit exceeded",
            "retExtInfo": {},
            "time": 1672828800000_i64,
            "result": {}
        })))
        .mount(&server)
        .await;

    let client = mock_client(&server);
    let result = client.get_server_time().await;

    // 429 might come through as HTTP error before JSON parsing,
    // or as a Bybit API error. Either way it should be an error.
    assert!(result.is_err(), "Rate limit should produce an error");
}

#[tokio::test]
async fn test_get_orderbook_success() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("v5/market/orderbook"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "retCode": 0,
            "retMsg": "OK",
            "retExtInfo": {},
            "time": 1672828800000_i64,
            "result": {
                "s": "BTCUSDT",
                "b": [["50000.00", "1.5"], ["49900.00", "2.0"]],
                "a": [["50100.00", "1.0"], ["50200.00", "0.5"]],
                "ts": 1672828800000_i64,
                "u": 1,
                "seq": 100
            }
        })))
        .mount(&server)
        .await;

    let client = mock_client(&server);
    let result = client
        .get_orderbook(bybit_rust_api::Category::Spot, "BTCUSDT", Some(5))
        .await;

    assert!(result.is_ok(), "Expected Ok, got: {:?}", result.err());
    let response = result.unwrap();
    assert_eq!(response.ret_code, 0);
}
