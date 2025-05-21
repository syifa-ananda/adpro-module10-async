use futures_util::{sink::SinkExt, stream::StreamExt};
use http::Uri;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    select,
};
use tokio_websockets::{ClientBuilder, Message};

#[tokio::main]
async fn main() -> Result<(), tokio_websockets::Error> {
    let (mut ws, _) = ClientBuilder::from_uri(Uri::from_static("ws://127.0.0.1:8080"))
        .connect()
        .await?;

    let stdin = BufReader::new(tokio::io::stdin()).lines();

    tokio::pin!(stdin); 

    loop {
        select! {
            maybe_line = stdin.next_line() => {
                match maybe_line {
                    Ok(Some(input)) => {
                        ws.send(Message::text(input)).await?;
                    }
                    _ => break, 
                }
            }

            maybe_msg = ws.next() => {
                match maybe_msg {
                    Some(Ok(msg)) if msg.is_text() => {
                        println!("{}", msg.as_text().unwrap());
                    }
                    Some(Ok(_)) => {} 
                    _ => break,       
                }
            }
        }
    }
    Ok(())
}
