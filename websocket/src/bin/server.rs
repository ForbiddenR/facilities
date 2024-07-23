use std::{env, net::SocketAddr, sync::Arc};

use futures_util::{SinkExt, StreamExt};
use tokio::{net::{TcpListener, TcpStream}, sync::broadcast};
use tokio_tungstenite::{accept_async, tungstenite::Message};

#[tokio::main]
async fn main() {
    let port = env::var("WEBSOCKET_PORT").unwrap_or("18888".into());
    let address = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&address).await.expect("Failed to bind");
    println!("Listen to {}", &address);
    let (tx, _rx) = broadcast::channel(100);
    let tx = Arc::new(tx);

    while let Ok((stream, addr)) = listener.accept().await {
        let tx = tx.clone();
        tokio::spawn(handle_connection(stream, addr, tx));
    }
}

async fn handle_connection(stream: TcpStream, addr: SocketAddr, tx: Arc<broadcast::Sender<String>>) {
    let ws_stream = accept_async(stream).await.expect("Failed to accept");
    println!("New Websocket connection: {}", addr);
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    let mut rx = tx.subscribe();

    loop {
        tokio::select! {
            msg = ws_receiver.next() => {
                match msg {
                    Some(Ok(msg)) => {
                        if msg.is_text() || msg.is_binary() {
                            let message = msg.to_text().unwrap().to_string();
                            println!("Received message {}", message);
                            tx.send(message).unwrap();
                        }
                    },
                    Some(Err(e)) => {
                        eprintln!("Websocket error: {}", e);
                        break;
                    },
                    None => break,
                }
            },
            Ok(msg) = rx.recv() => {
                ws_sender.send(Message::Text(msg)).await.unwrap();
            }
        }
    }
}