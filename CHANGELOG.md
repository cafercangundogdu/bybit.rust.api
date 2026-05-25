# Changelog

## [0.4.0] - 2026-05-26

> **Dependency pinning**: This release pins all dependencies to exact versions for reproducible builds.

### 🚀 Major Features

- **WebSocket Support**: Full WebSocket streaming implementation across all Bybit V5 channels.
  - **Public channels (5)**: Orderbook (snapshot/delta), Trade, Ticker, Kline, Liquidation
  - **Private channels (6)**: Position, Execution, Order, Wallet, Greeks, DCP
  - **Trade channels (3)**: Create, amend, cancel orders via WebSocket
  - **Auth**: HMAC-SHA256 signing for private topic authentication
  - **Auto-reconnect**: Exponential backoff with automatic re-subscription
  - **Auto-ping**: 20-second heartbeat to maintain connection
  - **Stream API**: Implements `futures::Stream` for async iteration
  - **Graceful shutdown**: `WsClient::close()` + `Drop` impl

- **Rate Limiter**: Token-bucket algorithm shared across REST and WebSocket clients.
  - `RateLimiter::public_rest()` — 50 req/s
  - `RateLimiter::private_rest()` — 50 req/s
  - `RateLimiter::ws_order_entry()` — 20 req/s
  - Integrated into `RestClient::get()` / `RestClient::post()`
  - Builder: `RestClient::with_rate_limiter()`

- **Environment Variable Support**:
  - `ApiKeyPair::from_env()` — reads `BYBIT_API_KEY` / `BYBIT_API_SECRET`
  - `ApiKeyPair::from_env_testnet()` — reads testnet variants
  - `dotenvy` integration in all examples and CLI
  - `.env.example` template

- **CLI**: Command-line interface for quick market data queries.
  - `bybit-cli time` — server time
  - `bybit-cli ticker [SYMBOL]` — 24hr stats
  - `bybit-cli kline [SYMBOL] [INTERVAL] [N]` — candlesticks
  - `bybit-cli orderbook [SYMBOL] [DEPTH]` — orderbook

- **Regional Endpoints**: Support for TR, KZ, GE, AE, NL endpoints.

- **CI/CD**: GitHub Actions workflow (fmt + clippy + test + build on ubuntu + macos).

### ⚡ Dependency Updates

- **Removed**: `ring` (unused), `serde_yaml` (deprecated)
- **Updated**: `tokio-tungstenite` 0.27 → 0.29, `hmac` 0.12 → 0.13
- **Updated**: `sha2` 0.10 → 0.11, `reqwest` 0.12 → 0.13 (`rustls-tls` → `rustls`)
- **Updated**: `tokio` 1.47 → 1.52, `thiserror` 2.0.16 → 2.0.18
- **Updated**: `chrono` 0.4.41 → 0.4.44, `anyhow` 1.0.99 → 1.0.102
- **Updated**: `url` 2.5.6 → 2.5.8, `futures-util` 0.3.31 → 0.3.32
- **Added**: `dotenvy` 0.15.7
- **Added dev-deps**: `wiremock` 0.6.5, `tokio-test` 0.4.5, `env_logger` 0.11.10

### 🐛 Fixes

- Fixed `hmac::KeyInit` import for hmac 0.13 compatibility
- Fixed unused `ring` dependency
- Fixed unused `Read` import
- Fixed unqualified path import warnings
- Fixed `Interval` enum variant names in CLI

### 🧪 Tests

- 15 WebSocket unit tests (parse + topic matching)
- 8 wiremock HTTP mock tests (success, error, 500, rate limit, malformed)
- Total: 103 tests, 0 failures, 10 ignored (live API calls)

### 📦 Packaging

- Added `LICENSE` (MIT)
- Added `.env.example`
- Comprehensive `lib.rs` docs for docs.rs

## [0.3.0] - 2025-12-28

### ⚠ Breaking Changes

- **Error Handling Implementation**: The library now uses a custom `BybitError` type (via `thiserror`) instead of generic `anyhow::Result`.
  - **Action Required**: Update your error handling code to match `BybitResult<T>` or `Result<T, BybitError>`.
- **Response Type Changes**: All client methods now return `BybitResult<ServerResponse<T>>`.
  - This provides access to `ret_code`, `ret_msg`, `time`, and `ret_ext_info` alongside the result.
  - **Action Required**: You may need to access `.result` on the returned response to get the data payload.
- **Flattened API Structure**: Module paths have been simplified for better ergonomics.
  - `enums::category::Category` -> `enums::Category` (and re-exported at crate root).
  - `dto::headers::Header` -> `dto::Header` (example pattern).
  - **Action Required**: Update your import paths. Most common types are now available at the top level or directly under `enums`/`dto`.

### 🚀 Features & Improvements

- **Automatic API Error Validation**: The client now inspects `retCode` in the response. If non-zero, it automatically returns `BybitError::Api(ErrorCode)`, allowing for type-safe error handling of Bybit specific errors.
- **Robust Error Parsing**: Improved deserialization logic to handle cases where an API error response has a different JSON structure than the success response, preventing "JSON parse fail" from masking the actual API error.
- **Thread Safety**: Confirmed and tested full `Send` + `Sync` support for multi-threaded applications.
- **Documentation**: Added comprehensive crate-level documentation and a "Quick Start" guide in `lib.rs`.

### 🐛 Bug Fixes

- **Critical POST Signature Fix**: Fixed a bug where POST request signatures were generated using URL-encoding instead of JSON, which caused requests to fail on V5 API.
- **Linting**: Resolved all `clippy` warnings for a clean build.

### 📦 Dependencies

- Added `thiserror` for library-grade error handling.
