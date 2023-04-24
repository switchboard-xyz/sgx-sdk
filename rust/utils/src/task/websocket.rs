use futures_util::{SinkExt, StreamExt};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::sync::RwLock;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;
use url::Url;

use crate::json_parse_task;

pub type Filter = String;

#[derive(Clone, Debug)]
pub struct ISubscriptionDefinition {
    pub subscription: String,
    pub filter: String,
    pub max_age_seconds: u64,
}

#[derive(Clone, Debug)]
pub struct SubscriptionDefinition {
    pub id: String,
    pub definition: ISubscriptionDefinition,
}

#[derive(Clone, Debug)]
pub struct CachedWebsocketMessage {
    pub timestamp: i64,
    pub data: Value,
}

pub struct WebSocket {
    pub url: Url,
    pub subscriptions: HashMap<String, ISubscriptionDefinition>,
    pub cache: Arc<Mutex<HashMap<String, CachedWebsocketMessage>>>,
    pub ws_stream: Arc<
        RwLock<Option<WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>>>,
    >,
}

impl WebSocket {
    pub fn new(url: &str, subscriptions: HashMap<String, ISubscriptionDefinition>) -> Self {
        WebSocket {
            subscriptions,
            url: Url::parse(url).unwrap(),
            cache: Arc::new(Mutex::new(HashMap::new())),
            ws_stream: Arc::new(RwLock::new(None)),
        }
    }

    pub async fn start(&mut self) -> Result<(), Box<dyn std::error::Error, dyn s\ Send>> {
        loop {
            let (ws_stream, _) = match tokio_tungstenite::connect_async(&self.url).await {
                Ok(stream) => stream,
                Err(e) => {
                    tokio::time::sleep(Duration::from_secs(5)).await;
                    continue;
                }
            };

            // Store the WebSocketStream in the ws_stream field
            {
                let mut stream_write = self.ws_stream.write().await;
                *stream_write = Some(ws_stream);
            }

            // Handle the connection
            self.handle_connection().await;
        }
    }

    async fn reconnect(&self) {
        for sub_def in self.subscriptions.values() {
            let message = Message::Text(sub_def.subscription.clone());
            if let Some(ws_stream) = self.ws_stream.write().await.as_mut() {
                ws_stream
                    .send(message)
                    .await
                    .expect("Error sending subscription message");
            }
        }
    }

    async fn handle_connection(&self) {
        self.reconnect().await;

        if let Some(read) = self.ws_stream.write().await.as_mut() {
            while let Some(Ok(message)) = read.next().await {
                match message {
                    Message::Text(text) => {
                        if let Ok(value) = serde_json::from_str::<Value>(&text) {
                            for (id, definition) in &self.subscriptions {
                                let json_path =
                                    json_parse_task(value.clone(), definition.filter.as_str());

                                let json_path = json_path.unwrap_or(Value::Null);
                                if json_path.is_null()
                                    || (json_path.is_array()
                                        && json_path.as_array().unwrap().is_empty())
                                {
                                    continue;
                                }

                                let mut cache = self.cache.lock().await;
                                cache.insert(
                                    id.clone(),
                                    CachedWebsocketMessage {
                                        timestamp: 0,
                                        data: value.clone(),
                                    },
                                );

                                break;
                            }
                        }
                    }
                    Message::Binary(_binary) => {
                        eprintln!("WebSocket error: Binary message type not supported");
                        break;
                    }
                    Message::Frame(_binary) => {
                        eprintln!("WebSocket error: Frame message type not supported");
                        break;
                    }
                    Message::Ping(data) => {
                        if let Some(ws_stream) = self.ws_stream.write().await.as_mut() {
                            ws_stream
                                .send(Message::Pong(data))
                                .await
                                .unwrap_or_else(|e| {
                                    eprintln!("Error sending pong message: {:?}", e);
                                });
                        }
                    }
                    Message::Pong(_data) => {
                        eprintln!("WebSocket error: Pong message type not supported");
                        break;
                    }
                    Message::Close(_) => {
                        break;
                    }
                };
            }
        }
    }

    pub async fn close(&mut self) {
        if let Some(ws_stream) = self.ws_stream.write().await.as_mut() {
            ws_stream.close(None).await.unwrap_or_else(|e| {
                eprintln!("Error sending close message: {:?}", e);
            });
        }
    }

    async fn send_subscription(&self, sub_def: &ISubscriptionDefinition) {
        let message = Message::Text(sub_def.subscription.clone());

        if let Some(ws_stream) = self.ws_stream.write().await.as_mut() {
            ws_stream
                .send(message)
                .await
                .expect("Error sending subscription message");
        }
    }

    pub async fn add_subscription(&mut self, sub_def: ISubscriptionDefinition) {
        self.subscriptions
            .insert(sub_def.subscription.clone(), sub_def.clone());
        self.send_subscription(&sub_def).await;
    }

    pub async fn get_cached_value(&self, id: String) -> Option<Value> {
        let cache = self.cache.lock().await;

        let value = cache.get(&id).cloned()?;

        Some(value.data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures_util::{SinkExt, StreamExt};
    use serde_json::Value;
    use tokio::net::TcpListener;
    use tokio_tungstenite::accept_async;

    const WS_SERVER_URL: &str = "ws://localhost:12345";

    const JSON_STRING: &str = r#"{"symbol":"SOLUSDT","price":"25.36000000"}"#;

    #[tokio::test]
    async fn test_websocket() {
        // Start a WebSocket server
        let websocket_server = tokio::spawn(async move {
            let listener = TcpListener::bind("127.0.0.1:12345").await.unwrap();
            let (stream, _) = listener.accept().await.unwrap();
            let mut ws = accept_async(stream).await.unwrap();
            let mut is_subscribed = false;

            while let Some(Ok(message)) = ws.next().await {
                if message.is_text() {
                    let text = message.to_text().unwrap();
                    if let Ok(json_value) = serde_json::from_str::<Value>(text) {
                        // Check for the subscription JSON object
                        if json_value["symbol"].is_string() {
                            is_subscribed = true;
                            ws.send(tokio_tungstenite::tungstenite::Message::text("Subscribed"))
                                .await
                                .unwrap();
                        }
                    }
                }

                if is_subscribed {
                    // Send the static value 8 to the client every 100 ms

                    ws.send(tokio_tungstenite::tungstenite::Message::text(JSON_STRING))
                        .await
                        .unwrap();
                    tokio::time::sleep(Duration::from_millis(100)).await;
                }
            }
        });

        // Wait for the server to start
        tokio::time::sleep(Duration::from_secs(1)).await;

        let sol_subscription = r#"{"symbol": "SOLUSDT"}"#.to_string();
        let subscriptions = HashMap::new();

        let mut websocket = WebSocket::new(WS_SERVER_URL, subscriptions.clone());

        websocket
            .add_subscription(ISubscriptionDefinition {
                subscription: sol_subscription.clone(),
                filter: "$.price".to_string(),
                max_age_seconds: 30,
            })
            .await;
        let websocket_handle = tokio::spawn(async move { websocket.start().await });

        // assert!(websocket_handle.is_ok());

        // Wait 1 seconds after sending the subscription
        tokio::time::sleep(Duration::from_secs(1)).await;

        // Read the cache and assert it is 8

        let cached_value = websocket.get_cached_value(sol_subscription).await.unwrap();

        assert_eq!(
            cached_value,
            serde_json::from_str::<Value>(JSON_STRING).unwrap()
        );

        println!("closing websocket");

        websocket.close().await;
        let _ = websocket_handle.await; // Wait for the WebSocket to close
    }
}
