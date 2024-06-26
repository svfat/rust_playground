use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures::{SinkExt, StreamExt};
use url::Url;
use thiserror::Error;

#[derive(Error, Debug)]
enum WebSocketError {
    #[error("Failed to connect to WebSocket server: {0}")]
    ConnectionError(#[from] tokio_tungstenite::tungstenite::Error),
    #[error("URL parsing error: {0}")]
    UrlParseError(#[from] url::ParseError),
    #[error("Stream error: {0}")]
    StreamError(String),
}

const WS_URL: &str = "wss://echo.websocket.org";

type Result<T> = std::result::Result<T, WebSocketError>;

#[tokio::main]
async fn main() -> Result<()> {
    let url = Url::parse(WS_URL)?;
    let (ws_stream, _) = connect_async(url.as_str()).await?;
    let (mut write, mut read) = ws_stream.split();

    let send_msg = async {
        let msg = "Hello, WebSocket!";
        write.send(Message::Text(msg.to_string())).await.map_err(|e| WebSocketError::StreamError(e.to_string()))?;
        println!("Sent message: {:?}", msg);
        Ok::<(), WebSocketError>(())
    };

    let recv_msg = async {
        match read.next().await {
            Some(Ok(msg)) => {
                println!("Received message: {:?}", msg);
            }
            Some(Err(e)) => {
                return Err(WebSocketError::StreamError(e.to_string()));
            }
            None => {
                return Err(WebSocketError::StreamError("No message received".to_string()));
            }
        }
        Ok::<(), WebSocketError>(())
    };
    tokio::try_join!(send_msg, recv_msg)?;

    Ok::<(), WebSocketError>(())
    }