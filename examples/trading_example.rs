use bybit_rust_api::rest::enums::category::Category;
use bybit_rust_api::rest::enums::order_type::OrderType;
use bybit_rust_api::rest::enums::side::Side;
use bybit_rust_api::rest::enums::time_in_force::TimeInForce;
use bybit_rust_api::rest::order::dto::{CancelOrderRequest, PlaceOrderRequest};
use bybit_rust_api::rest::{ApiKeyPair, OrderClient, RestClient};
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Get API credentials from environment variables
    let api_key = env::var("BYBIT_API_KEY").expect("BYBIT_API_KEY not set");
    let api_secret = env::var("BYBIT_API_SECRET").expect("BYBIT_API_SECRET not set");

    // Create API key pair
    let api_key_pair = ApiKeyPair::new("trading".to_string(), api_key, api_secret);

    // Create REST client for testnet
    let rest_client = RestClient::new(api_key_pair, "https://api-testnet.bybit.com".to_string());

    // Create Order client
    let order_client = OrderClient::new(rest_client);

    // Place a limit order
    println!("Placing a limit order...");
    let place_order_request = PlaceOrderRequest {
        category: Category::Spot,
        symbol: "BTCUSDT".to_string(),
        side: Side::Buy,
        order_type: OrderType::Limit,
        qty: "0.001".to_string(),
        price: Some("40000".to_string()),
        time_in_force: Some(TimeInForce::GTC),
        order_link_id: Some(format!("rust_sdk_test_{}", chrono::Utc::now().timestamp())),
        is_leverage: None,
        trigger_price: None,
        trigger_direction: None,
        trigger_by: None,
        order_filter: None,
        order_iv: None,
        position_idx: None,
        take_profit: None,
        stop_loss: None,
        tp_trigger_by: None,
        sl_trigger_by: None,
        reduce_only: None,
        close_on_trigger: None,
        smp_type: None,
        mmp: None,
        tpsl_mode: None,
        tp_limit_price: None,
        sl_limit_price: None,
        tp_order_type: None,
        sl_order_type: None,
    };

    match order_client.place_order(place_order_request).await {
        Ok(response) => {
            println!("Order placed successfully!");
            println!("Order ID: {}", response.result.order_id);
            println!("Order Link ID: {}", response.result.order_link_id);

            // Get open orders
            println!("\nGetting open orders...");
            let open_orders = order_client
                .get_open_orders(
                    Category::Spot,
                    Some("BTCUSDT"),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(10),
                    None,
                )
                .await?;
            println!("Open orders count: {}", open_orders.result.list.len());

            // Cancel the order
            println!("\nCancelling the order...");
            let cancel_request = CancelOrderRequest {
                category: Category::Spot,
                symbol: "BTCUSDT".to_string(),
                order_id: Some(response.result.order_id.clone()),
                order_link_id: None,
                order_filter: None,
            };

            let cancel_response = order_client.cancel_order(cancel_request).await?;
            println!("Order cancelled successfully!");
            println!("Cancelled Order ID: {}", cancel_response.result.order_id);
        }
        Err(e) => {
            eprintln!("Failed to place order: {}", e);
        }
    }

    // Get order history
    println!("\nGetting order history...");
    let order_history = order_client
        .get_order_history(
            Category::Spot,
            Some("BTCUSDT"),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(10),
            None,
        )
        .await?;
    println!("Order history count: {}", order_history.result.list.len());

    Ok(())
}
