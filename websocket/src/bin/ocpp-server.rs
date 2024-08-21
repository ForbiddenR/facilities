use std::{collections::HashMap, sync::Arc};

use futures_util::{SinkExt, StreamExt};
use rust_ocpp::v1_6::messages::boot_notification::{
    BootNotificationRequest, BootNotificationResponse,
};
use serde_json::Value;
use tokio::sync::{mpsc, Mutex};
use warp::{filters::ws::Message, Filter};

#[path = "../model.rs"]
mod model;

type ChargerConnections = Arc<Mutex<HashMap<String, mpsc::UnboundedSender<String>>>>;

#[tokio::main]
async fn main() {
    let connections: ChargerConnections = Arc::new(Mutex::new(HashMap::new()));

    let ocpp_route = warp::path("ocpp")
        .and(warp::path::param())
        .and(warp::ws())
        .and(warp::any().map(move || connections.clone()))
        .map(
            |charger_id: String, ws: warp::ws::Ws, connections: ChargerConnections| {
                ws.on_upgrade(|socket| handle_ocpp_connection(socket, charger_id, connections))
            },
        );

    println!("OCPP server starting");
    warp::serve(ocpp_route).run(([0, 0, 0, 0], 12000)).await;
}

async fn handle_ocpp_connection(
    ws: warp::ws::WebSocket,
    charger_id: String,
    connections: ChargerConnections,
) {
    println!("New websocket connection from charger: {}", charger_id);
    let (tx, mut rx) = mpsc::unbounded_channel();
    connections.lock().await.insert(charger_id.clone(), tx);

    let (mut ws_tx, mut ws_rx) = ws.split();

    tokio::task::spawn(async move {
        while let Some(message) = rx.recv().await {
            ws_tx.send(warp::ws::Message::text(message)).await.unwrap();
        }
    });

    while let Some(result) = ws_rx.next().await {
        match result {
            Ok(msg) => {
                if let Ok(text) = msg.to_str() {
                    handle_ocpp_message(&charger_id, text).await;
                }
            }
            Err(e) => {
                eprintln!("Websocket error: {}", e);
                break;
            }
        }
    }
}

async fn handle_ocpp_message(charge_point_id: &str, message: &str) {
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
                            handle_boot_notification(charge_point_id, array[3].clone()).await
                        }
                        _ => println!("Unsupported action: {}", action),
                    };
                }
                _ => println!("Unsupported message type: {}", message_type_id),
            }
        }
    }
}

async fn handle_boot_notification(charge_point_id: &str, payload: Value) {
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
    send_message(charge_point_id, &message).await;
}

async fn send_message(charge_point_id: &str, message: &str) {
    // let mut cps = charge_points.lock().await;
    // if let Some(cp) = cps.get_mut(charge_point_id) {
    //     if let Err(e) = cp.sender.send(Message::Text(message.to_string())).await {
    //         eprintln!("Error sending message to {}: {}", charge_point_id, e);
    //     }
    //     println!("succeed sending message");
    // }
}

fn test() -> Option<String> {
    let result = check()?;
    Some(result)
}

fn check() -> Option<String> {
    Some(String::new())
}
