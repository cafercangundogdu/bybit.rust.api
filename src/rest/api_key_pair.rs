use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};

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

    pub fn add(&mut self, profile_name: String, key: String, secret: String) {
        self.pairs.insert(
            profile_name.to_string(),
            ApiKeyPair::new(profile_name, key, secret),
        );
    }

    pub fn get(&self, profile_name: &str) -> Option<&ApiKeyPair> {
        self.pairs.get(profile_name)
    }

    /*
    [
        {
            "profile_name": "profile1",
            "key": "key1",
            "secret": "secret1"
        },
        {
            "profile_name": "profile2",
            "key": "key2",
            "secret": "secret2"
        }
    ]
     */
    pub fn load_from_json_file(file_path: &str) -> Result<ApiKeyPairs, Box<dyn std::error::Error>> {
        let mut api_key_pairs = ApiKeyPairs::new();
        let pairs: Vec<ApiKeyPair> =
            serde_json::from_reader(BufReader::new(File::open(file_path)?))?;
        for pair in pairs {
            api_key_pairs.add(pair.profile_name, pair.key, pair.secret);
        }
        Ok(api_key_pairs)
    }

    /*
    profiles:
        profile1:
            key: "key1"
            secret: "secret1"
        profile2:
            key: "key2"
            secret: "secret2"
    */

    pub fn load_from_yaml_file(file_path: &str) -> Result<ApiKeyPairs, Box<dyn std::error::Error>> {
        let mut file = File::open(file_path).expect(&format!("Unable to open file: {}", file_path));
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect(&format!("Unable to read file: {}", file_path));
        let pairs: ApiKeyPairs =
            serde_yaml::from_str(&contents).expect(&format!("Unable to parse file: {}", file_path));

        Ok(pairs)
    }
}
