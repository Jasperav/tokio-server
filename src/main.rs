use tokio::net::{TcpListener, TcpStream};
use std::net::SocketAddr;
use std::sync::Arc;
use futures::{StreamExt, SinkExt};
use tungstenite::Message;
use tokio_tungstenite::WebSocketStream;

struct DatabaseConnection;

#[tokio::main]
async fn main() -> Result<(), ()> {
    listen("127.0.0.1:3012", Arc::new(DatabaseConnection)).await
}

async fn listen(address: &str, db: Arc<DatabaseConnection>) -> Result<(), ()> {
    let try_socket = TcpListener::bind(address).await;
    let mut listener = try_socket.expect("Failed to bind on address");

    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(stream, addr, db.clone()));
    }

    Ok(())
}

async fn handle_connection(raw_stream: TcpStream, addr: SocketAddr, db: Arc<DatabaseConnection>) {
    let db = &*db;
    let ws_stream = tokio_tungstenite::accept_async(raw_stream).await.unwrap();

    let (mut outgoing, incoming) = ws_stream.split();

    incoming.for_each(|m| async {
        match m {
            Ok(message) => do_something(message, db, &mut outgoing).await,
            Err(e) => panic!(e)
        }
    }).await;
}

async fn do_something(message: Message, db: &DatabaseConnection, outgoing: &mut futures_util::stream::SplitSink<WebSocketStream<TcpStream>, Message>) {
    // Do something...

    // Send some message
    let _ = outgoing.send(Message::Text("yay".to_string())).await;
}