use serde_json::Value;
use tokio::sync::oneshot;
use tokio_tungstenite::tungstenite::Message;

use super::game_view::GameView;
#[derive(Debug)]
pub enum GeneralMessage {
    CommandMessage(CommandMessage),
    TwitchMessage(TwitchMessage),
    TwitchSendMessage(String),
}

/// Messages received from the app
#[derive(Debug)]
pub enum CommandMessage {
    GetSessionId {
        sender: oneshot::Sender<Option<String>>,
    },
    GetGameState {
        sender: oneshot::Sender<Option<GameView>>,
    },
    SetGameWord {
        word: String,
        sender: oneshot::Sender<String>,
    },
}
/// Mesages received from Twitch
#[derive(Debug)]
pub enum TwitchMessage {
    Message {
        message: Message,
    },
    PlayerMessage {
        message_text: String,
        player_id: String,
        player_name: String,
    },
    WelcomeMessage {
        session_id: String,
    },
    OtherText {
        text: String,
    },
    Other {
        mesage: Message,
    },
    None,
}

impl TwitchMessage {
    fn from_message_text(value: String) -> Self {
        let v: Value = serde_json::from_str(&value).expect("Error deserializing message");
        let message_type = &v["metadata"]["message_type"];
        let message_type = message_type.as_str();
        match message_type {
            Some("notification") => {
                let text = &v["payload"]["event"]["message"]["text"];
                let text = text.as_str().unwrap_or_default().to_owned();

                let player_id = &v["payload"]["event"]["chatter_user_id"];
                let player_id = player_id.as_str().unwrap_or_default().to_owned();

                let player = &v["payload"]["event"]["chatter_user_name"];
                let player = player.as_str().unwrap_or_default().to_owned();
                Self::PlayerMessage {
                    message_text: text,
                    player_id,
                    player_name: player,
                }
            }
            Some("session_welcome") => {
                let session_id = &v["payload"]["session"]["id"];
                let session_id = session_id.as_str().unwrap_or_default().to_owned();
                Self::WelcomeMessage { session_id }
            }
            Some("session_keepalive") => Self::None,
            Some(message_type) => {
                println!("[WARNING] Type not handled {}", message_type);
                Self::OtherText { text: value }
            }
            None => Self::None,
        }
    }
}

impl From<Message> for TwitchMessage {
    fn from(value: Message) -> Self {
        let message: Self = match value {
            Message::Text(msg_text) => Self::from_message_text(msg_text),
            _ => Self::Other { mesage: value },
        };
        message
    }
}
