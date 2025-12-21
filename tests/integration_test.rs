use bybit_rust_api::rest::enums::category::Category;
use bybit_rust_api::rest::*;

/// Test helper to create a test REST client
fn create_test_client() -> RestClient {
    let api_key_pair = ApiKeyPair::new("test".to_string(), "".to_string(), "".to_string());

    RestClient::new(api_key_pair, "https://api.bybit.com".to_string())
}

#[cfg(test)]
mod market_tests {
    use super::*;

    #[tokio::test]
    async fn test_market_client_creation() {
        let client = create_test_client();
        let _market_client = MarketClient::new(client);
    }

    #[tokio::test]
    #[ignore]
    async fn test_server_time() {
        let client = create_test_client();
        let market_client = MarketClient::new(client);

        let result = market_client.get_server_time().await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.ret_code, 0);
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_tickers() {
        let client = create_test_client();
        let market_client = MarketClient::new(client);

        let result = market_client
            .get_tickers(Category::Spot, Some("BTCUSDT"), None, None)
            .await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.ret_code, 0);
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_orderbook() {
        let client = create_test_client();
        let market_client = MarketClient::new(client);

        let result = market_client
            .get_orderbook(Category::Spot, "BTCUSDT", Some(5))
            .await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.ret_code, 0);
    }
}

#[cfg(test)]
mod account_tests {
    use super::*;

    #[tokio::test]
    async fn test_account_client_creation() {
        let client = create_test_client();
        let _account_client = AccountClient::new(client);
    }
}

#[cfg(test)]
mod order_tests {
    use super::*;

    #[tokio::test]
    async fn test_order_client_creation() {
        let client = create_test_client();
        let _order_client = OrderClient::new(client);
    }
}

#[cfg(test)]
mod position_tests {
    use super::*;

    #[tokio::test]
    async fn test_position_client_creation() {
        let client = create_test_client();
        let _position_client = PositionClient::new(client);
    }
}

#[cfg(test)]
mod asset_tests {
    use super::*;

    #[tokio::test]
    async fn test_asset_client_creation() {
        let client = create_test_client();
        let _asset_client = AssetClient::new(client);
    }
}

#[cfg(test)]
mod user_tests {
    use super::*;

    #[tokio::test]
    async fn test_user_client_creation() {
        let client = create_test_client();
        let _user_client = UserClient::new(client);
    }
}

#[cfg(test)]
mod announcements_tests {
    use super::*;

    #[tokio::test]
    async fn test_announcements_client_creation() {
        let client = create_test_client();
        let _announcements_client = AnnouncementsClient::new(client);
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_announcements() {
        let client = create_test_client();
        let announcements_client = AnnouncementsClient::new(client);

        let result = announcements_client
            .get_announcements(Some("en-US"), None, None, None, Some(1))
            .await;

        if let Err(e) = &result {
            println!("Error: {:?}", e);
        }
        assert!(
            result.is_ok(),
            "Get announcements failed: {:?}",
            result.err()
        );
        let response = result.unwrap();
        assert_eq!(response.ret_code, 0);
    }
}

#[cfg(test)]
mod broker_tests {
    use super::*;

    #[tokio::test]
    async fn test_broker_client_creation() {
        let client = create_test_client();
        let _broker_client = BrokerClient::new(client);
    }
}

#[cfg(test)]
mod crypto_loan_tests {
    use super::*;

    #[tokio::test]
    async fn test_crypto_loan_client_creation() {
        let client = create_test_client();
        let _crypto_loan_client = CryptoLoanClient::new(client);
    }
}

#[cfg(test)]
mod institutional_loan_tests {
    use super::*;

    #[tokio::test]
    async fn test_institutional_loan_client_creation() {
        let client = create_test_client();
        let _institutional_loan_client = InstitutionalLoanClient::new(client);
    }
}

#[cfg(test)]
mod pre_upgrade_tests {
    use super::*;

    #[tokio::test]
    async fn test_pre_upgrade_client_creation() {
        let client = create_test_client();
        let _pre_upgrade_client = PreUpgradeClient::new(client);
    }
}

#[cfg(test)]
mod spot_leverage_token_tests {
    use super::*;

    #[tokio::test]
    async fn test_spot_leverage_token_client_creation() {
        let client = create_test_client();
        let _spot_leverage_token_client = SpotLeverageTokenClient::new(client);
    }

    #[tokio::test]
    #[ignore] // Fails with EOF on Testnet
    async fn test_get_leverage_token_info() {
        let client = create_test_client();
        let spot_leverage_token_client = SpotLeverageTokenClient::new(client);

        let result = spot_leverage_token_client
            .get_leverage_token_info(None)
            .await;

        if let Err(e) = &result {
            println!("Error: {:?}", e);
        }
        assert!(
            result.is_ok(),
            "Get leverage token info failed: {:?}",
            result.err()
        );
        let response = result.unwrap();
        assert_eq!(response.ret_code, 0);
    }
}

#[cfg(test)]
mod spot_margin_trade_tests {
    use super::*;

    #[tokio::test]
    async fn test_spot_margin_trade_client_creation() {
        let client = create_test_client();
        let _spot_margin_trade_client = SpotMarginTradeClient::new(client);
    }
}
