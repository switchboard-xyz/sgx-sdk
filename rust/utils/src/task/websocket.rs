use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::RwLock;
use tokio_tungstenite::WebSocketStream;
use url::Url;

use futures_util::{future, pin_mut, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

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
    pub auto_reconnect: bool,
    pub ws_stream: Arc<
        RwLock<Option<WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>>>,
    >,
}

impl WebSocket {
    pub fn new(
        url: &str,
        subscriptions: HashMap<String, ISubscriptionDefinition>,
        auto_reconnect: bool,
    ) -> Self {
        WebSocket {
            subscriptions,
            auto_reconnect,
            url: Url::parse(url).unwrap(),
            cache: Arc::new(Mutex::new(HashMap::new())),
            ws_stream: Arc::new(RwLock::new(None)),
        }
    }

    pub async fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let (stdin_tx, stdin_rx) = futures_channel::mpsc::unbounded();
        tokio::spawn(read_stdin(stdin_tx));

        let (ws_stream, _) = connect_async(self.url.clone())
            .await
            .expect("Failed to connect");

        println!("WebSocket handshake has been successfully completed");

        let (write, read) = ws_stream.split();

        let stdin_to_ws = stdin_rx.map(Ok).forward(write);
        let ws_to_stdout = {
            read.for_each(|message| async {
                let data = message.unwrap().into_data();
                tokio::io::stdout().write_all(&data).await.unwrap();
            })
        };

        pin_mut!(stdin_to_ws, ws_to_stdout);
        future::select(stdin_to_ws, ws_to_stdout).await;

        if let Some(read) = self.ws_stream.write().await.as_mut() {
            while let Some(message) = read.next().await {
                let message = match message {
                    Ok(msg) => msg,
                    Err(e) => {
                        eprintln!("WebSocket error: {:?}", e);
                        break;
                    }
                };

                if let Message::Text(text) = message {
                    if let Ok(value) = serde_json::from_str::<Value>(&text) {
                        for (id, definition) in &self.subscriptions {
                            let json_path =
                                json_parse_task(value.clone(), definition.filter.as_str());

                            let json_path = json_path.unwrap_or(Value::Null);
                            if json_path.is_null() {
                                return;
                            }

                            if json_path.is_array() && json_path.as_array().unwrap().len() == 0 {
                                return;
                            }

                            let mut cache = self.cache.lock().await;
                            cache.insert(
                                id.clone(),
                                CachedWebsocketMessage {
                                    timestamp: 0,
                                    data: value.clone(),
                                },
                            );

                            return;
                        }
                    }
                }
            }
        }
    }
}

// Our helper method which will read data from stdin and send it along the
// sender provided.
async fn read_stdin(tx: futures_channel::mpsc::UnboundedSender<Message>) {
    let mut stdin = tokio::io::stdin();
    loop {
        let mut buf = vec![0; 1024];
        let n = match stdin.read(&mut buf).await {
            Err(_) | Ok(0) => break,
            Ok(n) => n,
        };
        buf.truncate(n);
        tx.unbounded_send(Message::binary(buf)).unwrap();
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

        let value = cache.get(&id).cloned();
        if value.is_none() {
            return None;
        }

        Some(value.unwrap().data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Error;
    use futures_util::{SinkExt, StreamExt};
    use serde_json::Value;
    use std::str::FromStr;
    use tokio::net::TcpListener;
    use tokio_tungstenite::accept_async;

    const WS_SERVER_URL: &str = "ws://localhost:12345";

    const JSON_STRING: &str = r#"{"symbol":"SOLUSDT","price":"25.36000000"}"#;

    #[tokio::test]
    async fn test_websocket() {
        // Start a WebSocket server
        tokio::spawn(async move {
            let listener = TcpListener::bind("127.0.0.1:12345").await.unwrap();
            let (stream, _) = listener.accept().await.unwrap();
            let mut ws = accept_async(stream).await.unwrap();
            let mut is_subscribed = false;

            while let Some(Ok(message)) = ws.next().await {
                if message.is_text() {
                    let text = message.to_text().unwrap();
                    if let Ok(json_value) = serde_json::from_str::<Value>(&text) {
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
        let mut subscriptions = HashMap::new();

        let mut websocket = WebSocket::new(WS_SERVER_URL, subscriptions.clone(), false);

        websocket
            .add_subscription(ISubscriptionDefinition {
                subscription: sol_subscription.clone(),
                filter: "$.price".to_string(),
                max_age_seconds: 30,
            })
            .await;
        let start_websocket_result = websocket.start().await;

        assert!(start_websocket_result.is_ok());

        // Wait 1 seconds after sending the subscription
        tokio::time::sleep(Duration::from_secs(1)).await;

        // Read the cache and assert it is 8

        let cached_value = websocket.get_cached_value(sol_subscription).await.unwrap();

        assert_eq!(
            cached_value,
            serde_json::from_str::<Value>(JSON_STRING).unwrap()
        );
    }
}
