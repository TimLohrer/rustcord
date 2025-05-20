use std::{sync::Arc, time::Duration};
use anyhow::Result;
use tauri::{AppHandle, Emitter, Manager};
use tokio::{time::sleep, sync::Mutex};
use websockets::WebSocket;

use crate::{gateway::gateway_messages::{HeartbeatMessage, IdentifyMessage, IdentifyProperties}, VERBOSE_GATEWAY};

use super::gateway_dtos::{GenericMessage, Message, OpCode, EventName};

pub(crate) struct GatewayConnection {
    pub token: String,
    pub version: String,
    pub intents: u64,
    pub is_ready: Arc<Mutex<bool>>,
    pub ws: Option<Arc<Mutex<WebSocket>>>
}

impl GatewayConnection {
    pub fn new(token: String, version: String, intents: u64) -> Self {
        Self {
            token,
            version,
            intents,
            is_ready: Arc::new(Mutex::new(false)),
            ws: None
        }
    }

    pub async fn connect_to_gateway(&mut self, app: AppHandle) -> Result<()> {
        let _ws = WebSocket::builder()
            .connect(format!("wss://gateway.discord.gg/?v={}&encoding=json", self.version).as_str())
            .await?;

        self.ws = Some(Arc::new(Mutex::new(_ws)));

        // Clone the necessary fields to move into the async block
        let ws_clone = self.ws.as_ref().unwrap().clone();
        let token = self.token.clone();
        let intents = self.intents;
        let is_ready = self.is_ready.clone();
        let app_handle = app.clone();

        tokio::spawn(async move {
            loop {
                let message = {
                    let mut recive_ws = ws_clone.lock().await;
                    let msg = recive_ws.receive().await.unwrap();
                    msg
                };

                let raw_data = match message.is_text() {
                    true => message.as_text().unwrap().0,
                    false => {
                        println!("Error converting message to text: {:?}", message);
                        continue;
                    }
                };
                let data: GenericMessage = match serde_json::from_str(&raw_data) {
                    Ok(data) => data,
                    Err(e) => {
                        println!("Error parsing JSON: {:?}", e);
                        continue;
                    }
                };

                let window = app_handle.get_webview_window("main").unwrap();
                window.emit("GATEWAY_EVENT", data.clone()).ok();

                if data.op_code == OpCode::Heartbeat {
                    println!("Heartbeat received!");
                } else if data.op_code == OpCode::Hello {
                    let heartbeat_interval = data.data.as_ref().unwrap()["heartbeat_interval"].as_u64().unwrap();
                    println!("Heartbeat interval: {} ms.", heartbeat_interval);

                    let heartbeat_message_json = HeartbeatMessage {}.to_json();
                    let ws_clone_inner = ws_clone.clone();
                    // Spawn a task to periodically send heartbeats
                    tokio::spawn(async move {
                        loop {
                            let send_result = {
                                let mut heartbeat_ws = ws_clone_inner.lock().await;
                                heartbeat_ws.send_text(heartbeat_message_json.clone()).await
                            };
                            if send_result.is_err() {
                                eprintln!("Failed to send heartbeat. Exiting...");
                                // Optionally handle closing or cleanup here if needed
                                break;
                            }
                            println!("Heartbeat sent.");
                            sleep(Duration::from_millis(heartbeat_interval)).await;
                        }
                    });
                } else if data.op_code == OpCode::HeartbeatAck {
                    println!("Received Heartbeat Acknowledgment");

                    let is_ready_guard = is_ready.lock().await;
                    if !*is_ready_guard {
                        let identify_message = IdentifyMessage {
                            token: token.clone(),
                            intents,
                            properties: IdentifyProperties {
                                os: "rustcord".to_string(),
                                browser: "rustcord".to_string(),
                                device: "rustcord".to_string(),
                            },
                        };
                        let mut send_ws = ws_clone.lock().await;
                        match send_ws.send_text(identify_message.to_json()).await {
                            Ok(_) => println!("Identify message sent."),
                            Err(e) => panic!("Error sending identify message: {:?}", e)
                        };
                    }
                } else if data.op_code == OpCode::Dispatch {
                    if VERBOSE_GATEWAY {
                        println!("Dispatch event received: {:?}", data.event_name.as_ref().unwrap());
                    }
                    let event_name = data.event_name.unwrap();
                    if event_name == EventName::Ready {
                        println!("Gateway connection is now ready!");
                        let mut is_ready_guard = is_ready.lock().await;
                        *is_ready_guard = true;
                    }
                } else {
                    if VERBOSE_GATEWAY {
                        println!("Other message received: {:?}", message.as_text().unwrap());
                    }
                }
                
                sleep(Duration::from_millis(10)).await;
            }
        });

        Ok(())
    }

    pub async fn send_message(&self, message: String) -> Result<(), String> {
        if let Some(ws) = &self.ws {
            let mut ws = ws.lock().await;
            let _ = ws.send_text(message).await.map_err(|e| e.to_string());
        } else {
            println!("Gateway is not connected.");
        }
        Ok(())
    }
}