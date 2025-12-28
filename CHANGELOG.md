# Changelog

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
