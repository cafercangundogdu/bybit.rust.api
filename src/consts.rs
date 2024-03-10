// Https
pub const MAINNET: &str       = "https://api.bybit.com";
pub const MAINNET_BACKT: &str = "https://api.bytick.com";
pub const TESTNET      : &str = "https://api-testnet.bybit.com";
// WebSocket public channel - Mainnet
pub const SPOT_MAINNET   : &str = "wss://stream.bybit.com/v5/public/spot";
pub const LINEAR_MAINNET : &str = "wss://stream.bybit.com/v5/public/linear";
pub const INVERSE_MAINNET: &str = "wss://stream.bybit.com/v5/public/inverse";
pub const OPTION_MAINNET : &str = "wss://stream.bybit.com/v5/public/option";
// WebSocket public channel - Testnet
pub const SPOT_TESTNET   : &str = "wss://stream-testnet.bybit.com/v5/public/spot";
pub const LINEAR_TESTNET : &str = "wss://stream-testnet.bybit.com/v5/public/linear";
pub const INVERSE_TESTNET: &str = "wss://stream-testnet.bybit.com/v5/public/inverse";
pub const OPTION_TESTNET : &str = "wss://stream-testnet.bybit.com/v5/public/option";
// WebSocket private channel
pub const WEBSOCKET_PRIVATE_MAINNET: &str = "wss://stream.bybit.com/v5/private";
pub const WEBSOCKET_PRIVATE_TESTNET: &str = "wss://stream-testnet.bybit.com/v5/private";
// V3
pub const V3_CONTRACT_PRIVATE: &str = "wss://stream.bybit.com/contract/private/v3";
pub const V3_UNIFIED_PRIVATE : &str = "wss://stream.bybit.com/unified/private/v3";
pub const V3_SPOT_PRIVATE    : &str = "wss://stream.bybit.com/spot/private/v3";
// Globals
pub const TIMESTAMP_KEY: &str = "X-BAPI-TIMESTAMP";
pub const SIGNATURE_KEY: &str = "X-BAPI-SIGN";
pub const API_REQUEST_KEY: &str = "X-BAPI-API-KEY";
pub const RECV_WINDOW_KEY: &str = "X-BAPI-RECV-WINDOW";
pub const SIGN_TYPE_KEY: &str = "X-BAPI-SIGN-TYPE";
