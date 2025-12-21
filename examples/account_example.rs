use bybit_rust_api::rest::account::dto::account_wallet::GetWalletBalanceParams;
use bybit_rust_api::rest::enums::account_type::AccountType;
use bybit_rust_api::rest::{AccountClient, ApiKeyPair, RestClient};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("BYBIT_API_KEY").unwrap_or_else(|_| "YOUR_API_KEY".to_string());
    let api_secret = env::var("BYBIT_API_SECRET").unwrap_or_else(|_| "YOUR_API_SECRET".to_string());

    // Using testnet by default
    let base_url = "https://api-testnet.bybit.com".to_string();

    // ApiKeyPair takes (profile_name, key, secret)
    let key_pair = ApiKeyPair::new("account_demo".to_string(), api_key, api_secret);
    let client = RestClient::new(key_pair, base_url);

    // AccountClient takes ownership of RestClient
    let account_client = AccountClient::new(client);

    println!("--- Testing get_wallet_balance ---");
    let params = GetWalletBalanceParams {
        account_type: AccountType::UNIFIED,
        coin: None,
    };

    match account_client.get_wallet_balance(params).await {
        Ok(response) => println!("Success: {:?}", response),
        Err(e) => println!("Error: {:?}", e),
    }

    println!("\n--- Testing get_fee_rate ---");
    match account_client.get_fee_rate("spot", None, None).await {
        Ok(response) => println!("Success: {:?}", response),
        Err(e) => println!("Error: {:?}", e),
    }

    Ok(())
}
