use std::{collections::HashMap, sync::Arc};

use futures_util::{stream::SplitSink, SinkExt, StreamExt};
use rust_ocpp::v1_6::messages::boot_notification::{
    BootNotificationRequest, BootNotificationResponse,
};
use serde_json::Value;
use tokio::{
    net::{TcpListener, TcpStream},
    sync::Mutex,
};
use tokio_tungstenite::{accept_async, tungstenite::Message, WebSocketStream};

#[path = "../model.rs"]
mod model;

type ChargePoints = Arc<Mutex<HashMap<String, ChargePoint>>>;

struct ChargePoint {
    id: String,
    sender: SplitSink<WebSocketStream<TcpStream>, Message>,
}

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).await.expect("Failed to bind");
    println!("OCPP server listening on: {}", addr);

    let charge_points: ChargePoints = Arc::new(Mutex::new(HashMap::new()));

    while let Ok((stream, _)) = listener.accept().await {
        let charge_points = charge_points.clone();
        tokio::spawn(handle_connection(stream, charge_points));
    }
}

async fn handle_connection(stream: TcpStream, charge_points: ChargePoints) {
    let addr = stream
        .peer_addr()
        .expect("Connected streams should have a peer address");
    println!("New Websocket connection: {}", addr);

    let ws_stream = accept_async(stream)
        .await
        .expect("Failed to accept websocket");

    let (ws_sender, mut ws_receiver) = ws_stream.split();

    let charge_point_id = match ws_receiver.next().await {
        Some(Ok(msg)) => {
            if let Ok(text) = msg.to_text() {
                String::from(text.trim())
            } else {
                println!("Invalid charge point ID");
                return;
            }
        }
        _ => {
            println!("Failed to receive charge point ID");
            return;
        }
    };

    println!("Charge Point connected: {}", charge_point_id);

    {
        let mut cps = charge_points.lock().await;
        cps.insert(
            charge_point_id.clone(),
            ChargePoint {
                id: charge_point_id.clone(),
                sender: ws_sender,
            },
        );
    }

    while let Some(msg) = ws_receiver.next().await {
        match msg {
            Ok(msg) => {
                if msg.is_text() {
                    let text = msg.to_text().unwrap();
                    handle_ocpp_message(&charge_point_id, text, charge_points.clone()).await;
                }
            }
            Err(e) => {
                eprintln!("Websocket error: {}", e);
                break;
            }
        }
    }

    println!("Charge Point disconnected: {}", charge_point_id);
    let mut cps = charge_points.lock().await;
    cps.remove(&charge_point_id);
}

async fn handle_ocpp_message(charge_point_id: &str, message: &str, charge_points: ChargePoints) {
    let parsed: Value = serde_json::from_str(message).unwrap();
    if let Value::Array(array) = parsed {
        if array.len() >= 3 {
            let message_type_id = array[0].as_i64().unwrap();
            let _message_id = array[1].as_str().unwrap();
            let action = array[2].as_str().unwrap();

            match message_type_id {
                2 => {
                    match action {
                        "BootNotification" => {
                            handle_boot_notification(
                                charge_point_id,
                                array[3].clone(),
                                charge_points,
                            )
                            .await
                        }
                        _ => println!("Unsupported action: {}", action),
                    };
                }
                _ => println!("Unsupported message type: {}", message_type_id),
            }
        }
    }
}

async fn handle_boot_notification(
    charge_point_id: &str,
    payload: Value,
    charge_points: ChargePoints,
) {
    let request: BootNotificationRequest = serde_json::from_value(payload).unwrap();
    let mut interval = 30;
    if request.iccid.is_none() {
        interval = 20;
    }

    let message = serde_json::to_string(&BootNotificationResponse {
        current_time: chrono::Utc::now(),
        interval: interval,
        status: rust_ocpp::v1_6::types::RegistrationStatus::Accepted,
    })
    .unwrap();
    send_message(charge_point_id, &message, charge_points).await;
}

async fn send_message(charge_point_id: &str, message: &str, charge_points: ChargePoints) {
    let mut cps = charge_points.lock().await;
    if let Some(cp) = cps.get_mut(charge_point_id) {
        if let Err(e) = cp.sender.send(Message::Text(message.to_string())).await {
            eprintln!("Error sending message to {}: {}", charge_point_id, e);
        }
    }
}
