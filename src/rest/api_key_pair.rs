use anyhow::Context;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Clone, Deserialize)]
pub struct ApiKeyPair {
    profile_name: String,
    key: String,
    secret: String,
}

#[derive(Debug, Deserialize)]
pub struct ApiKeyPairs {
    #[serde(rename = "profiles")]
    pairs: HashMap<String, ApiKeyPair>,
}

impl ApiKeyPair {
    pub fn new(profile_name: String, key: String, secret: String) -> ApiKeyPair {
        ApiKeyPair {
            profile_name: profile_name.to_string(),
            key: key.to_string(),
            secret: secret.to_string(),
        }
    }

    /// Load API credentials from environment variables.
    ///
    /// Reads `BYBIT_API_KEY` and `BYBIT_API_SECRET`.
    /// Optionally loads `.env` file via `dotenvy`.
    ///
    /// # Example
    /// ```ignore
    /// // .env file or environment:
    /// // BYBIT_API_KEY=your_key
    /// // BYBIT_API_SECRET=your_secret
    ///
    /// dotenvy::dotenv().ok(); // load .env if present
    /// let keys = ApiKeyPair::from_env()?;
    /// ```
    pub fn from_env() -> anyhow::Result<Self> {
        let key = std::env::var("BYBIT_API_KEY").context("BYBIT_API_KEY not set")?;
        let secret = std::env::var("BYBIT_API_SECRET").context("BYBIT_API_SECRET not set")?;
        Ok(ApiKeyPair::new("env".to_string(), key, secret))
    }

    /// Load API credentials from Testnet environment variables.
    ///
    /// Reads `BYBIT_TESTNET_API_KEY` and `BYBIT_TESTNET_API_SECRET`.
    pub fn from_env_testnet() -> anyhow::Result<Self> {
        let key = std::env::var("BYBIT_TESTNET_API_KEY")
            .or_else(|_| std::env::var("BYBIT_API_KEY"))
            .context("BYBIT_API_KEY or BYBIT_TESTNET_API_KEY not set")?;
        let secret = std::env::var("BYBIT_TESTNET_API_SECRET")
            .or_else(|_| std::env::var("BYBIT_API_SECRET"))
            .context("BYBIT_API_SECRET or BYBIT_TESTNET_API_SECRET not set")?;
        Ok(ApiKeyPair::new("testnet".to_string(), key, secret))
    }

    pub fn profile_name(&self) -> &str {
        self.profile_name.as_str()
    }

    pub fn key(&self) -> &str {
        self.key.as_str()
    }

    pub fn secret(&self) -> &str {
        self.secret.as_str()
    }
}

impl ApiKeyPairs {
    pub fn new() -> ApiKeyPairs {
        ApiKeyPairs {
            pairs: HashMap::new(),
        }
    }
}

impl Default for ApiKeyPairs {
    fn default() -> Self {
        Self::new()
    }
}

impl ApiKeyPairs {
    pub fn add(&mut self, profile_name: String, key: String, secret: String) {
        self.pairs.insert(
            profile_name.to_string(),
            ApiKeyPair::new(profile_name, key, secret),
        );
    }

    pub fn get(&self, profile_name: &str) -> Option<&ApiKeyPair> {
        self.pairs.get(profile_name)
    }

    /// Load API keys from a JSON file.
    ///
    /// Expected format: array of `{ "profile_name", "key", "secret" }` objects.
    pub fn load_from_json_file(file_path: &str) -> Result<ApiKeyPairs, Box<dyn std::error::Error>> {
        let mut api_key_pairs = ApiKeyPairs::new();
        let pairs: Vec<ApiKeyPair> =
            serde_json::from_reader(BufReader::new(File::open(file_path)?))?;
        for pair in pairs {
            api_key_pairs.add(pair.profile_name, pair.key, pair.secret);
        }
        Ok(api_key_pairs)
    }
}
