//
// https://github.com/snapview/tokio-tungstenite/blob/master/examples/interval-server.rs
//

use futures_channel::mpsc::{unbounded, UnboundedSender};
use futures_util::{future, pin_mut, StreamExt, TryStreamExt};
use std::{
    collections::HashMap,
    env,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::Message;

type Tx = UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;

async fn handle_connection(peer_map: PeerMap, raw_stream: TcpStream, addr: SocketAddr) {
    println!("Incoming TCP connection from: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during WebSocket handshake occurred");

    // Insert the write part of this peer to the peer map.
    let (tx, rx) = unbounded();
    peer_map.lock().unwrap().insert(addr, tx);

    let (outgoing, incoming) = ws_stream.split();

    let broadcast_incoming = incoming.try_for_each(|msg| {
        match &msg {
            Message::Text(text) => {
                println!("Text message: {}", text);

                let peers = peer_map.lock().unwrap();

                // We want to broadcast the message to everyone except ourselves.
                let broadcast_recipients = peers
                    .iter()
                    .filter(|(peer_addr, _)| peer_addr != &&addr)
                    .map(|(_, ws_sink)| ws_sink);

                for recp in broadcast_recipients {
                    recp.unbounded_send(msg.clone()).unwrap();
                }
            }
            Message::Binary(_) => println!("Binary message"),
            Message::Ping(_) => println!("Ping message"),
            Message::Pong(_) => println!("Pong message"),
            Message::Close(_) => {
                println!("Close message {}", addr);
                // Remove the peer from the map when they close the connection
                peer_map.lock().unwrap().remove(&addr);
            }
            Message::Frame(_) => println!("Frame message"),
        };

        future::ok(())
    });

    let receive_from_others = rx.map(Ok).forward(outgoing);

    pin_mut!(broadcast_incoming, receive_from_others);
    future::select(broadcast_incoming, receive_from_others).await;

    println!("{} disconnected", &addr);
    peer_map.lock().unwrap().remove(&addr);
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:9002".to_string());

    let state = PeerMap::new(Mutex::new(HashMap::new()));

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", addr);

    while let Ok((stream, client_addr)) = listener.accept().await {
        tokio::spawn(handle_connection(state.clone(), stream, client_addr));
    }
}
