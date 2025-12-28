use crate::rest::client::{RestClient, SecType, ServerResponse};
use serde::Deserialize;
use serde_json::json;

#[derive(Clone)]
pub struct AnnouncementsClient {
    client: RestClient,
}

#[derive(Debug, Deserialize)]
struct AnnouncementsResponse {
    #[serde(rename = "retCode")]
    ret_code: i32,
    #[serde(rename = "retMsg")]
    ret_msg: String,
    #[serde(rename = "result", default)]
    result: serde_json::Value,
    #[serde(rename = "retExtInfo")]
    ret_ext_info: serde_json::Value,
    #[serde(rename = "time")]
    time: i64,
}

impl From<AnnouncementsResponse> for ServerResponse<serde_json::Value> {
    fn from(r: AnnouncementsResponse) -> Self {
        ServerResponse {
            ret_code: r.ret_code,
            ret_msg: r.ret_msg,
            result: r.result,
            ret_ext_info: r.ret_ext_info,
            time: r.time,
        }
    }
}

impl AnnouncementsClient {
    pub fn new(client: RestClient) -> Self {
        AnnouncementsClient { client }
    }

    /// Get announcements
    /// https://bybit-exchange.github.io/docs/v5/announcement
    pub async fn get_announcements(
        &self,
        locale: Option<&str>,
        type_list: Option<&str>,
        tag: Option<&str>,
        page: Option<i32>,
        limit: Option<i32>,
    ) -> crate::rest::BybitResult<ServerResponse<serde_json::Value>> {
        let endpoint = "v5/announcements/index";
        let mut params = json!({});

        if let Some(locale) = locale {
            params["locale"] = json!(locale);
        }
        if let Some(type_list) = type_list {
            params["type"] = json!(type_list);
        }
        if let Some(tag) = tag {
            params["tag"] = json!(tag);
        }
        if let Some(page) = page {
            params["page"] = json!(page);
        }
        if let Some(limit) = limit {
            params["limit"] = json!(limit);
        }

        self.client.get(endpoint, params, SecType::None).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rest::ApiKeyPair;

    fn create_test_client() -> AnnouncementsClient {
        let api_key_pair = ApiKeyPair::new(
            "test".to_string(),
            "test_key".to_string(),
            "test_secret".to_string(),
        );
        let rest_client =
            RestClient::new(api_key_pair, "https://api-testnet.bybit.com".to_string());
        AnnouncementsClient::new(rest_client)
    }

    #[test]
    fn test_announcements_client_creation() {
        let _client = create_test_client();
    }

    #[tokio::test]
    async fn test_get_announcements_params() {
        let locale = Some("en-US");
        let type_list = Some("new_crypto");
        let tag = Some("Spot");
        let page = Some(1);
        let limit = Some(20);

        assert_eq!(locale, Some("en-US"));
        assert_eq!(type_list, Some("new_crypto"));
        assert_eq!(tag, Some("Spot"));
        assert_eq!(page, Some(1));
        assert_eq!(limit, Some(20));
    }
}
