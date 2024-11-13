use super::{
    game_actor::TwitchGameHandle,
    models::messages::{GeneralMessage, TwitchMessage},
    WebSocket, WsReceiver,
};

use futures_util::StreamExt;
use tokio::sync::mpsc;
use tokio_tungstenite::tungstenite::Message;

struct TwitchMessageActor<Handle> {
    twitch_receiver: WsReceiver,
    handle: Handle,
    handle_receiver: mpsc::Receiver<GeneralMessage>,
}

impl TwitchMessageActor<TwitchGameHandle> {
    pub fn new(
        twitch_receiver: WsReceiver,
        handle: TwitchGameHandle,
        handle_receiver: mpsc::Receiver<GeneralMessage>,
    ) -> Self {
        Self {
            twitch_receiver,
            handle,
            handle_receiver,
        }
    }

    async fn handle_twitch_message(&mut self, message: Message) {
        let twitch_message = TwitchMessage::from(message);
        let general_message = match &twitch_message {
            TwitchMessage::None | TwitchMessage::Other { mesage: _ } => {
                return;
            }
            _ => GeneralMessage::TwitchMessage(twitch_message),
        };
        self.handle.non_sleeping_send(general_message);
    }

    pub async fn run(mut self) {
        loop {
            tokio::select! {
               Some(Ok(message)) = self.twitch_receiver.next() => {
                   self.handle_twitch_message(message).await;
               }
               Some(_) = self.handle_receiver.recv() => {

               }
               else => break,
            }
        }

        println!("[actor TwitchMessageActor]: Finished");
    }
}
#[derive(Clone, Debug)]
pub struct TwitchMessageHandle {
    _sender: mpsc::Sender<GeneralMessage>,
}

impl TwitchMessageHandle {
    pub fn new(
        ws_stream: WebSocket,
        handle: TwitchGameHandle,
    ) -> (Self, tokio::task::JoinHandle<()>) {
        let (send, recv) = mpsc::channel(100);
        let (_, ws_recv) = ws_stream.split();
        let actor = TwitchMessageActor::new(ws_recv, handle, recv);
        let task = tokio::spawn(actor.run());
        (Self { _sender: send }, task)
    }
}
