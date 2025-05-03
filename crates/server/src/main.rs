use futures::{SinkExt, StreamExt};
use log::{error, info};
use std::env;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};

//
// https://rishabh.io/building-a-rusty-websocket-server-4f3ba4b6b19c
//
#[tokio::main]
async fn main() {
    println!("Starting WebSocket server...");

    env_logger::init();

    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    let addr: SocketAddr = addr.parse().expect("Invalid address");
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");

    info!("Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_connection(stream));
    }
}

async fn handle_connection(stream: TcpStream) {
    let ws_stream = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            error!("Error during WebSocket handshake: {}", e);
            return;
        }
    };
    info!("New WebSocket connection established.");

    let (mut sender, mut receiver) = ws_stream.split();

    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                info!("Received message: {}", text);
                // Echo the message back to the client
                if let Err(e) = sender.send(Message::Text(text)).await {
                    error!("Error sending message: {}", e);
                    break;
                }
            }
            Ok(_) => (),
            Err(e) => {
                error!("Error receiving message: {}", e);
                break;
            }
        }
    }
}
