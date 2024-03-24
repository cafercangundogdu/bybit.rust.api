use crate::consts::MAINNET;
use crate::rest::account::dto::account_wallet::{GetWalletBalanceParams, GetWalletBalanceResponse};
use crate::rest::account::dto::transaction_log::{
    GetTransactionLogParams, GetTransactionLogResponse,
};
use crate::rest::api_key_pair::ApiKeyPair;
use crate::rest::client::{RestClient, SecType};

struct AccountClient {
    rest_client: RestClient,
}

impl AccountClient {
    pub fn new(api_key_pair: ApiKeyPair) -> AccountClient {
        AccountClient {
            rest_client: RestClient::new(api_key_pair, MAINNET.to_string(), true),
        }
    }

    pub async fn get_transaction_log(
        &self,
        params: &GetTransactionLogParams,
    ) -> Result<GetTransactionLogResponse, reqwest::Error> {
        let response: GetTransactionLogResponse = self
            .rest_client
            .get(
                "v5/account/transaction-log",
                serde_json::to_value(params).unwrap(),
                SecType::Signed,
            )
            .await?;
        Ok(response)
    }

    pub async fn get_wallet_balance(
        &self,
        params: &GetWalletBalanceParams,
    ) -> Result<GetWalletBalanceResponse, reqwest::Error> {
        let response: GetWalletBalanceResponse = self
            .rest_client
            .get(
                "v5/account/wallet-balance",
                serde_json::to_value(params).unwrap(),
                SecType::Signed,
            )
            .await?;
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rest::account::account_type::AccountType;
    use AccountType::UTAUnified;

    #[test]
    fn test_get_wallet_balance() {
        let api_key_pair =
            ApiKeyPair::new("ninja".to_string(), "key".to_string(), "secret".to_string());
        let account_client = AccountClient::new(api_key_pair);

        let params = GetWalletBalanceParams {
            account_type: UTAUnified,
        };

        let rt = tokio::runtime::Runtime::new().unwrap();
        let response = rt.block_on(account_client.get_wallet_balance(&params));

        match response {
            Ok(balance) => {
                println!("{:?}", balance);
                assert!(balance.into_response().time > 0);
            }
            Err(e) => {
                eprintln!("Error occurred: {}", e);
            }
        }
    }
}
