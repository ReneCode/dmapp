//
// https://github.com/snapview/tokio-tungstenite/blob/master/examples/client.rs
//

use futures_util::{future, pin_mut, StreamExt};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

#[tokio::main]
async fn main() {
    let url = "ws://localhost:9002";

    // let url = env::args()
    //     .nth(1)
    //     .unwrap_or_else(|| panic!("this program requires at least one argument"));

    let (stdin_tx, stdin_rx) = futures_channel::mpsc::unbounded();
    tokio::spawn(read_stdin(stdin_tx));

    let (ws_stream, _) = connect_async(url.to_string())
        .await
        .expect("Failed to connect");
    println!("WebSocket handshake has been successfully completed");

    let (write, read) = ws_stream.split();

    let stdin_to_ws = stdin_rx.map(Ok).forward(write);
    let ws_to_stdout = {
        read.for_each(|message| async {
            match message {
                Ok(Message::Text(text)) => println!("Text message: {}", text),
                Ok(Message::Binary(_)) => println!("Binary message"),
                Ok(Message::Ping(_)) => println!("Ping message"),
                Ok(Message::Pong(_)) => println!("Pong message"),
                Ok(Message::Close(_)) => println!("Close message"),
                Ok(Message::Frame(_)) => println!("Frame message"),
                Err(e) => eprintln!("Error: {}", e),
            }
            // let data = message.unwrap().into_data();
            // tokio::io::stdout().write_all(&data).await.unwrap();
        })
    };

    pin_mut!(stdin_to_ws, ws_to_stdout);
    future::select(stdin_to_ws, ws_to_stdout).await;
}

async fn read_stdin(tx: futures_channel::mpsc::UnboundedSender<Message>) {
    let stdin = tokio::io::stdin();
    let reader = BufReader::new(stdin);
    let mut lines = reader.lines();

    while let Ok(Some(line)) = lines.next_line().await {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        tx.unbounded_send(Message::text(trimmed.to_string()))
            .unwrap();
    }

    // let mut buf = String::new();
    // loop {
    //     buf.clear();
    //     // let mut buf = vec![0; 1024];
    //     let n = match stdin.read_to_string(&mut buf).await {
    //         Err(_) | Ok(0) => break,
    //         Ok(n) => n,
    //     };
    //     // remove whitespaces
    //     let trimmed = buf.trim();
    //     tx.unbounded_send(Message::text(trimmed.to_string()))
    //         .unwrap();
    // }
}
