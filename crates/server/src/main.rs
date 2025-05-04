//
// https://github.com/snapview/tokio-tungstenite/blob/master/examples/interval-server.rs
//
// use futures_channel::mpsc::{unbounded, UnboundedSender};
use futures_util::{future, pin_mut, SinkExt, StreamExt};
use log::*;
use std::{
    collections::HashMap,
    env,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::mpsc::{unbounded_channel, UnboundedSender}, // sync::mpsc::unbounded_channel,
};
use tokio_tungstenite::tungstenite::Message;

type Tx = UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;

async fn handle_connection(peer_map: PeerMap, raw_stream: TcpStream, addr: SocketAddr) {
    println!("Incoming TCP connection from: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during WebSocket handshake occurred");

    // insert the write part of this peer to the peer map
    let (tx, rx) = unbounded_channel::<Message>();

    // tx.send(Message::text("Hello from server!")).await;

    peer_map.lock().unwrap().insert(addr, tx);

    let (mut outgoing, mut incoming) = ws_stream.split();

    outgoing
        .send(Message::text("Hello from server!"))
        .await
        .expect("Error sending message");

    loop {
        tokio::select! {
            msg = incoming.next() => {
                match msg {
                    Some(msg) => {
                        let msg = msg.expect("Error receiving message");
                        if msg.is_text() || msg.is_binary() {
                            println!("Received message: {:?}", msg);


                          for (peer_addr, peer_tx) in peer_map.lock().unwrap().iter() {
                            if *peer_addr != addr {
                              info!("Sending message to peer: {}", peer_addr);

                                // Send the message to all other peers
                                peer_tx.send(msg.clone()).expect("Error sending message");
                            }
                          }


                          // let peers = peer_map.lock().unwrap();
                      //     let broadcast_recipients =
                      //     peers.iter().filter(|(peer_addr, _)| peer_addr != &&addr).map(|(_, ws_sink)| ws_sink);

                      // for recp in broadcast_recipients {
                      //     recp.unbounded_send(msg.clone()).unwrap();
                      // }


                            // echo the message back to the sender
                            outgoing.send(msg).await.expect("Error sending message");
                        } else if msg.is_close() {
                            println!("Connection closed by client");
                            break;
                        }
                    }
                    None => break,
                }
            }
            // handle other tasks here, e.g., sending messages to peers
        }
    }

    println!("Connection closed: {}", addr);

    // if let Err(e) = handle_connection(peer, stream).await {
    //     match e {
    //         Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
    //         err => error!("Error processing connection: {}", err),
    //     }
    // }
}

/*
async fn org_handle_connection(peer_map: PeerMap, raw_stream: TcpStream, addr: SocketAddr) {
    println!("Incoming TCP connection from: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occurred");
    println!("WebSocket connection established: {}", addr);

    // Insert the write part of this peer to the peer map.
    let (tx, rx) = unbounded();
    peer_map.lock().unwrap().insert(addr, tx);

    let (outgoing, incoming) = ws_stream.split();

    let broadcast_incoming = incoming.try_for_each(|msg| {
        println!(
            "Received a message from {}: {}",
            addr,
            msg.to_text().unwrap()
        );
        let peers = peer_map.lock().unwrap();

        // We want to broadcast the message to everyone except ourselves.
        let broadcast_recipients = peers
            .iter()
            .filter(|(peer_addr, _)| peer_addr != &&addr)
            .map(|(_, ws_sink)| ws_sink);

        for recp in broadcast_recipients {
            recp.unbounded_send(msg.clone()).unwrap();
        }

        future::ok(())
    });

    let receive_from_others = rx.map(Ok).forward(outgoing);

    pin_mut!(broadcast_incoming, receive_from_others);
    future::select(broadcast_incoming, receive_from_others).await;

    println!("{} disconnected", &addr);
    peer_map.lock().unwrap().remove(&addr);
}
 */

/*
async fn _handle_connection(peer: SocketAddr, stream: TcpStream) -> Result<()> {
    let ws_stream = accept_async(stream).await.expect("Failed to accept");
    info!("New WebSocket connection: {}", peer);
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();
    let mut interval = tokio::time::interval(Duration::from_millis(1000));

    // Echo incoming WebSocket messages and send a message periodically every second.

    loop {
        tokio::select! {
            msg = ws_receiver.next() => {
                match msg {
                    Some(msg) => {
                        let msg = msg?;
                        if msg.is_text() ||msg.is_binary() {
                            ws_sender.send(msg).await?;
                        } else if msg.is_close() {
                            break;
                        }
                    }
                    None => break,
                }
            }
            _ = interval.tick() => {
              info!("Sending tick to {}", peer);
                ws_sender.send(Message::text("tick")).await?;
            }
        }
    }

    Ok(())
}
*/
#[tokio::main]
async fn main() {
    env_logger::init();

    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:9002".to_string());

    let state = PeerMap::new(Mutex::new(HashMap::new()));

    // let listener = TcpListener::bind(&addr).await.expect("Can't listen");
    // info!("Listening on: {}", addr);

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", addr);

    while let Ok((stream, client_addr)) = listener.accept().await {
        // let peer = stream
        //     .peer_addr()
        //     .expect("connected streams should have a peer address");
        // info!("Peer address: {}", peer);

        tokio::spawn(handle_connection(state.clone(), stream, client_addr));
        // tokio::spawn(org_handle_connection(state.clone(), stream, client_addr));
    }

    // Ok(())
}
