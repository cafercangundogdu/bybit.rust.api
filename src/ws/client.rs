//! WebSocket client for Bybit V5 with automatic reconnection and resubscription.
//!
//! # Features
//! - Connect to public or private WebSocket streams
//! - Subscribe/unsubscribe topics
//! - Automatic ping/pong heartbeat (20s)
//! - Automatic reconnection with exponential backoff
//! - Automatic re-subscription after reconnect
//! - Implements `futures::Stream` for async iteration
//!
//! # Example
//!
//! ```ignore
//! use bybit_rust_api::ws::{WsClient, topics};
//! use futures_util::StreamExt;
//!
//! let mut client = WsClient::connect("wss://stream.bybit.com/v5/public/linear").await?;
//! client.subscribe(vec![topics::orderbook(1, "BTCUSDT")]).await?;
//! while let Some(msg) = client.next().await {
//!     println!("{:?}", msg);
//! }
//! ```

use crate::rest::errors::{BybitError, BybitResult};
use crate::ws::messages::{WsMessage, WsRequest};
use futures_util::stream::SplitSink;
use futures_util::{SinkExt, Stream, StreamExt};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::sync::{mpsc, Mutex};
use tokio::time::{interval, sleep};
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};

type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

/// Maximum reconnection attempts before giving up.
const MAX_RECONNECT_ATTEMPTS: u32 = 10;
/// Base delay for exponential backoff (ms).
const RECONNECT_BASE_DELAY_MS: u64 = 500;
/// Maximum delay between reconnection attempts (ms).
const RECONNECT_MAX_DELAY_MS: u64 = 30_000;
/// Ping interval in seconds.
const PING_INTERVAL_SECS: u64 = 20;

/// Authentication parameters stored for reconnection.
#[derive(Clone)]
struct AuthParams {
    api_key: String,
    expires: u64,
    signature: String,
}

/// A WebSocket client for Bybit V5 streams with auto-reconnect.
pub struct WsClient {
    /// Channel to send commands to the connection task
    command_tx: mpsc::UnboundedSender<Command>,
    /// Receives parsed messages from the connection task
    message_rx: mpsc::UnboundedReceiver<WsMessage>,
    /// Handle to the connection task
    _handle: Option<tokio::task::JoinHandle<()>>,
    /// The WebSocket endpoint URL
    url: String,
    /// Currently subscribed topics (for resubscribe on reconnect)
    subscribed_topics: Arc<Mutex<Vec<String>>>,
}

enum Command {
    Subscribe(Vec<String>),
    Unsubscribe(Vec<String>),
    Authenticate {
        api_key: String,
        expires: u64,
        signature: String,
    },
}

impl WsClient {
    /// Connect to a Bybit WebSocket endpoint.
    ///
    /// Spawns a background task that manages the connection lifecycle
    /// including automatic reconnection with exponential backoff.
    pub async fn connect(url: &str) -> BybitResult<Self> {
        let (command_tx, command_rx) = mpsc::unbounded_channel();
        let (message_tx, message_rx) = mpsc::unbounded_channel();

        let subscribed_topics = Arc::new(Mutex::new(Vec::new()));
        let topics = subscribed_topics.clone();
        let url_owned = url.to_string();

        let handle = tokio::spawn(async move {
            run_connection_loop(&url_owned, command_rx, message_tx, topics).await;
        });

        Ok(WsClient {
            command_tx,
            message_rx,
            _handle: Some(handle),
            url: url.to_string(),
            subscribed_topics,
        })
    }

    /// Subscribe to one or more topics.
    ///
    /// Topics are remembered for automatic re-subscription on reconnect.
    pub async fn subscribe(&self, topics: Vec<String>) -> BybitResult<()> {
        // Store topics for reconnect
        {
            let mut stored = self.subscribed_topics.lock().await;
            for t in &topics {
                if !stored.contains(t) {
                    stored.push(t.clone());
                }
            }
        }

        self.command_tx
            .send(Command::Subscribe(topics))
            .map_err(|e| BybitError::Internal(format!("Subscribe channel closed: {}", e)))?;
        Ok(())
    }

    /// Unsubscribe from one or more topics.
    pub async fn unsubscribe(&self, topics: Vec<String>) -> BybitResult<()> {
        // Remove from stored topics
        {
            let mut stored = self.subscribed_topics.lock().await;
            stored.retain(|t| !topics.contains(t));
        }

        self.command_tx
            .send(Command::Unsubscribe(topics))
            .map_err(|e| BybitError::Internal(format!("Unsubscribe channel closed: {}", e)))?;
        Ok(())
    }

    /// Authenticate for private channels.
    ///
    /// Auth params are remembered for automatic re-authentication on reconnect.
    pub async fn authenticate(
        &self,
        api_key: &str,
        expires: u64,
        signature: &str,
    ) -> BybitResult<()> {
        self.command_tx
            .send(Command::Authenticate {
                api_key: api_key.to_string(),
                expires,
                signature: signature.to_string(),
            })
            .map_err(|e| BybitError::Internal(format!("Auth channel closed: {}", e)))?;
        Ok(())
    }

    /// Get the WebSocket endpoint URL.
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Close the WebSocket connection gracefully.
    ///
    /// After calling this, the stream will end and no more messages
    /// will be received. The background task is notified to shut down.
    pub fn close(&mut self) {
        // Dropping command_tx will cause the connection handler to exit
        self.command_tx = mpsc::unbounded_channel().0;
        self._handle = None;
    }
}

impl Drop for WsClient {
    fn drop(&mut self) {
        // Shutdown the background task by dropping the sender
        // The JoinHandle will be cancelled on drop
    }
}

impl Stream for WsClient {
    type Item = WsMessage;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.message_rx.poll_recv(cx)
    }
}

/// Connection loop: connects, handles messages, reconnects on failure.
async fn run_connection_loop(
    url: &str,
    mut command_rx: mpsc::UnboundedReceiver<Command>,
    message_tx: mpsc::UnboundedSender<WsMessage>,
    subscribed_topics: Arc<Mutex<Vec<String>>>,
) {
    let mut auth_params: Option<AuthParams> = None;
    let mut attempt = 0;

    loop {
        if attempt > 0 {
            let delay_ms =
                (RECONNECT_BASE_DELAY_MS * 2_u64.pow(attempt.min(6))).min(RECONNECT_MAX_DELAY_MS);
            log::warn!(
                "Reconnecting in {}ms (attempt {}/{})...",
                delay_ms,
                attempt,
                MAX_RECONNECT_ATTEMPTS
            );
            sleep(Duration::from_millis(delay_ms)).await;
        }

        if attempt >= MAX_RECONNECT_ATTEMPTS {
            log::error!("Max reconnect attempts reached. Giving up.");
            break;
        }

        match connect_async(url).await {
            Ok((ws_stream, _)) => {
                log::info!("WebSocket connected to {}", url);
                attempt = 0; // reset on successful connection

                let (ws_write, ws_read) = ws_stream.split();
                let ws_write = Arc::new(Mutex::new(ws_write));

                // Re-authenticate if we had auth before
                if let Some(ref auth) = auth_params {
                    let req = WsRequest::auth(&auth.api_key, auth.expires, &auth.signature);
                    send_command(&ws_write, &req).await;
                }

                // Re-subscribe all topics
                {
                    let topics = subscribed_topics.lock().await;
                    if !topics.is_empty() {
                        let req = WsRequest::subscribe(topics.clone());
                        send_command(&ws_write, &req).await;
                    }
                }

                // Run the connection until it fails
                run_connection(
                    ws_read,
                    ws_write,
                    &mut command_rx,
                    &message_tx,
                    &mut auth_params,
                )
                .await;
            }
            Err(e) => {
                log::error!("Connection failed: {}", e);
            }
        }

        attempt += 1;
    }
}

/// Send a WS request through the writer.
async fn send_command(writer: &Arc<Mutex<SplitSink<WsStream, Message>>>, req: &WsRequest) {
    if let Ok(json) = serde_json::to_string(req) {
        if let Ok(mut w) = writer.try_lock() {
            let _ = w.send(Message::Text(json.into())).await;
        }
    }
}

/// Core connection handler: reads WS messages, processes commands, sends pings.
async fn run_connection(
    mut ws_read: futures_util::stream::SplitStream<WsStream>,
    ws_write: Arc<Mutex<SplitSink<WsStream, Message>>>,
    command_rx: &mut mpsc::UnboundedReceiver<Command>,
    message_tx: &mpsc::UnboundedSender<WsMessage>,
    auth_params: &mut Option<AuthParams>,
) {
    let mut ping_interval = interval(Duration::from_secs(PING_INTERVAL_SECS));

    loop {
        tokio::select! {
            // Handle incoming commands
            cmd = command_rx.recv() => {
                match cmd {
                    Some(Command::Subscribe(topics)) => {
                        let req = WsRequest::subscribe(topics);
                        send_command(&ws_write, &req).await;
                    }
                    Some(Command::Unsubscribe(topics)) => {
                        let req = WsRequest::unsubscribe(topics);
                        send_command(&ws_write, &req).await;
                    }
                    Some(Command::Authenticate { api_key, expires, signature }) => {
                        *auth_params = Some(AuthParams {
                            api_key: api_key.clone(),
                            expires,
                            signature: signature.clone(),
                        });
                        let req = WsRequest::auth(&api_key, expires, &signature);
                        send_command(&ws_write, &req).await;
                    }
                    None => {
                        // Command channel closed
                        break;
                    }
                }
            }

            // Send ping every N seconds
            _ = ping_interval.tick() => {
                let ping = WsRequest::ping();
                send_command(&ws_write, &ping).await;
            }

            // Read incoming messages
            msg = ws_read.next() => {
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        match serde_json::from_str::<WsMessage>(&text) {
                            Ok(parsed) => {
                                if message_tx.send(parsed).is_err() {
                                    break; // receiver dropped
                                }
                            }
                            Err(e) => {
                                log::warn!("Failed to parse WS message: {} -- raw: {}", e, text);
                            }
                        }
                    }
                    Some(Ok(Message::Ping(data))) => {
                        if let Ok(mut writer) = ws_write.try_lock() {
                            let _ = writer.send(Message::Pong(data)).await;
                        }
                    }
                    Some(Ok(Message::Close(frame))) => {
                        log::info!(
                            "WebSocket closed by server: {:?}",
                            frame.map(|f| f.reason.to_string())
                        );
                        break;
                    }
                    Some(Err(e)) => {
                        log::error!("WebSocket error: {}", e);
                        break;
                    }
                    None => {
                        log::info!("WebSocket stream ended");
                        break;
                    }
                    _ => {} // ignore binary, pong, etc.
                }
            }
        }
    }

    log::info!("WebSocket connection handler exited");
}
