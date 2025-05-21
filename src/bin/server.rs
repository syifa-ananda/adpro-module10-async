use futures_util::{sink::SinkExt, stream::StreamExt};
use std::{error::Error, net::SocketAddr};
use tokio::{
    net::{TcpListener, TcpStream},
    select,
    sync::broadcast::{channel, Sender},
};
use tokio_websockets::{Message, ServerBuilder, WebSocketStream};

const SERVER_NAME: &str = "Syifa's Computer";

async fn handle_connection(
    peer: SocketAddr,
    mut socket: WebSocketStream<TcpStream>,
    broadcaster: Sender<String>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let welcome = format!(
        "{} - From server: Welcome to chat! Type a message",
        SERVER_NAME
    );
    socket.send(Message::text(welcome)).await?;

    println!("▶️  Connected: {}", peer);
    let mut rx = broadcaster.subscribe();

    loop {
        select! {
            client_frame = socket.next() => {
                match client_frame {
                    Some(Ok(frame)) if frame.is_text() => {
                        let text = frame.as_text().unwrap();
                        println!("{} Received: {}", peer, text);

                        let msg = format!(
                            "{} - From server: {}: {}",
                            SERVER_NAME, peer, text
                        );
                        broadcaster.send(msg)?;
                    }
                    Some(Ok(_)) => {}
                    _ => {
                        println!("Disconnected: {}", peer);
                        break;
                    }
                }
            }

            broadcasted = rx.recv() => {
                if let Ok(msg) = broadcasted {
                    if socket.send(Message::text(msg)).await.is_err() {
                        println!("Failed to send to {}", peer);
                        break;
                    }
                } else {
                    break;
                }
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let (tx, _) = channel(32);
    let listener = TcpListener::bind(("127.0.0.1", 8080)).await?;
    println!("Server listening on port 8080");

    loop {
        let (stream, addr) = listener.accept().await?;
        let tx_cloned = tx.clone();

        tokio::spawn(async move {
            let (_req, ws) = ServerBuilder::new()
                .accept(stream)
                .await
                .expect("WebSocket handshake failed");
            if let Err(e) = handle_connection(addr, ws, tx_cloned).await {
                eprintln!("Error for {}: {}", addr, e);
            }
        });
    }
}
