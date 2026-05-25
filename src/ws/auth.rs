//! WebSocket authentication for Bybit V5 private channels.
//!
//! Authentication is required for private WebSocket topics (position, execution,
//! order, wallet). The process:
//! 1. Generate an `expires` timestamp (current ms + 10 seconds)
//! 2. Sign the expires value with HMAC-SHA256 using the API secret
//! 3. Send `{"op": "auth", "args": [api_key, expires, signature]}`

use crate::utils;

/// Generate WebSocket authentication parameters.
///
/// Returns `(expires, signature)` for use in the auth request.
///
/// # Arguments
/// * `secret` - Your Bybit API secret
///
/// # Example
/// ```ignore
/// let (expires, sig) = generate_auth_params("my_secret");
/// let auth_msg = WsRequest::auth("my_key", expires, &sig);
/// ```
pub fn generate_auth_params(secret: &str) -> (u64, String) {
    let expires = utils::millis() as u64 + 10_000; // 10 seconds from now
    let expires_str = expires.to_string();
    let signature = utils::sign(secret, &expires_str);
    (expires, signature)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_auth_params() {
        let (expires, sig) = generate_auth_params("test_secret");
        assert!(expires > 0);
        assert!(!sig.is_empty());
        assert_eq!(sig.len(), 64); // SHA-256 hex is 64 chars
    }

    #[test]
    fn test_auth_params_deterministic_at_same_time() {
        // Same secret should produce different signatures at different times
        let (e1, s1) = generate_auth_params("secret");
        std::thread::sleep(std::time::Duration::from_millis(10));
        let (e2, s2) = generate_auth_params("secret");
        // expires should differ
        assert!(e2 > e1);
        // signatures should differ because expires differs
        assert_ne!(s1, s2);
    }
}
