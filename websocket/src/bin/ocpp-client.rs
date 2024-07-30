use std::{error::Error, time::Duration};

use futures_util::{SinkExt, StreamExt};
use rust_ocpp::v1_6::messages::boot_notification::{BootNotificationRequest, BootNotificationResponse};
use tokio::time;
use tokio_tungstenite::{connect_async, tungstenite::Message};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "ws://example.com/ocpp/OCIJFFJIDF";
    let (ws_stream, _) = connect_async(url).await?;
    let (mut write, mut read) = ws_stream.split();

    let boot_notification = BootNotificationRequest {
        charge_box_serial_number: None,
        charge_point_model: "".to_string(),
        charge_point_serial_number: None,
        charge_point_vendor: "".to_string(),
        firmware_version: None,
        iccid: None,
        imsi: None,
        meter_serial_number: None,
        meter_type: None,
    };

    let request = serde_json::to_string(&boot_notification)?;
    write.send(Message::Text(request)).await?;

    let mut interval = time::interval(Duration::from_secs(60));
    loop {
        tokio::select! {
            Some(msg) = read.next() => {
                match msg {
                    Ok(Message::Text(text)) => {
                        let _response: BootNotificationResponse =  serde_json::from_str(&text)?;
                    },
                    Ok(Message::Close(_)) => break,
                    Err(e) => eprintln!("Error: {}", e),
                    _ => {}
                }
            },
            _ = interval.tick() => {
                let ping = Message::Ping(vec![]);
                write.send(ping).await?;
                println!("Ping sent");
            }
        }
    }
    Ok(())
}
