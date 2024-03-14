use crate::handler::validate_params;
use anyhow::Result;
use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct ServerResponse {}

/*pub struct BrokerServiceClient {
    client: Client,
    api_key: String,
    secret: String,
    pub base_url: String,
    pub recv_window: u64,
}*/
struct BrokerServiceClient {
    client: Client,
    params: HashMap<String, Option<String>>,
}

impl BrokerServiceClient {
    pub fn new() -> Self {
        BrokerServiceClient {
            client: Client::new(),
            params: HashMap::new(),
        }
    }

    pub async fn get_broker_earning(&self) -> Result<ServerResponse, Box<dyn std::error::Error>> {
        if let Err(e) = validate_params(&self.params) {
            return Err(Box::new(e));
        }

        let response = self
            .client
            .get("https://api.bybit.com/v5/broker/earnings-info")
            .query(&self.params)
            .send()
            .await?
            .json::<ServerResponse>()
            .await?;

        Ok(response)
    }
}
