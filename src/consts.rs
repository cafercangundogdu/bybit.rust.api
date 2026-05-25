//! Bybit API endpoint constants.
//!
//! Includes mainnet, testnet, and regional endpoints for both
//! REST and WebSocket connections.

pub const NAME: &str = "bybit.api.rust";
pub const VERSION: &str = "0.4.0";

// ── REST Endpoints ─────────────────────────────────────────────

/// Mainnet REST API (global)
pub const REST_MAINNET: &str = "https://api.bybit.com";
/// Mainnet REST API (backup)
pub const REST_MAINNET_BACKUP: &str = "https://api.bytick.com";
/// Testnet REST API
pub const REST_TESTNET: &str = "https://api-testnet.bybit.com";

// Regional REST endpoints (V5 requirement)
pub const REST_TR: &str = "https://api.bybit.tr";
pub const REST_KZ: &str = "https://api.bybit.kz";
pub const REST_GE: &str = "https://api.bybitgeorgia.ge";
pub const REST_AE: &str = "https://api.bybit.ae";
pub const REST_NL: &str = "https://api.bybit.nl";

// ── WebSocket Public Endpoints ─────────────────────────────────

pub const WS_SPOT_MAINNET: &str = "wss://stream.bybit.com/v5/public/spot";
pub const WS_LINEAR_MAINNET: &str = "wss://stream.bybit.com/v5/public/linear";
pub const WS_INVERSE_MAINNET: &str = "wss://stream.bybit.com/v5/public/inverse";
pub const WS_OPTION_MAINNET: &str = "wss://stream.bybit.com/v5/public/option";

pub const WS_SPOT_TESTNET: &str = "wss://stream-testnet.bybit.com/v5/public/spot";
pub const WS_LINEAR_TESTNET: &str = "wss://stream-testnet.bybit.com/v5/public/linear";
pub const WS_INVERSE_TESTNET: &str = "wss://stream-testnet.bybit.com/v5/public/inverse";
pub const WS_OPTION_TESTNET: &str = "wss://stream-testnet.bybit.com/v5/public/option";

// ── WebSocket Private Endpoints ────────────────────────────────

pub const WS_PRIVATE_MAINNET: &str = "wss://stream.bybit.com/v5/private";
pub const WS_PRIVATE_TESTNET: &str = "wss://stream-testnet.bybit.com/v5/private";

// ── WebSocket Trade Endpoints ──────────────────────────────────

pub const WS_TRADE_MAINNET: &str = "wss://stream.bybit.com/v5/trade";
pub const WS_TRADE_TESTNET: &str = "wss://stream-testnet.bybit.com/v5/trade";

// ── HTTP Headers ───────────────────────────────────────────────

pub const TIMESTAMP_KEY: &str = "X-BAPI-TIMESTAMP";
pub const SIGNATURE_KEY: &str = "X-BAPI-SIGN";
pub const API_REQUEST_KEY: &str = "X-BAPI-API-KEY";
pub const RECV_WINDOW_KEY: &str = "X-BAPI-RECV-WINDOW";
pub const SIGN_TYPE_KEY: &str = "X-BAPI-SIGN-TYPE";

// ── Legacy (aliased for backward compat) ───────────────────────

pub use REST_MAINNET as MAINNET;
pub use REST_MAINNET_BACKUP as MAINNET_BACKT;
pub use REST_TESTNET as TESTNET;

pub use WS_INVERSE_MAINNET as INVERSE_MAINNET;
pub use WS_INVERSE_TESTNET as INVERSE_TESTNET;
pub use WS_LINEAR_MAINNET as LINEAR_MAINNET;
pub use WS_LINEAR_TESTNET as LINEAR_TESTNET;
pub use WS_OPTION_MAINNET as OPTION_MAINNET;
pub use WS_OPTION_TESTNET as OPTION_TESTNET;
pub use WS_PRIVATE_MAINNET as WEBSOCKET_PRIVATE_MAINNET;
pub use WS_PRIVATE_TESTNET as WEBSOCKET_PRIVATE_TESTNET;
pub use WS_SPOT_MAINNET as SPOT_MAINNET;
pub use WS_SPOT_TESTNET as SPOT_TESTNET;
