//! Token-bucket rate limiter shared across REST and WebSocket clients.
//!
//! Bybit V5 rate limits:
//! - Public endpoints: 50 requests/second
//! - Private endpoints: 50 requests/second (separate pool)
//! - WebSocket order entry: 20 orders/second
//!
//! The limiter uses a token bucket algorithm with async waiting.

use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

/// Token bucket rate limiter.
///
/// Tokens refill at a constant rate up to a maximum capacity.
/// When tokens are exhausted, callers wait asynchronously.
#[derive(Clone)]
pub struct RateLimiter {
    inner: Arc<Mutex<Bucket>>,
}

struct Bucket {
    tokens: f64,
    capacity: f64,
    refill_rate: f64, // tokens per second
    last_refill: Instant,
}

impl RateLimiter {
    /// Create a new rate limiter.
    ///
    /// # Arguments
    /// * `rate` - Sustained requests per second
    /// * `burst` - Maximum burst size (default: same as rate)
    pub fn new(rate: u32, burst: Option<u32>) -> Self {
        let burst = burst.unwrap_or(rate);
        RateLimiter {
            inner: Arc::new(Mutex::new(Bucket {
                tokens: burst as f64,
                capacity: burst as f64,
                refill_rate: rate as f64,
                last_refill: Instant::now(),
            })),
        }
    }

    /// Acquire a single token, waiting asynchronously if necessary.
    ///
    /// Returns immediately if a token is available, otherwise sleeps
    /// until enough tokens have refilled.
    pub async fn acquire(&self) {
        let wait = {
            let mut bucket = self.inner.lock().await;
            bucket.refill();
            if bucket.tokens >= 1.0 {
                bucket.tokens -= 1.0;
                None
            } else {
                // Calculate how long to wait for 1 token
                let needed = 1.0 - bucket.tokens;
                let wait_secs = needed / bucket.refill_rate;
                Some(Duration::from_secs_f64(wait_secs))
            }
        };

        if let Some(duration) = wait {
            tokio::time::sleep(duration).await;
            // After waiting, try again
            let mut bucket = self.inner.lock().await;
            bucket.refill();
            if bucket.tokens >= 1.0 {
                bucket.tokens -= 1.0;
            }
        }
    }

    /// Try to acquire a token without waiting.
    ///
    /// Returns `true` if a token was available and consumed.
    pub async fn try_acquire(&self) -> bool {
        let mut bucket = self.inner.lock().await;
        bucket.refill();
        if bucket.tokens >= 1.0 {
            bucket.tokens -= 1.0;
            true
        } else {
            false
        }
    }

    /// Get the current number of available tokens (for diagnostics).
    pub async fn available(&self) -> f64 {
        let mut bucket = self.inner.lock().await;
        bucket.refill();
        bucket.tokens
    }

    /// Create a rate limiter for public REST endpoints (50 req/s).
    pub fn public_rest() -> Self {
        RateLimiter::new(50, Some(50))
    }

    /// Create a rate limiter for private REST endpoints (50 req/s).
    pub fn private_rest() -> Self {
        RateLimiter::new(50, Some(50))
    }

    /// Create a rate limiter for WebSocket order entry (20 req/s).
    pub fn ws_order_entry() -> Self {
        RateLimiter::new(20, Some(20))
    }
}

impl Bucket {
    fn refill(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refill).as_secs_f64();
        self.tokens = (self.tokens + elapsed * self.refill_rate).min(self.capacity);
        self.last_refill = now;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_initial_tokens_available() {
        let limiter = RateLimiter::new(10, Some(10));
        let available = limiter.available().await;
        assert!(available > 9.0);
    }

    #[tokio::test]
    async fn test_acquire_consumes_token() {
        let limiter = RateLimiter::new(10, Some(10));
        let before = limiter.available().await;
        limiter.acquire().await;
        let after = limiter.available().await;
        assert!(after < before);
    }

    #[tokio::test]
    async fn test_try_acquire() {
        let limiter = RateLimiter::new(100, Some(100));
        assert!(limiter.try_acquire().await);
        assert!(limiter.try_acquire().await);
    }

    #[tokio::test]
    async fn test_burst_behavior() {
        let limiter = RateLimiter::new(100, Some(5));
        // Should be able to burst-acquire up to 5 immediately
        for _ in 0..5 {
            assert!(limiter.try_acquire().await);
        }
        // 6th should fail (burst exhausted)
        assert!(!limiter.try_acquire().await);
    }

    #[test]
    fn test_default_constructors() {
        let _public = RateLimiter::public_rest();
        let _private = RateLimiter::private_rest();
        let _ws = RateLimiter::ws_order_entry();
    }
}
