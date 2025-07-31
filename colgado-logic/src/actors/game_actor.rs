use crate::word::Game;
use std::cell::OnceCell;

use tokio::sync::{mpsc, oneshot};
use trequests::models::info;
use trequests::models::requests::send_msg_request::SendMsgRequest;

use super::models::game_view::GameView;
use super::models::messages::{CommandMessage, GeneralMessage, TwitchMessage};

pub struct TwitchGameActor {
    receiver: mpsc::UnboundedReceiver<GeneralMessage>,
    session_id: OnceCell<String>,
    game: Option<Game>,
    user_info: info::User,
    bot_info: info::Bot,
    command: String,
}

impl TwitchGameActor {
    pub fn new(
        receiver: mpsc::UnboundedReceiver<GeneralMessage>,
        user_info: info::User,
        bot_info: info::Bot,
        command: String,
    ) -> Self {
        let session_id = OnceCell::<String>::new();
        let command = command + " ";
        Self {
            receiver,
            session_id,
            game: None,
            user_info,
            bot_info,
            command,
        }
    }

    async fn handle(&mut self, message: GeneralMessage) -> bool {
        match message {
            GeneralMessage::CommandMessage(command) => self.handle_command_message(command),
            GeneralMessage::TwitchMessage(message) => self.handle_twitch_message(message).await,
            GeneralMessage::TwitchSendMessage(message) => {
                let user_id = &self.user_info.user_id;
                SendMsgRequest::new(user_id, user_id, &message)
                    .bot_name("Colgado")
                    .send(&self.bot_info)
                    .await
                    .expect("Error sending message: {message}");

                true
            }
        }
    }

    async fn handle_twitch_message(&mut self, message: TwitchMessage) -> bool {
        let user_id = &self.user_info.user_id;
        match message {
            TwitchMessage::WelcomeMessage { session_id } => {
                trequests::subscribe_to_wb(&self.bot_info, &session_id, user_id, user_id)
                    .await
                    .expect("Error suscribing to channel");
                self.session_id.set(session_id).expect("Error setting id");
            }
            TwitchMessage::PlayerMessage {
                message_text,
                message_id,
                ..
            } => {
                let word_chars = if let Some(game) = &self.game
                    && !game.is_completed()
                {
                    self.valid_player_message(&message_text)
                } else {
                    None
                };
                if let (Some(word_chars), Some(game)) = (word_chars, &mut self.game) {
                    let result = game.check_word_chars(&word_chars);
                    if let Err(err) = result {
                        let game_error = err.twitch_message_error();
                        SendMsgRequest::new(user_id, user_id, game_error)
                            .reply_to(&message_id)
                            .bot_name("Colgado")
                            .send(&self.bot_info)
                            .await
                            .expect("Error sending message to player");
                    }
                }
            }

            _ => {}
        }
        true
    }

    fn valid_player_message<'a>(&self, mut message_text: &'a str) -> Option<Vec<&'a str>> {
        if message_text.is_empty() {
            return None;
        }
        // command includes a space
        let command_included = message_text.len() > self.command.len()
            && message_text[..self.command.len()] == self.command;
        if command_included {
            message_text = &message_text[self.command.len()..];
        }
        let word_chars = Game::split_chars(message_text);
        if command_included || word_chars.len() == 1 {
            Some(word_chars)
        } else {
            None
        }
    }

    fn handle_command_message(&mut self, message: CommandMessage) -> bool {
        match message {
            CommandMessage::GetSessionId { sender } => {
                let id = self.session_id.get().cloned();
                let _ = sender.send(id);
            }
            CommandMessage::GetGameState { sender } => {
                let game_view: Option<GameView> = self.game.as_ref().map(GameView::from);
                let _ = sender.send(game_view);
            }
            CommandMessage::SetGameWord { word, sender } => {
                let game = Game::new(word);
                let word = game.get_actual_word();
                self.game = Some(game);
                let _ = sender.send(word);
            }
        }
        true
    }

    pub async fn run(mut self) {
        while let Some(message) = self.receiver.recv().await {
            let continue_loop = self.handle(message).await;
            if !continue_loop {
                break;
            }
        }
        println!("[actor TwitchGameActor]: Finished");
    }
}

#[derive(Debug, Clone)]
pub struct TwitchGameHandle {
    sender: mpsc::UnboundedSender<GeneralMessage>,
}

impl TwitchGameHandle {
    pub fn new_and_joinhandle(
        user_info: info::User,
        bot_info: info::Bot,
        command: String,
    ) -> (Self, tokio::task::JoinHandle<()>) {
        let (send, recv) = mpsc::unbounded_channel();
        let actor = TwitchGameActor::new(recv, user_info, bot_info, command);
        let task = tokio::spawn(actor.run());
        (Self { sender: send }, task)
    }

    async fn send_and_recv<MSType>(
        &self,
        message: CommandMessage,
        recv: oneshot::Receiver<MSType>,
    ) -> MSType {
        let message = GeneralMessage::CommandMessage(message);
        self.sender
            .send(message)
            .expect("Error: Twitch Actor killed");
        recv.await.expect("Error: Twitch Actor killed")
    }

    pub async fn get_id(&self) -> Option<String> {
        let (send, recv) = oneshot::channel::<Option<String>>();
        let message = CommandMessage::GetSessionId { sender: send };
        self.send_and_recv(message, recv).await
    }

    pub async fn get_game_state(&self) -> Option<GameView> {
        let (send, recv) = oneshot::channel::<Option<GameView>>();
        let message = CommandMessage::GetGameState { sender: send };
        self.send_and_recv(message, recv).await
    }

    pub async fn set_game_word(&self, word: String) -> String {
        let (send, recv) = oneshot::channel::<String>();
        let message = CommandMessage::SetGameWord { word, sender: send };
        self.send_and_recv(message, recv).await
    }

    pub fn non_sleeping_send(&self, message: GeneralMessage) {
        // since it is an unbound channel it can not sleep,
        // in case of bounded channels try_send is needed
        let _ = self.sender.send(message);
    }

    pub async fn send_message(&self, message: String) {
        let message = GeneralMessage::TwitchSendMessage(message);
        let _ = self.sender.send(message);
    }

    pub async fn send_messages<IntoStringIter>(&self, messages: IntoStringIter)
    where
        IntoStringIter: IntoIterator<Item = String>,
    {
        for message in messages {
            self.send_message(message).await;
        }
    }
}
