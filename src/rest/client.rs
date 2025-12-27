use crate::consts::{
    API_REQUEST_KEY, RECV_WINDOW_KEY, SIGNATURE_KEY, SIGN_TYPE_KEY, TIMESTAMP_KEY,
};
use crate::rest::api_key_pair::ApiKeyPair;
use crate::utils::{millis, sign};
use anyhow::Result;
use reqwest::RequestBuilder;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SecType {
    None = 0,
    Signed = 1,
}

#[derive(Clone)]
pub struct RestClient {
    api_key_pair: ApiKeyPair,
    base_url: String,

    http_client: reqwest::Client,
    recv_window: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerResponse<T> {
    #[serde(rename = "retCode")]
    pub ret_code: i32,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    #[serde(rename = "result")]
    pub result: T,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: serde_json::Value,
    #[serde(rename = "time")]
    pub time: i64,
}

impl RestClient {
    pub fn new(api_key_pair: ApiKeyPair, base_url: String) -> RestClient {
        RestClient {
            api_key_pair,
            base_url,
            http_client: reqwest::Client::new(),
            recv_window: "5000".to_string(),
        }
    }

    pub fn with_recv_window(mut self, recv_window: impl Into<String>) -> Self {
        self.recv_window = recv_window.into();
        self
    }

    fn query_string(&self, query: serde_json::Value) -> Result<String> {
        let object = query
            .as_object()
            .ok_or(anyhow::anyhow!("Query params must be a JSON object"))?;
        Ok(serde_urlencoded::to_string(object)?)
    }

    fn sign_request(
        &self,
        request_builder: RequestBuilder,
        query_or_body_param: String,
    ) -> RequestBuilder {
        let recv_window = &self.recv_window;
        let timestamp_str = millis().to_string();
        let signature = sign(
            &self.api_key_pair.secret(),
            &format!(
                "{}{}{}{}",
                timestamp_str,
                self.api_key_pair.key(),
                recv_window,
                query_or_body_param
            ),
        );

        request_builder
            .header(SIGN_TYPE_KEY, "2")
            .header(API_REQUEST_KEY, self.api_key_pair.key())
            .header(TIMESTAMP_KEY, timestamp_str)
            .header(RECV_WINDOW_KEY, recv_window)
            .header(SIGNATURE_KEY, signature)
    }

    pub async fn get<A: DeserializeOwned>(
        &self,
        endpoint: &str,
        query: serde_json::Value,
        sec_type: SecType,
    ) -> Result<A> {
        let mut url = format!("{}/{}", self.base_url, endpoint);
        let query_string = self.query_string(query)?;

        if !query_string.is_empty() {
            url.push_str(&format!("?{}", query_string));
        }

        let mut request_builder = self.http_client.get(&url);
        if sec_type == SecType::Signed {
            request_builder = self.sign_request(request_builder, query_string);
        }

        log::debug!("url: {}", url);

        let r = request_builder.send().await?;
        let server_response: A = r.json().await?;
        Ok(server_response)
    }

    pub async fn post<A: DeserializeOwned>(
        &self,
        endpoint: &str,
        body: serde_json::Value,
        sec_type: SecType,
    ) -> Result<A, reqwest::Error> {
        let url = format!("{}/{}", self.base_url, endpoint);
        let mut request_builder = self.http_client.post(&url);
        if sec_type == SecType::Signed {
            request_builder = self.sign_request(
                request_builder,
                serde_urlencoded::to_string(body.as_object().unwrap()).unwrap(),
            );
        }

        let server_response = request_builder.json(&body).send().await?.json().await?;
        Ok(server_response)
    }
}
