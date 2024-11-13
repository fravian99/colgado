pub mod game_actor;
pub mod message_actor;

pub type WebSocket = tokio_tungstenite::WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>;
pub type WsSender = SplitSink<WebSocket, Message>;
pub type WsReceiver = SplitStream<WebSocket>;

pub use crate::models;
use futures_util::stream::{SplitSink, SplitStream};
use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream};
