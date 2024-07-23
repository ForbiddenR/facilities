use std::{env, io::stdin};

use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use url::Url;

#[tokio::main]
async fn main() {
    let connect_addr = env::var("WEBSOCKET_ADDRESS").expect("Websocket address is needed");
    let url = Url::parse(&connect_addr).unwrap();

    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("Websocket handshake has been successfully completed");

    let (mut write, mut read) = ws_stream.split();

    tokio::spawn(async move {
        while let Some(message) = read.next().await {
            match message {
                Ok(msg) => {
                    if msg.is_text() || msg.is_binary() {
                        println!("Received: {}", msg.to_text().unwrap());
                    }
                },
                Err(e) => {
                    eprintln!("Error receiving message: {}", e);
                    break;
                },
            }
        }
    });

    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input == "exit" {
            break;
        }
        write.send(Message::Text(input.to_string())).await.unwrap();
    }

    println!("Existing...");
}
